use anchor_lang::prelude::*;

pub const ESCROW_SEED: &[u8] = b"escrow";
pub const ESCROW_VAULT_SEED: &[u8] = b"escrow_vault";

#[account]
#[derive(InitSpace)]
pub struct Escrow {

    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,

    pub mint_a_token_program: Pubkey,
    pub mint_b_token_program: Pubkey,

    pub give: u64,
    pub receive: u64,
    pub bump: u8,
}

