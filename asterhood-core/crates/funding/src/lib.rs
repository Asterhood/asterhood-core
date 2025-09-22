pub fn funding_index(prev: i128, rate_bps: i64) -> i128 {
    prev + (prev * (rate_bps as i128)) / 10_000
}
