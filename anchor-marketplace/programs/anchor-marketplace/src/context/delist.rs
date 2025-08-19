use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{close_account, CloseAccount}, 
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    /// Maker (owner) who is delisting the NFT
    #[account(mut)]
    pub maker: Signer<'info>,

    /// Marketplace PDA
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// Mint of the NFT being delisted
    pub maker_mint: InterfaceAccount<'info, Mint>,

    /// Maker's associated token account for the NFT
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    /// Listing PDA, closed after delisting
    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    /// Vault holding the NFT during listing
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Delist<'info> {

    /// Closes the vault account and returns rent to the maker
    pub fn close_mint_vault(&mut self) -> Result<()> {
        let seeds = &[
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(ctx)
    }

    /// Transfers the NFT from the vault back to the maker and closes the listing
    pub fn delist(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info()
        };
        let seeds = &[
            self.marketplace.to_account_info().key.as_ref(), 
            self.maker_mint.to_account_info().key.as_ref(), 
            &[self.listing.bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Transfer 1 NFT back to maker, using mint decimals for precision
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
    }
}