use anchor_lang::prelude::*;

declare_id!("EeJNHvtc3jxUAyEPbcz3BP1t3jou7qvHPhoJ8PpsVxfk");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
