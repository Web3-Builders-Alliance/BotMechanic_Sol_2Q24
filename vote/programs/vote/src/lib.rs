use anchor_lang::prelude::*;

declare_id!("6rLssdoxfvgTDKLPWPj37ZHGAVrKjceg91ugQKnjNgPu");

#[program]
pub mod vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _url: String) -> Result<()> {

        ctx.accounts.initialize(&ctx.bumps)?;
        // ctx.accounts.vote.score = 0;
        // ctx.accounts.vote.bump = ctx.bumps.vote;

        Ok(())
    }

    pub fn upvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.upvote()?;

        Ok(())
    }

    pub fn downvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.downvote()?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init, 
        payer = signer,
        space = VoteState::INIT_SPACE,
        seeds = [_url.as_bytes().as_ref()],
        bump,
    )]
    pub vote_account: Account<'info, VoteState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Vote<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [_url.as_bytes().as_ref()],
        bump = vote_account.bump,
    )]
    pub vote_account: Account<'info, VoteState>,

}

impl<'info> Vote<'info> {
    pub fn upvote(&mut self) -> Result<()> {
        self.vote_account.score += 1;

        Ok(())
    }

    pub fn downvote(&mut self) -> Result<()> {
        self.vote_account.score -= 1;

        Ok(())
    }
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vote_account.score = 0;
        self.vote_account.bump = bumps.vote_account;

        Ok(())
    }
}

#[account]
pub struct VoteState {
    score: i64,
    bump: u8,
}

impl Space for VoteState {
    const INIT_SPACE: usize = 8 + 8  + 1;
}


