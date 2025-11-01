use anchor_lang::prelude::*;

pub const ESCROW_SEED: &[u8] = b"escrow";
pub const ESCROW_VAULT_SEED: &[u8] = b"escrow_vault";

#[account]
#[derive(InitSpace)]
pub struct Escrow {

    pub maker: Pubkey,
    pub mint_maker: Pubkey,
    pub mint_taker: Pubkey,

    pub mint_maker_token_program: Pubkey,
    pub mint_taker_token_program: Pubkey,

    pub maker_offer: u64,
    pub maker_ask: u64,
    pub bump: u8,
}

