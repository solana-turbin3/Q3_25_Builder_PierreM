use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi,
            FreezeDelegatedAccountCpiAccounts
        },
        MasterEditionAccount,
        Metadata,
        MetadataAccount,
    }, 
    token::{
        approve,
        Approve, 
        Mint, 
        Token, 
        TokenAccount
    },
};

use crate::state::{StakeAccount, StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    //NFT mint account
    pub mint: Account<'info, Mint>,

    pub collection_mint: Account<'info, Mint>,

    //ATA holding the nft
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    //Metadata account for the NFT, checks collection and verification
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref(), b"edition"],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    //stake account 
    #[account(
        init,
        payer = user,
        seeds = [b"stake", mint.key().as_ref(), config.key().as_ref()],
        bump,
        space = StakeAccount::INIT_SPACE + 8,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    //user account
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        //Ensure max stake not reached
        assert!(self.user_account.amount_staked < self.config.max_stake);

        //Set fields of stake account
        self.stake_account.set_inner(StakeAccount { 
            owner: self.user.key(), 
            mint: self.mint.key(), 
            staked_at: Clock::get()?.unix_timestamp, 
            bump: bumps.stake_account
        });

        //Delegate NFT to stake account
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        approve(cpi_ctx, 1)?;

        //signer seeds
        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];
        let signer_seeds = &[&seeds[..]];
        
        //freeze token account
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        FreezeDelegatedAccountCpi::new(
            metadata_program, 
            FreezeDelegatedAccountCpiAccounts { 
                delegate, 
                token_account, 
                edition, 
                mint, 
                token_program 
            }
        ).invoke_signed(signer_seeds)?;
        
        self.user_account.amount_staked += 1;

        Ok(())
    }
}