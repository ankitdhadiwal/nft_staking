use anchor_lang::prelude::*;

declare_id!("5WcTh23GQdwG5MuWJFMUE7QfwQ51kx33eQkSuDd3FTE8");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
