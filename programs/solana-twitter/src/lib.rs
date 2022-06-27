use anchor_lang::prelude::*;

declare_id!("BZvPErdwUEKjFk76hT2Z4FGqAwmf8k2cVRpL9wTHswVN");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
