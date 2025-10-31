use anchor_lang::prelude::*;

declare_id!("BNVdmmxsCQiyPkTEFGg7SxkuRWZ2MiMyYamQnYu2wiPN");

mod states;
pub use states::*;

mod instructions;
pub use instructions::*;

#[program]
pub mod p2p_token_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
