use anchor_lang::prelude::*;

declare_id!("4EFSQt73xMA7Mtaw8tsMd27kVyJH9KiBzEeYQhFrtT1E");

#[program]
pub mod escrow_openbooks {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
