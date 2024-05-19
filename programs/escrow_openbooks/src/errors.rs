use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Rental period is not over yet.")]    
    RentalPeriodNotOver,                        
    #[msg("Insufficient funds to pay for rent and deposit.")]
    InsufficientFunds,
}