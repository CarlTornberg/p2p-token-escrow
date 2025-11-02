use anchor_lang::prelude::*;

declare_id!("ChDn1PMciMVbLKoQXe2qHo6HtLCikFsDaKC7fJgfaPNc");

mod states;
pub use states::*;
mod errors;
pub use errors::*;

mod instructions;
pub use instructions::*;

#[program]
pub mod p2p_token_escrow {
    use super::*;

    pub fn make(ctx: Context<CreateMake>, seed: u64, maker_offer: u64, maker_ask: u64) -> Result<()> {
        instructions::create_make(ctx, seed, maker_offer, maker_ask)
    }

    pub fn refund(ctx: Context<RefundMake>, seed: u64) -> Result<()> {
        instructions::refund_make(ctx, seed)
    }

    pub fn take(ctx: Context<TakeMake>, seed: u64, maker_offer: u64, maker_ask: u64) -> Result<()> {
        instructions::take_make(ctx, seed, maker_offer, maker_ask)
    }
}
