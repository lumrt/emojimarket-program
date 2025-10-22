## Next steps to finish implementation
- Implement SPL Token CPI transfers (create escrow token account per post, mint check for USDC) // current
- Implement on-chain pricing math: store per-emoji supplies in `Post` or a separate PDA and compute exponential price with u128 fixed-point
- Implement ResolveMarket flow with either batched payouts or per-user claim entitlements (recommended: pull pattern)
- Add rent-exempt account creation helper functions
- Add comprehensive unit tests for price math and handlers


## How to build


1. Install Solana toolchain and Rust with `rustup`
2. `cargo build-bpf` or `cargo build` depending on local setup