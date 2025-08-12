use anchor_lang::prelude::*;

declare_id!("5u5iCtN9n9vWborv8LsEcByGNg33wBZjRjzfpJyXY8JX");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
