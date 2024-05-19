use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_wallet_account: Pubkey,
    pub taker_key: Pubkey,
    pub price_per_day: u64,
    pub deposit_amount: u64,
    pub rental_days: u64,
    pub rent_start_time: i64,
    pub is_accepted: bool,
}