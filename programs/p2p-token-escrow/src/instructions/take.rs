use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token, token_interface::{self, *}};

use crate::{Escrow, EscrowError, ESCROW_SEED};

pub fn take_make(ctx: Context<TakeMake>, seed: u64, maker_offer: u64, maker_ask: u64) -> Result<()> {

    // Front running attack prevention. Could also use slip.
    require_eq!(ctx.accounts.escrow.maker_ask, maker_ask, EscrowError::MakeAndTakeAmountMissmatch);
    require_eq!(ctx.accounts.escrow.maker_offer, maker_offer, EscrowError::MakeAndTakeAmountMissmatch);

    msg!("Transfer {} from taker to maker (Legacy)", maker_ask);
    let cpi = token_interface::TransferChecked {
        from: ctx.accounts.taker_ata_from.to_account_info(),
        mint: ctx.accounts.mint_taker.to_account_info(),
        to: ctx.accounts.maker_ata_to.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };
    let ctx_transfer = CpiContext::new(
        ctx.accounts.mint_taker_token_program.to_account_info(), 
        cpi);
    token_interface::transfer_checked(
        ctx_transfer, 
        maker_ask, 
        ctx.accounts.mint_taker.decimals)?;

    msg!("Transfer {} from maker to taker (in vault: {})", maker_offer, ctx.accounts.escrow_vault.amount);
    // Transfer from escrow vault to taker
    let decimals_maker = ctx.accounts.mint_maker.decimals;
    let cpi_evtt = token_interface::TransferChecked {
        from: ctx.accounts.escrow_vault.to_account_info(),
        mint: ctx.accounts.mint_maker.to_account_info(),
        to: ctx.accounts.taker_ata_to.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(), 
    };
    let signers_seeds: &[&[&[u8]]] = &[&[
        ESCROW_SEED,
        &ctx.accounts.maker.key().to_bytes(),
        &seed.to_le_bytes(),
        &[ctx.bumps.escrow]
    ]];
    let ctx_evtt = CpiContext::new(
        ctx.accounts.mint_maker_token_program.to_account_info(), 
        cpi_evtt)
        .with_signer(signers_seeds);
    token_interface::transfer_checked(
        ctx_evtt, 
        maker_offer,
        decimals_maker)?;

    // Close escrow vault
    let cpi_close_vault = token_interface::CloseAccount {
        account: ctx.accounts.escrow_vault.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    let signers_seeds: &[&[&[u8]]] = &[&[
        ESCROW_SEED,
        &ctx.accounts.maker.key().to_bytes(),
        &seed.to_le_bytes(),
        &[ctx.bumps.escrow]
    ]];
    let ctx_close_vault = CpiContext::new(
        ctx.accounts.mint_maker_token_program.to_account_info(), 
        cpi_close_vault).with_signer(signers_seeds);
    token_interface::close_account(ctx_close_vault)?;

    Ok(())
}


#[derive(Accounts)]
#[instruction(seed: u64, taker_give: u64)]
pub struct TakeMake<'info> {
    #[account(mut)]
    taker: Signer<'info>,

    /// CHECK: By PDA of escrow
    #[account(mut)]
    maker: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = mint_taker,
        associated_token::authority = taker,
        associated_token::token_program = mint_taker_token_program,
    )]
    taker_ata_from: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_maker,
        associated_token::authority = taker,
        associated_token::token_program = mint_maker_token_program,
    )]
    taker_ata_to: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_taker,
        associated_token::authority = maker,
        associated_token::token_program = mint_taker_token_program,
    )]
    maker_ata_to: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = maker,
        has_one = mint_maker,
        has_one = mint_taker_token_program,
        seeds = [ESCROW_SEED, maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        close = maker,
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
