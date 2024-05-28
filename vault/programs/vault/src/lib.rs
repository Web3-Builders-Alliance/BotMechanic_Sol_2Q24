use anchor_lang::prelude::*;

declare_id!("2NTSzo14WWinXikBQJFr3nJZs7W98j2NtCrHA4CAYucN");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {}
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user,
        seeds 
    )]


impl<'info> Intialize<'info> {
    pub fn initialize(&mut self, amount: u64, bumps: &InitializeBumps) -> Result<()> {
        
    }
}

#[account]
pub struct VaultState {
    pub amount: u64,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultSpace {
    const INIT_SPACE: usize = 8 + 8 + 1 + 1;
}
