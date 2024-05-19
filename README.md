# OpenBooks Escrow Smart Contract Documentation

## Overview

This smart contract is specifically designed for renting books with customizable pricing based on the number of days. This document serves as comprehensive documentation for the implemented smart contract.

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

### 1. Customizable Pricing

#### Implementation Strategy:

Define contract storage variable for `price_per_day`.

Provide setter and getter functions for adjusting the price, leveraging Rust's strong type system to ensure type safety.

Users can input a desired number of days, and the contract will dynamically calculate the total price (`total_price = price_per_day * number_of_days`).

### 2. Variable Rental Duration

#### Implementation Strategy:

Implement a struct to store rental details, including the duration in days and the datetime when the rental started.

Create functions to start and end rentals, using the blockchain timestamp to manage and verify the rental duration.

Allow modifications to the rental period, adjusting costs dynamically if changed before the rental starts.

### 3. Escrow Mechanism

#### Implementation Strategy:

Ensure that when a rental agreement is initiated, the funds (total rent price) from the renter are moved to an escrow account within the contract.

Hold funds in the escrow until the end of the rental period. Use smart contract conditions to ensure automatic release only if the rental term expires without disputes.

Include functions for conflict resolution where either party can raise disputes, to be handled according to predefined rules.

### 4. Security

#### Implementation Strategy:

Utilize Rust’s features for memory safety and strong typing to minimize common vulnerabilities associated with smart contracts.

Implement ownership and permission checks to restrict sensitive actions to authorized users only.

Conduct thorough testing (unit and integration tests) using tools and frameworks compatible with Rust. Consider fuzzing for unexpected inputs handling.

Peer reviews and possibly a third-party audit to further ensure the contract's security.

### 5. Documentation

#### Implementation Strategy:

Write comprehensive and clear documentation using markdown for readability. This will cover:

- Deployment procedures.
- Method descriptions with parameters, expected outcomes, and potential errors.
- Examples of typical interactions with the contract.
- Maintenance guidelines.

## Deliverables

- **Smart Contract Code**: Written in Rust, well-commented, and adhering to best coding practices.
- **Testing Scripts**: For automated testing of the contract functionalities.
- **API Documentation and Code**: Using OpenAPI specifications for clarity and standardization.
- **User and Developer Documentation**: In markdown format, hosted on platforms like GitHub or ReadTheDocs.

## Created by: Ritik Bhatt

---

This README provides an overview of the OpenBooks Escrow Smart Contract and its implementation. For further assistance or inquiries, please refer to the provided contacts.

*Contact:*
- [Telegram](https://t.me/+m5kYre9QPGsyMTdl)
---
