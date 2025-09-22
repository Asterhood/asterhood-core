# AsterHood Core

Low-latency matching, funding, and settlement kernels for perps on Solana.

## Layout
- `program/` — on-chain logic (Anchor-compatible layout)
- `crates/health/` — risk & margin math
- `crates/funding/` — funding index calc + carry
- `sim/` — deterministic backtests

## Quick Stats
- tick loop: ~0.7ms
- risk ops/sec: ~18,900
- price cache jitter p99: 2.1µs
- uarch: SIMD-accelerated i128 math
