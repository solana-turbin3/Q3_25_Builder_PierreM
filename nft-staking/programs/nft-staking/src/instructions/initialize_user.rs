use anchor_lang::prelude::*;

use crate::state::UserAccount;


#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"user", user.key().as_ref()],
        bump,
        space = UserAccount::INIT_SPACE + 8,
    )]
    pub user_account: Account<'info, UserAccount>,

    system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
    pub fn initialize_user(&mut self, bumps: &InitializeUserBumps) -> Result<()> {

        self.user_account.set_inner(UserAccount {
            points: 0,
            amount_staked: 0,
            bump: bumps.user_account
        });

        Ok(())
    }
}