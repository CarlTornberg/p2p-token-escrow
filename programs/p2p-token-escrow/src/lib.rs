use anchor_lang::prelude::*;

declare_id!("BNVdmmxsCQiyPkTEFGg7SxkuRWZ2MiMyYamQnYu2wiPN");

mod states;
pub use states::*;

mod instructions;
pub use instructions::*;

#[program]
pub mod p2p_token_escrow {
    use super::*;

    pub fn make(ctx: Context<CreateMake>, seed: u64, give: u64, receive: u64) -> Result<()> {
        instructions::create_make(ctx, seed, give, receive)
    }

    pub fn refund(ctx: Context<RefundMake>, seed: u64) -> Result<()> {
        instructions::refund_make(ctx, seed)
    }
}
