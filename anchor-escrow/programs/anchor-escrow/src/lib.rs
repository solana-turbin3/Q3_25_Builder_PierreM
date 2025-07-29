#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

declare_id!("HYZJANuj2BnzrvSigdZGvGNmEPgj7MrbSaFWHj3dU7CJ");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}


