use crate::error::ErrorCode;
use crate::state::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = payer,
        space = Config::LEN,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
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
    // Validate: platform_fee_bps + creator_fee_bps <= 10000
    require!(
        platform_fee_bps as u32 + creator_fee_bps as u32 <= 10000,
        ErrorCode::FeesTooHigh
    );

    // Validate durations
    require!(
        min_duration_secs < max_duration_secs,
        ErrorCode::InvalidDurationRange
    );

    // Validate base_price_lamports > 0
    require!(base_price_lamports > 0, ErrorCode::InvalidBasePrice);

    let config = &mut ctx.accounts.config;
    config.admin_address = admin_address;
    config.platform_fee_bps = platform_fee_bps;
    config.creator_fee_bps = creator_fee_bps;
    config.base_price_lamports = base_price_lamports;
    config.malus_k_millis = malus_k_millis;
    config.quad_a_micros = quad_a_micros;
    config.quad_b_micros = quad_b_micros;
    config.min_duration_secs = min_duration_secs;
    config.max_duration_secs = max_duration_secs;
    config.bump = ctx.bumps.config;

    Ok(())
}
