use crate::error::ErrorCode;
use crate::state::{Config, Market};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(market_id: u64)]
pub struct CreateMarket<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = creator,
        space = Market::space(100, 200),
        seeds = [b"market", creator.key().as_ref(), &market_id.to_le_bytes()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateMarket>,
    _market_id: u64,
    title: String,
    image_url: Option<String>,
    end_ts: i64,
) -> Result<()> {
    let config = &ctx.accounts.config;
    let market = &mut ctx.accounts.market;
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    // Validate title length
    require!(title.len() <= 100, ErrorCode::TitleTooLong);

    // Validate image URL length
    if let Some(ref url) = image_url {
        require!(url.len() <= 200, ErrorCode::ImageUrlTooLong);
    }

    // Set start_ts = now
    let start_ts = now;

    // Calculate duration
    let duration_secs = end_ts
        .checked_sub(start_ts)
        .ok_or(ErrorCode::ArithmeticUnderflow)?;

    require!(duration_secs > 0, ErrorCode::DurationTooShort);

    // Validate duration in [min_duration_secs, max_duration_secs]
    require!(
        duration_secs >= config.min_duration_secs as i64,
        ErrorCode::DurationTooShort
    );
    require!(
        duration_secs <= config.max_duration_secs as i64,
        ErrorCode::DurationTooLong
    );

    // Initialize market
    market.creator = ctx.accounts.creator.key();
    market.title = title;
    market.image_url = image_url;
    market.start_ts = start_ts;
    market.end_ts = end_ts;
    market.status = 0; // Active
    market.total_pot = 0;
    market.total_votes = 0;
    market.emoji_ids = Vec::new();
    market.emoji_votes = Vec::new();
    market.winner = None;
    market.platform_fee_taken = 0;
    market.creator_fee_taken = 0;

    // Snapshot pricing/fee params from Config
    market.base_price_lamports = config.base_price_lamports;
    market.malus_k_millis = config.malus_k_millis;
    market.quad_a_micros = config.quad_a_micros;
    market.quad_b_micros = config.quad_b_micros;
    market.platform_fee_bps = config.platform_fee_bps;
    market.creator_fee_bps = config.creator_fee_bps;
    market.min_duration_secs = config.min_duration_secs;
    market.max_duration_secs = config.max_duration_secs;
    market.bump = ctx.bumps.market;

    Ok(())
}
