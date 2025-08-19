use anchor_lang::prelude::*;

declare_id!("7g6dfEQbuNNszUdtDB9ye2XcsNhTfvAwBpdsaYJQjrFL");

#[program]
pub mod anchor_dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
