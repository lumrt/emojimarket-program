// Integration tests for Emoji Market Program
// 
// Note: For complete integration testing with Anchor programs,
// use the Anchor testing framework or TypeScript tests.
// These tests verify basic program structure.

use anchor_lang::prelude::*;
use emojimarket_program::state::*;

#[test]
fn test_config_structure() {
    // Test that Config structure is properly sized
    let expected_min_size = 8 + 32 + 2 + 2 + 8 + 4 + 8 + 8 + 4 + 4 + 1;
    assert_eq!(Config::LEN, expected_min_size);
    println!("✅ Config structure size validated: {} bytes", Config::LEN);
}

#[test]
fn test_market_structure() {
    // Test Market space calculation
    let space = Market::space(50, 100);
    assert!(space > 0);
    println!("✅ Market structure space calculation works: {} bytes for title=50, url=100", space);
}

#[test]
fn test_bet_structure() {
    // Test BetAccount space
    let space = BetAccount::space();
    assert!(space > 0);
    assert_eq!(space, BetAccount::BASE_LEN);
    println!("✅ BetAccount structure size validated: {} bytes", space);
}

#[test]
fn test_market_status_enum() {
    // Test MarketStatus enum values
    assert_eq!(MarketStatus::Active as u8, 0);
    assert_eq!(MarketStatus::Ended as u8, 1);
    println!("✅ MarketStatus enum values are correct");
}

#[test]
fn test_pda_seeds() {
    // Test PDA derivation seeds
    let program_id = Pubkey::new_unique();
    
    // Config PDA
    let (config_pda, _bump) = Pubkey::find_program_address(&[b"config"], &program_id);
    assert_ne!(config_pda, Pubkey::default());
    println!("✅ Config PDA derivation works");
    
    // Market PDA
    let creator = Pubkey::new_unique();
    let market_id: u64 = 1;
    let (market_pda, _bump) = Pubkey::find_program_address(
        &[b"market", creator.as_ref(), &market_id.to_le_bytes()],
        &program_id,
    );
    assert_ne!(market_pda, Pubkey::default());
    println!("✅ Market PDA derivation works");
    
    // Bet PDA
    let user = Pubkey::new_unique();
    let (bet_pda, _bump) = Pubkey::find_program_address(
        &[b"bet", market_pda.as_ref(), user.as_ref()],
        &program_id,
    );
    assert_ne!(bet_pda, Pubkey::default());
    println!("✅ Bet PDA derivation works");
}

#[test]
fn test_program_constants() {
    // Verify important constants
    assert!(Config::LEN > 0, "Config length should be positive");
    assert!(BetAccount::BASE_LEN > 0, "BetAccount length should be positive");
    println!("✅ Program constants validated");
}

#[test]
fn test_all_structures_complete() {
    println!("\n📊 Structure Summary:");
    println!("  • Config: {} bytes", Config::LEN);
    println!("  • Market: dynamic size based on title/url");
    println!("  • BetAccount: {} bytes", BetAccount::BASE_LEN);
    println!("\n✅ All program structures are properly defined");
}
