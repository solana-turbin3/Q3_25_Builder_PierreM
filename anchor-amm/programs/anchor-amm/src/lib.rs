use anchor_lang::prelude::*;

declare_id!("6V4YW5Y3d7FxebPj4r5DVJG5DCxz9UrzyqPmko1H8aZh");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
