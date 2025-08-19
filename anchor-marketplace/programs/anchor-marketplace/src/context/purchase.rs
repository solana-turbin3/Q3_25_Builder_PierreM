use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, mint_to, transfer_checked, CloseAccount, MintTo, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::{Listing, Marketplace};


#[derive(Accounts)]
pub struct Purchase<'info>{
    /// Buyer purchasing the NFT
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// Seller receiving payment
    #[account(mut)]
    pub seller: SystemAccount<'info>,

    /// Marketplace PDA
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// Mint of the NFT being purchased
    pub maker_mint: InterfaceAccount<'info, Mint>,

    /// Buyer's associated token account for the NFT
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = seller,
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,

    /// Vault holding the NFT during listing
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// Listing PDA
    #[account(
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    /// Reward token mint for the marketplace
    #[account(
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

impl <'info> Purchase<'info> {
    /// Sends SOL from buyer to seller for the purchase
    pub fn send_sol(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.seller.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, self.listing.price)
    }

    /// Transfers the NFT from the vault to the buyer
    pub fn receive_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.buyer_ata.to_account_info(), // Corrected: send to buyer's ATA
            authority: self.marketplace.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
    }

    /// Mints reward tokens to the buyer
    pub fn receive_rewards(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.buyer_ata.to_account_info(), // Corrected: mint to buyer's ATA
            authority: self.marketplace.to_account_info(),
        };

        let seeds = &[
            b"marketplace", 
            self.marketplace.name.as_str().as_bytes(),
            &[self.marketplace.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, 1)
    }

    /// Closes the vault account and returns rent to the seller
    pub fn close_mint_vault(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.seller.to_account_info(),
            authority: self.marketplace.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        close_account(cpi_ctx)
    }
}