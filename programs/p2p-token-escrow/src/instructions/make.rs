use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::*};

use crate::{Escrow, ESCROW_SEED};

pub fn create_make(ctx: Context<CreateMake>, seed: u64, maker_offer: u64, maker_ask: u64) -> Result<()> {

    ctx.accounts.escrow.set_inner(Escrow { 
            maker: ctx.accounts.maker.key(), 
            mint_maker: ctx.accounts.mint_maker.key(), 
            mint_taker: ctx.accounts.mint_taker.key(), 
            mint_maker_token_program: ctx.accounts.mint_maker_token_program.key(), 
            mint_taker_token_program: ctx.accounts.mint_taker_token_program.key(),
            maker_offer, 
            maker_ask, 
            bump: ctx.bumps.escrow,
        });
    let escrow = &ctx.accounts.escrow;

    msg!("Created escrow {} trading {} of mint {} for {} of mint {}.",
        escrow.key(),
        escrow.maker_ask,
        escrow.mint_taker.key(),
        escrow.maker_offer,
        escrow.mint_maker.key(),
    ); 

    Ok(())
}

#[derive(Accounts)]
#[instruction(seed: u64, amount: u64)]
pub struct CreateMake<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_maker,
        associated_token::authority = maker,
        associated_token::token_program = mint_maker_token_program,
    )]
    maker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [ESCROW_SEED, maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_maker,
        associated_token::authority = escrow,
        associated_token::token_program = mint_maker_token_program,
    )]
    escrow_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        token::token_program = mint_maker_token_program,
    )]
    mint_maker: InterfaceAccount<'info, Mint>,

    #[account(
        token::token_program = mint_taker_token_program,
    )]
    mint_taker: InterfaceAccount<'info, Mint>,

    system_program: Program<'info, System>,
    mint_maker_token_program: Interface<'info, TokenInterface>,
    mint_taker_token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>
}
