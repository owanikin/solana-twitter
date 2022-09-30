use anchor_lang::prelude::*;

declare_id!("EMdQpPZ7s3c49wBeqvxpCJBuUwbfSQ47HJefERMmj74E");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
