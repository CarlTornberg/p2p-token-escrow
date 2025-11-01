use anchor_lang::prelude::*; 

#[error_code]
pub enum EscrowError {
    #[msg("Requested by maker and supplied by taker not matching")]
    MakeAndTakeAmountMissmatch
}
