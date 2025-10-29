use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Total fees exceed 100%")]
    FeesTooHigh,

    #[msg("Invalid duration range")]
    InvalidDurationRange,

    #[msg("Base price must be greater than zero")]
    InvalidBasePrice,

    #[msg("Market duration is too short")]
    DurationTooShort,

    #[msg("Market duration is too long")]
    DurationTooLong,

    #[msg("Market is not active")]
    MarketNotActive,

    #[msg("Market has not ended yet")]
    MarketNotEnded,

    #[msg("Market has already ended")]
    MarketEnded,

    #[msg("Vote quantity must be at least 1")]
    InvalidVoteQuantity,

    #[msg("User has no votes on winning emoji")]
    NoWinningVotes,

    #[msg("Rewards already claimed")]
    AlreadyClaimed,

    #[msg("Unauthorized: caller is not admin or creator")]
    Unauthorized,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,

    #[msg("Division by zero")]
    DivisionByZero,

    #[msg("Invalid calculation result")]
    InvalidCalculation,

    #[msg("Title too long")]
    TitleTooLong,

    #[msg("Image URL too long")]
    ImageUrlTooLong,
}
