use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{self, *}};

use crate::{Escrow, EscrowError, ESCROW_SEED};

pub fn take_make(ctx: Context<TakeMake>, seed: u64, maker_offer: u64, maker_ask: u64) -> Result<()> {

    // Front running attack prevention. Could also use slip.
    require_eq!(ctx.accounts.escrow.maker_ask, maker_ask, EscrowError::MakeAndTakeAmountMissmatch);
    require_eq!(ctx.accounts.escrow.maker_offer, maker_offer, EscrowError::MakeAndTakeAmountMissmatch);

    // Transfer from taker to maker
    let cpi_ttm = token_interface::TransferChecked {
        from: ctx.accounts.taker_ata.to_account_info(),
        mint: ctx.accounts.mint_taker.to_account_info(),
        to: ctx.accounts.maker_ata.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };
    let ctx_ttm = CpiContext::new(
        ctx.accounts.mint_taker_token_program.to_account_info(), 
        cpi_ttm);
    token_interface::transfer_checked(ctx_ttm, maker_ask, ctx.accounts.mint_taker.decimals)?;

    // Transfer from maker to taker

    // Close escrow

    // Close escrow vault

    Ok(())
}


#[derive(Accounts)]
#[instruction(seed: u64, taker_give: u64)]
pub struct TakeMake<'info> {
    #[account(mut)]
    taker: Signer<'info>,

    /// CHECK: By PDA of escrow
    #[account()]
    maker: AccountInfo<'info>,
    
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_taker,
        associated_token::authority = taker,
        associated_token::token_program = mint_taker_token_program,
    )]
    taker_ata: InterfaceAccount<'info, TokenAccount>,

    // taker receiving ata
    // address the maker deposits to the taker

    #[account(
        mut,
        associated_token::mint = mint_maker,
        associated_token::authority = maker,
        associated_token::token_program = mint_maker_token_program,
    )]
    maker_ata: InterfaceAccount<'info, TokenAccount>,
    
    // maker receiving ata
    // Address the taker deposits to the maker

    #[account(
        mut,
        has_one = maker,
        has_one = mint_maker,
        has_one = mint_taker_token_program,
        seeds = [ESCROW_SEED, maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    escrow: Account<'info, Escrow>,

    #[account(
        mut,
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
