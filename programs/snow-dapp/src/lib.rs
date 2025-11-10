use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("snowDApp111111111111111111111111111");

#[program]
pub mod snow_dapp {
    use super::*;

    pub fn initialize_staking(
        ctx: Context<InitializeStaking>,
        reward_rate: u64,
        lock_period: i64,
    ) -> Result<()> {
        let staking_pool = &mut ctx.accounts.staking_pool;
        staking_pool.reward_rate = reward_rate;
        staking_pool.lock_period = lock_period;
        staking_pool.total_staked = 0;
        staking_pool.bump = ctx.bumps.staking_pool;
        Ok(())
    }

    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
        let staking_pool = &mut ctx.accounts.staking_pool;
        let user_stake = &mut ctx.accounts.user_stake;
        
        user_stake.amount += amount;
        user_stake.stake_time = Clock::get()?.unix_timestamp;
        staking_pool.total_staked += amount;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.staking_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount)?;
        
        Ok(())
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, title: String, description: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.title = title;
        proposal.description = description;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.created_at = Clock::get()?.unix_timestamp;
        proposal.creator = ctx.accounts.creator.key();
        Ok(())
    }
}

#[account]
pub struct StakingPool {
    pub reward_rate: u64,
    pub lock_period: i64,
    pub total_staked: u64,
    pub bump: u8,
}

#[account]
pub struct UserStake {
    pub amount: u64,
    pub stake_time: i64,
    pub user: Pubkey,
}

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub created_at: i64,
    pub creator: Pubkey,
}

#[derive(Accounts)]
pub struct InitializeStaking<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 8 + 8 + 1, seeds = [b"staking-pool".as_ref()], bump)]
    pub staking_pool: Account<'info, StakingPool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut, seeds = [b"staking-pool".as_ref()], bump = staking_pool.bump)]
    pub staking_pool: Account<'info, StakingPool>,
    #[account(init_if_needed, payer = user, space = 8 + 8 + 8 + 32, seeds = [b"user-stake".as_ref(), user.key.as_ref()], bump)]
    pub user_stake: Account<'info, UserStake>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = creator, space = 8 + 256 + 512 + 8 + 8 + 8 + 32)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
