use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::*};

use crate::{Escrow, ESCROW_SEED};

pub fn create_make(ctx: Context<CreateMake>, seed: u64, give: u64, receive: u64) -> Result<()> {

    ctx.accounts.escrow.set_inner(Escrow { 
            maker: ctx.accounts.maker.key(), 
            mint_a: ctx.accounts.mint_a.key(), 
            mint_b: ctx.accounts.mint_b.key(), 
            mint_a_token_program: ctx.accounts.token_program.key(), 
            mint_b_token_program: ctx.accounts.token_program.key(), // TODO Add ability to set mint
            // B token program.
            give, 
            receive, 
            bump: ctx.bumps.escrow,
        });
    let escrow = &ctx.accounts.escrow;

    msg!("Created escrow {} trading {} of mint {} for {} of mint {}.",
        escrow.key(),
        escrow.receive,
        escrow.mint_b.key(),
        escrow.give,
        escrow.mint_a.key(),
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
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_a: InterfaceAccount<'info, TokenAccount>,

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
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    escrow_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        token::token_program = token_program,
    )]
    mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        token::token_program = token_program,
    )]
    mint_b: InterfaceAccount<'info, Mint>,

    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>
}
