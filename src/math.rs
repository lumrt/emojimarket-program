use crate::error::ErrorCode;
use anchor_lang::prelude::*;

/// Calculate the malus factor: exp((k * x) / (1 - x)) - 1
/// Where x is time progress [0, 1) and k = malus_k_millis / 1000
/// Returns the malus multiplied by 1e9 for precision
pub fn calculate_malus(
    elapsed_secs: i64,
    total_duration_secs: i64,
    malus_k_millis: u32,
) -> Result<u64> {
    if total_duration_secs == 0 {
        return err!(ErrorCode::DivisionByZero);
    }

    // Calculate x = elapsed / total using fixed point (multiply by 1e9)
    let x_nanos = (elapsed_secs as u128)
        .checked_mul(1_000_000_000)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(total_duration_secs as u128)
        .ok_or(ErrorCode::DivisionByZero)? as u64;

    // If x >= 1, that means market ended, return max malus
    if x_nanos >= 1_000_000_000 {
        return Ok(1_000_000_000_000); // Very high malus
    }

    // Calculate (1 - x) in nanos
    let one_minus_x = 1_000_000_000u64
        .checked_sub(x_nanos)
        .ok_or(ErrorCode::ArithmeticUnderflow)?;

    if one_minus_x == 0 {
        return Ok(1_000_000_000_000); // Very high malus
    }

    // Calculate k * x where k = malus_k_millis / 1000
    // k * x = (malus_k_millis / 1000) * (x_nanos / 1e9)
    // = (malus_k_millis * x_nanos) / (1000 * 1e9)
    let k_x_nanos = (malus_k_millis as u128)
        .checked_mul(x_nanos as u128)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(1000)
        .ok_or(ErrorCode::DivisionByZero)? as u64;

    // Calculate (k * x) / (1 - x)
    let exponent = (k_x_nanos as u128)
        .checked_mul(1_000_000_000)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(one_minus_x as u128)
        .ok_or(ErrorCode::DivisionByZero)? as u64;

    // Approximate exp(exponent) - 1 using Taylor series
    // exp(y) ≈ 1 + y + y²/2! + y³/3! + y⁴/4! + ...
    // We'll use first 5 terms for reasonable accuracy
    let malus = exp_taylor_minus_one(exponent)?;

    Ok(malus)
}

/// Approximate exp(x) - 1 using Taylor series where x is in nanos (x / 1e9)
/// Returns result * 1e9
fn exp_taylor_minus_one(x_nanos: u64) -> Result<u64> {
    // For small x: exp(x) - 1 ≈ x + x²/2 + x³/6 + x⁴/24 + x⁵/120
    // x is in nanos, so we need to be careful with scaling

    let x = x_nanos as u128;
    let scale = 1_000_000_000u128;

    // First term: x
    let mut result = x;

    // Second term: x²/2
    let x2 = x.checked_mul(x).ok_or(ErrorCode::ArithmeticOverflow)?;
    let term2 = x2.checked_div(scale * 2).ok_or(ErrorCode::DivisionByZero)?;
    result = result
        .checked_add(term2)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Third term: x³/6
    let x3 = x2.checked_mul(x).ok_or(ErrorCode::ArithmeticOverflow)?;
    let term3 = x3
        .checked_div(scale * scale * 6)
        .ok_or(ErrorCode::DivisionByZero)?;
    result = result
        .checked_add(term3)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Fourth term: x⁴/24
    let x4 = x3.checked_mul(x).ok_or(ErrorCode::ArithmeticOverflow)?;
    let term4 = x4
        .checked_div(scale * scale * scale * 24)
        .ok_or(ErrorCode::DivisionByZero)?;
    result = result
        .checked_add(term4)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Fifth term: x⁵/120
    let x5 = x4.checked_mul(x).ok_or(ErrorCode::ArithmeticOverflow)?;
    let term5 = x5
        .checked_div(scale * scale * scale * scale * 120)
        .ok_or(ErrorCode::DivisionByZero)?;
    result = result
        .checked_add(term5)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    Ok(result as u64)
}

/// Calculate quadratic uplift: f(n) = 1 + a*n + b*n²
/// where a = quad_a_micros / 1e6, b = quad_b_micros / 1e6
/// Returns the multiplier * 1e9 for precision
pub fn calculate_quadratic_uplift(n: u64, quad_a_micros: u64, quad_b_micros: u64) -> Result<u64> {
    let n_128 = n as u128;
    let scale = 1_000_000_000u128;

    // Start with 1.0 in our fixed point (1e9)
    let mut result = scale;

    // Add a*n term: (quad_a_micros / 1e6) * n = (quad_a_micros * n) / 1e6
    let a_n = (quad_a_micros as u128)
        .checked_mul(n_128)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_mul(1000) // Convert from micros (1e6) to nanos (1e9)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(1_000_000)
        .ok_or(ErrorCode::DivisionByZero)?;

    result = result
        .checked_add(a_n)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Add b*n² term: (quad_b_micros / 1e6) * n² = (quad_b_micros * n²) / 1e6
    let n_squared = n_128
        .checked_mul(n_128)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    let b_n2 = (quad_b_micros as u128)
        .checked_mul(n_squared)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_mul(1000) // Convert from micros to nanos
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(1_000_000)
        .ok_or(ErrorCode::DivisionByZero)?;

    result = result
        .checked_add(b_n2)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    Ok(result as u64)
}

/// Calculate the unit price for a vote
/// unit_price = base_price_lamports * (1 + malus) * f(n)
/// malus and f(n) are in nanos (1e9 scale)
pub fn calculate_unit_price(
    base_price_lamports: u64,
    malus_nanos: u64,
    quad_multiplier_nanos: u64,
) -> Result<u64> {
    let base = base_price_lamports as u128;
    let scale = 1_000_000_000u128;

    // Calculate (1 + malus) = (1e9 + malus_nanos)
    let one_plus_malus = scale
        .checked_add(malus_nanos as u128)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Multiply by base_price
    let price_with_malus = base
        .checked_mul(one_plus_malus)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(scale)
        .ok_or(ErrorCode::DivisionByZero)?;

    // Multiply by quadratic factor
    let final_price = price_with_malus
        .checked_mul(quad_multiplier_nanos as u128)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(scale)
        .ok_or(ErrorCode::DivisionByZero)?;

    Ok(final_price as u64)
}

/// Calculate total cost for vote_qty votes, rounding up
pub fn calculate_total_cost(unit_price: u64, vote_qty: u64) -> Result<u64> {
    let total = (unit_price as u128)
        .checked_mul(vote_qty as u128)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    Ok(total as u64)
}

/// Calculate fee amount from total using basis points
pub fn calculate_fee(total: u64, fee_bps: u16) -> Result<u64> {
    let fee = (total as u128)
        .checked_mul(fee_bps as u128)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(ErrorCode::DivisionByZero)?;

    Ok(fee as u64)
}

/// Calculate user's share of payout pool using 128-bit intermediates
pub fn calculate_user_share(
    payout_pool: u64,
    user_winning_votes: u64,
    total_winning_votes: u64,
) -> Result<u64> {
    if total_winning_votes == 0 {
        return err!(ErrorCode::DivisionByZero);
    }

    let share = (payout_pool as u128)
        .checked_mul(user_winning_votes as u128)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(total_winning_votes as u128)
        .ok_or(ErrorCode::DivisionByZero)?;

    Ok(share as u64)
}
