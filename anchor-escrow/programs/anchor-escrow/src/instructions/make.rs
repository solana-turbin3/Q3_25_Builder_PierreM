use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
    associated_token::AssociatedToken,
};

#[derive(Accounts)]
pub struct Make<'info>{
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        seeds=[b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8 + Escrow::INIT_SPACE,
        bump,
    )]

    pub escrow: Account<'info, Escrow>,
    
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]

    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
 pub fn init_escrow(&MakeBumps)
}


pub mod make;

pub use make::*;