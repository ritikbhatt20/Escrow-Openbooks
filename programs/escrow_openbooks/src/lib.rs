use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::transfer, clock::Clock};
use anchor_spl::token::{self, SetAuthority, Token, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("8yvPtTFYnTzw5GGBaJ6UgFrURzYg1CeFtKgiuA33cAG2");

#[program]
pub mod book_rental {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        price_per_day: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts.escrow_account.initializer_deposit_token_account = *ctx.accounts.initializer_deposit_token_account.to_account_info().key;
        ctx.accounts.escrow_account.initializer_receive_wallet_account = *ctx.accounts.initializer_receive_wallet_account.to_account_info().key;
        ctx.accounts.escrow_account.price_per_day = price_per_day;
        ctx.accounts.escrow_account.deposit_amount = deposit_amount;
        ctx.accounts.escrow_account.is_accepted = false;

        let (pda, _bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        token::set_authority(ctx.accounts.into(), AuthorityType::AccountOwner, Some(pda))?;
        Ok(())
    }

    pub fn request_rent(
        ctx: Context<RequestRent>,
        rental_days: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_account.taker_key = *ctx.accounts.taker.key;
        ctx.accounts.escrow_account.rental_days = rental_days;
        ctx.accounts.escrow_account.rent_start_time = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn accept_rent(ctx: Context<AcceptRent>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let total_amount = escrow_account.price_per_day * escrow_account.rental_days + escrow_account.deposit_amount;

        require!(
            **ctx.accounts.taker.lamports.borrow() >= total_amount,
            ErrorCode::InsufficientFunds
        );

        escrow_account.is_accepted = true;

        // Transfer total amount from taker to PDA
        invoke(
            &transfer(
                ctx.accounts.taker.key,
                ctx.accounts.pda_account.key,
                total_amount,
            ),
            &[
                ctx.accounts.taker.to_account_info(),
                ctx.accounts.pda_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn return_book(ctx: Context<ReturnBook>) -> Result<()> {
        let escrow_account = &ctx.accounts.escrow_account;
        let current_time = Clock::get()?.unix_timestamp;
        let rental_end_time = escrow_account.rent_start_time + (escrow_account.rental_days as i64) * 86400;

        require!(
            current_time >= rental_end_time,
            ErrorCode::RentalPeriodNotOver
        );

        // Transfer the NFT back to the initializer
        let (_pda, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], &[bump_seed]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_initializer_context()
                .with_signer(&[&seeds[..]]),
            1, // Transfer 1 token (NFT)
        )?;

        // Transfer rent and deposit from PDA to initializer
        let total_amount = escrow_account.price_per_day * escrow_account.rental_days + escrow_account.deposit_amount;

        invoke(
            &transfer(
                ctx.accounts.pda_account.key,
                ctx.accounts.initializer_receive_wallet_account.key,
                total_amount,
            ),
            &[
                ctx.accounts.pda_account.to_account_info(),
                ctx.accounts.initializer_receive_wallet_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        mut,
        constraint = initializer_deposit_token_account.amount == 1 // Ensure it's an NFT
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_wallet_account: AccountInfo<'info>,
    #[account(init, payer = initializer, space = 8 + EscrowAccount::LEN)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RequestRent<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
}

#[derive(Accounts)]
pub struct AcceptRent<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub pda_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReturnBook<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub initializer_receive_wallet_account: AccountInfo<'info>,
    #[account(mut)]
    pub pda_deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub pda_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
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

impl EscrowAccount {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 1;
}

impl<'info> From<&mut InitializeEscrow<'info>>
    for CpiContext<'_, '_, '_, 'info, SetAuthority<'info>>
{
    fn from(accounts: &mut InitializeEscrow<'info>) -> Self {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts
                .initializer_deposit_token_account
                .to_account_info()
                .clone(),
            current_authority: accounts.initializer.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> ReturnBook<'info> {
    fn into_transfer_to_initializer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pda_deposit_token_account.to_account_info().clone(),
            to: self.initializer_receive_wallet_account.to_account_info().clone(),
            authority: self.pda_account.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Rental period is not over yet.")]
    RentalPeriodNotOver,                        
    #[msg("Insufficient funds to pay for rent and deposit.")]
    InsufficientFunds,
}
