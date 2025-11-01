use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{self, *}};

use crate::{Escrow, ESCROW_SEED};

pub fn refund_make(ctx: Context<RefundMake>, seed: u64) -> Result<()> {

    let escrow_vault = &ctx.accounts.escrow_vault;
    let signers_seeds: &[&[&[u8]]] = &[&
        [
            ESCROW_SEED, 
            &ctx.accounts.maker.key().to_bytes(),
            &seed.to_le_bytes(),
            &[ctx.bumps.escrow]
        ]];

    // Extract all tokens from the vault.
    let cpi_accounts = token_interface::TransferChecked {
        from: ctx.accounts.escrow_vault.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
        to: ctx.accounts.maker_ata_a.to_account_info(),
        mint: ctx.accounts.mint_a.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts).with_signer(signers_seeds);

    token_interface::transfer_checked(
        cpi_context, 
        escrow_vault.amount, 
        ctx.accounts.mint_a.decimals
    )?;
        
    // Close ATA
    let close_accounts = token_interface::CloseAccount {
        account: ctx.accounts.escrow_vault.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };

    let close_cpi = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), close_accounts)
        .with_signer(signers_seeds);

    token_interface::close_account(close_cpi)?;

    Ok(())
}


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct RefundMake<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [ESCROW_SEED, maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    escrow: Account<'info, Escrow>,

    #[account(
        mut,
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
