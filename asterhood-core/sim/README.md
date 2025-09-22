# Sim

Reproducible backtests for the risk + funding engine.

```
# pseudo
for t in ticks:
  price = oracle[t]
  funding_idx = funding::funding_index(funding_idx, rate_bps[t])
  health = health::maintenance_health(equity, margin)
```
