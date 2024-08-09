use anchor_lang::prelude::*;

declare_id!("MWkrWhe9RCcnnCVM9RR7TtppWf2Mf7tnYgnwvG54CL7");

/// Turbin3 Vote program
/// 
#[program]
pub mod turbin3_vote {
    use super::*;

    /// Starting instruction to initialize the PDA account using the URL + ProgramId
    pub fn initialize(ctx: Context<Initialize>, _url: String) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    /// Instruction to increase in 1 the vote state of the PDA for a specific url
    pub fn upvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.upvote()?;

        Ok(())
    }

    /// Instruction to decrease in 1 the vote state of the PDA for a specific url
    pub fn downvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.downvote()?;

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Initialize<'info> {
    /// USer who will pay initializing the PDA
    #[account(mut)]
    pub payer: Signer<'info>,

    /// PDA generated with url + programId
    #[account(
        init,
        payer = payer,
        seeds = [_url.as_bytes().as_ref()],
        bump,
        space = VoteState::INIT_SPACE, 
    )]
    pub vote_account: Account<'info, VoteState>,

    /// The system program as we need to create an account
    pub system_program: Program<'info, System>,
}

impl<'info>  Initialize<'info> {
    /// The initialize insstruction will set the score to 0 and save the bump as a good practice
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vote_account.score = 0;
        self.vote_account.bump = bumps.vote_account;
        
        Ok(())
    }
}

/// VoteState will save the score  and the bump 
#[account]
pub struct VoteState {
    pub score: u64,
    pub bump: u8,
}

impl  Space for VoteState {
    /// We manually define the space required for deserializing this data saved in the account
    const INIT_SPACE: usize = 8 + 8 + 1;
    
}

/// The vote context will be used for upvote and downvote instructions
/// Note we use the instruction macro to expose url to the account parameters (seed)
#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Vote<'info> {
    /// We will need the right seeds to write in the correct PDA 
    #[account(
        mut,
        seeds = [_url.as_bytes().as_ref()],
        bump = vote_account.bump
    )]
    pub vote_account: Account<'info, VoteState>,
}

impl<'info> Vote<'info> {
    /// Instruction to upvote a url increasing in 1 the score
    pub fn upvote(&mut self) -> Result<()> {
        self.vote_account.score += 1;

        Ok(())
    }
    
    /// Instruction to downvote a url decreasing in 1 the score
    pub fn downvote(&mut self) -> Result<()> {
        self.vote_account.score -= 1;

        Ok(())
    }
}