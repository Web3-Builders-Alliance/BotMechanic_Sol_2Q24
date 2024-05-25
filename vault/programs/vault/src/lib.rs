use anchor_lang::prelude::*;

declare_id!("EMPw3kGpubiAsWGaL3G5gysG11zzQcUu9XARqjB3hZ3J");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
