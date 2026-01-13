// AURA Protocol - Core Smart Contract Prototype (Solana/Anchor)
use anchor_lang::prelude::*;

declare_id!("Aura111111111111111111111111111111111111111");

#[program]
pub mod aura_protocol {
    use super::*;
    pub fn initialize_pool(ctx: Context<InitializePool>, duration: i64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.end_time = Clock::get()?.unix_timestamp + duration;
        Ok(())
    }
}

#[account]
pub struct Pool { pub last_bidder: Pubkey, pub end_time: i64, pub total_yield: u64 }
#[derive(Accounts)]
pub struct InitializePool<'info> { 
    #[account(init, payer = user, space = 8 + 32 + 8 + 8)] pub pool: Account<'info, Pool>,
    #[account(mut)] pub user: Signer<'info>, pub system_program: Program<'info, System> 
}
