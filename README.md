# OpenBooks Escrow Smart Contract Documentation

## Overview

OpenBooks offers a bounty for the development of an escrow smart contract in Rust. This smart contract is specifically designed for renting books with customizable pricing based on the number of days. This document serves as comprehensive documentation for the implemented smart contract.

## Smart Contract Details

### Functions

#### `initialize_escrow`

```rust
pub fn initialize_escrow(
    ctx: Context<InitializeEscrow>,
    price_per_day: u64,
    deposit_amount: u64,
) -> Result<()>
```

- **Description**: Initializes the escrow account with the specified price per day and deposit amount.
- **Parameters**:
  - `price_per_day`: The price per day for renting the book.
  - `deposit_amount`: The deposit amount required for the rental.
- **Return**: `Result<()>`

#### `request_rent`

```rust
pub fn request_rent(
    ctx: Context<RequestRent>,
    rental_days: u64,
) -> Result<()>
```

- **Description**: Initiates a request to rent a book for the specified number of days.
- **Parameters**:
  - `rental_days`: The number of days for renting the book.
- **Return**: `Result<()>`

#### `accept_rent`

```rust
pub fn accept_rent(ctx: Context<AcceptRent>) -> Result<()>
```

- **Description**: Accepts the rental request and transfers the necessary funds to the escrow account.
- **Parameters**: None
- **Return**: `Result<()>`

#### `return_book`

```rust
pub fn return_book(ctx: Context<ReturnBook>) -> Result<()>
```

- **Description**: Processes the return of the rented book and transfers the deposit and rental fee accordingly.
- **Parameters**: None
- **Return**: `Result<()>`

### Data Structures

#### `EscrowAccount`

```rust
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
```

- **Description**: Represents the escrow account storing information about the rental transaction.

### Errors

#### `ErrorCode`

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Rental period is not over yet.")]
    RentalPeriodNotOver,
    #[msg("Insufficient funds to pay for rent and deposit.")]
    InsufficientFunds,
}
```

- **Description**: Defines custom error codes for the smart contract.

## Implementation Details

The smart contract is implemented using the Anchor framework in Rust. It consists of several functions responsible for initializing the escrow, requesting and accepting rentals, and processing returns. The `EscrowAccount` struct stores relevant data about the rental transaction.

Each function is designed to ensure security, flexibility, and reliability in handling book rentals. Error handling is implemented using custom error codes to provide meaningful feedback to users.

For detailed usage instructions and integration with the OpenBooks platform, refer to the comprehensive documentation provided alongside the smart contract.

---

This README provides an overview of the OpenBooks Escrow Smart Contract and its implementation. For further assistance or inquiries, please refer to the provided contacts.

*Contact:*
- [Telegram](https://t.me/+m5kYre9QPGsyMTdl)
