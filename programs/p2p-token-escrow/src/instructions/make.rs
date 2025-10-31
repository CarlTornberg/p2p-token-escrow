use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;


#[derive(Accounts)]
pub struct Make<'info> {
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
}
