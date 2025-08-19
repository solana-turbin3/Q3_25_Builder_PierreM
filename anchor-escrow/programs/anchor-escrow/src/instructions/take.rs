use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{
        close_account,
        transfer_checked, 
        Mint, 
        TokenAccount, 
        TokenInterface, 
        TransferChecked, 
        CloseAccount
    } };

//use crate::state::Escrow;
use crate::Escrow;

#[derive(Accounts)]
//#[instruction(seed: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker : Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        mint::token_program = token_program,

    )]
    pub mint_a: InterfaceAccount<'info, Mint>, //derive ata

    #[account(
        mint::token_program = token_program,

    )]
    pub mint_b: InterfaceAccount<'info, Mint>, //derive ata

    #[account(
        init_if_needed,
        payer = taker, 
        associated_token::mint = mint_b, 
        associated_token::authority = maker, 
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>, //derive b ata

    #[account(
        init_if_needed,
        payer = taker, 
        associated_token::mint = mint_a, 
        associated_token::authority = taker, 
        associated_token::token_program = token_program,
            )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>, //transfer tokens into taker ata_a acct from vault

    #[account(
        mut, 
        associated_token::mint = mint_b, 
        associated_token::authority = taker, 
        associated_token::token_program = token_program,
            )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>, //transfer b tokens from taker to maker

    #[account(
        mut,
        has_one = mint_a,
        has_one = mint_b, 
        has_one = maker,
        seeds = [b"escrow", escrow.maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()], 
        bump = escrow.bump, 
        close = maker
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a, 
        associated_token::authority = escrow, 
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> Take<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        let transfer_acccounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(), 
            to: self.maker_ata_b.to_account_info(), 
            authority: self.taker.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_acccounts);

        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)?;

        
Ok(())
    }

    pub fn withdraw_and_close(&mut self) -> Result<()> {
        msg!("withdraw and close");
        let signer_seeds: [&[&[u8]]; 1]    = [&[
            b"escrow", 
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump]
        ]];
        msg!("signer seeds");

        let accounts = TransferChecked{
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info()
        };
        msg!("accounts");

        let cpi_ctx = CpiContext::new_with_signer(
             self.token_program.to_account_info(), 
             accounts, 
             &signer_seeds);
             
        msg!("cpictx");
        msg!("Attempting to transfer {} tokens from vault.", self.vault.amount);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;
        msg!("transferchecked");

        let accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        msg!("closeacct");

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), 
        accounts, 
        &signer_seeds);
        msg!("cpictx");

        close_account(ctx)

    }

  
}