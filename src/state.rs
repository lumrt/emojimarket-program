use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin_address: Pubkey,
    pub platform_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub base_price_lamports: u64,
    pub malus_k_millis: u32,
    pub quad_a_micros: u64,
    pub quad_b_micros: u64,
    pub min_duration_secs: u32,
    pub max_duration_secs: u32,
    pub bump: u8,
}

impl Config {
    pub const LEN: usize = 8 + // discriminator
        32 + // admin_address
        2 + // platform_fee_bps
        2 + // creator_fee_bps
        8 + // base_price_lamports
        4 + // malus_k_millis
        8 + // quad_a_micros
        8 + // quad_b_micros
        4 + // min_duration_secs
        4 + // max_duration_secs
        1; // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum MarketStatus {
    Active = 0,
    Ended = 1,
}

#[account]
pub struct Market {
    pub creator: Pubkey,
    pub title: String,
    pub image_url: Option<String>,
    pub start_ts: i64,
    pub end_ts: i64,
    pub status: u8,
    pub total_pot: u64,
    pub total_votes: u64,
    pub emoji_ids: Vec<u32>,
    pub emoji_votes: Vec<u64>,
    pub winner: Option<u32>,
    pub platform_fee_taken: u64,
    pub creator_fee_taken: u64,
    // Snapshot of pricing/fee params from Config
    pub base_price_lamports: u64,
    pub malus_k_millis: u32,
    pub quad_a_micros: u64,
    pub quad_b_micros: u64,
    pub platform_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub min_duration_secs: u32,
    pub max_duration_secs: u32,
    pub bump: u8,
}

impl Market {
    pub const BASE_LEN: usize = 8 + // discriminator
        32 + // creator
        4 + 100 + // title (max 100 chars)
        1 + 4 + 200 + // image_url (max 200 chars)
        8 + // start_ts
        8 + // end_ts
        1 + // status
        8 + // total_pot
        8 + // total_votes
        4 + (4 * 50) + // emoji_ids (max 50 emojis)
        4 + (8 * 50) + // emoji_votes (max 50 emojis)
        1 + 4 + // winner (Option<u32>)
        8 + // platform_fee_taken
        8 + // creator_fee_taken
        8 + // base_price_lamports
        4 + // malus_k_millis
        8 + // quad_a_micros
        8 + // quad_b_micros
        2 + // platform_fee_bps
        2 + // creator_fee_bps
        4 + // min_duration_secs
        4 + // max_duration_secs
        1; // bump

    pub fn space(title_len: usize, image_url_len: usize) -> usize {
        8 + // discriminator
        32 + // creator
        4 + title_len.min(100) + // title
        1 + if image_url_len > 0 { 4 + image_url_len.min(200) } else { 0 } + // image_url
        8 + // start_ts
        8 + // end_ts
        1 + // status
        8 + // total_pot
        8 + // total_votes
        4 + (4 * 50) + // emoji_ids (max 50 emojis)
        4 + (8 * 50) + // emoji_votes (max 50 emojis)
        1 + 4 + // winner
        8 + // platform_fee_taken
        8 + // creator_fee_taken
        8 + // base_price_lamports
        4 + // malus_k_millis
        8 + // quad_a_micros
        8 + // quad_b_micros
        2 + // platform_fee_bps
        2 + // creator_fee_bps
        4 + // min_duration_secs
        4 + // max_duration_secs
        1 // bump
    }
}

#[account]
pub struct BetAccount {
    pub market: Pubkey,
    pub user: Pubkey,
    pub emoji_ids: Vec<u32>,
    pub emoji_votes: Vec<u64>,
    pub total_spent: u64,
    pub claimed: bool,
    pub bump: u8,
}

impl BetAccount {
    pub const BASE_LEN: usize = 8 + // discriminator
        32 + // market
        32 + // user
        4 + (4 * 50) + // emoji_ids (max 50 emojis)
        4 + (8 * 50) + // emoji_votes (max 50 emojis)
        8 + // total_spent
        1 + // claimed
        1; // bump

    pub fn space() -> usize {
        Self::BASE_LEN
    }
}
