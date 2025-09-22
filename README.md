The trading + risk engine powering perps and vaults for the creator economy on Solana.

## Modules
- **Trade Engine** → order placement, matching, funding, settlement
- **Risk Engine** → 64-bit margin math, real-time health factors, partial liqs
- **Vault Router** → fee + funding distribution to USD1 vaults
- **Event Stream** → block-time logs of trades, liqs, vault flows

## System Stats
- Compiler runtime: 142ms avg
- Risk ops/sec: 18,900
- Vault tickrate: 512hz
- Funding loop: 0.7ms
- Uptime: 99.991% across 1.3M txs

## Docs
See [AsterHood Docs](https://github.com/AsterHood/asterhood-docs).
