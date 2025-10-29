use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::*;

declare_id!("ZTnFhxro4BiVWvAhH6m11SJx4BUDieP2Vu4yYymco1u");

#[program]
pub mod emojimarket_program {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        admin_address: Pubkey,
        platform_fee_bps: u16,
        creator_fee_bps: u16,
        base_price_lamports: u64,
        malus_k_millis: u32,
        quad_a_micros: u64,
        quad_b_micros: u64,
        min_duration_secs: u32,
        max_duration_secs: u32,
    ) -> Result<()> {
        instructions::initialize_config::handler(
            ctx,
            admin_address,
            platform_fee_bps,
            creator_fee_bps,
            base_price_lamports,
            malus_k_millis,
            quad_a_micros,
            quad_b_micros,
            min_duration_secs,
            max_duration_secs,
        )
    }

    pub fn create_market(
        ctx: Context<CreateMarket>,
        market_id: u64,
        title: String,
        image_url: Option<String>,
        end_ts: i64,
    ) -> Result<()> {
        instructions::create_market::handler(ctx, market_id, title, image_url, end_ts)
    }

    pub fn bet(
        ctx: Context<PlaceBet>,
        _market_id: u64,
        emoji_id: u32,
        vote_qty: u64,
    ) -> Result<()> {
        instructions::bet::handler(ctx, emoji_id, vote_qty)
    }

    pub fn end_market(ctx: Context<EndMarket>, _market_id: u64) -> Result<()> {
        instructions::end_market::handler(ctx)
    }

    pub fn claim(ctx: Context<Claim>, _market_id: u64) -> Result<()> {
        instructions::claim::handler(ctx)
    }
}
