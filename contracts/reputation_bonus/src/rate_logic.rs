#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RateError {
    ArithmeticUnderflow,
    ArithmeticOverflow,
}

pub fn calculate_effective_rate(
    base_rate_bps: u32,
    reputation_score: u32,
    high_rep_threshold: u32,
    bonus_bps: u32,
    min_discount_rate_bps: u32,
) -> Result<u32, RateError> {
    if reputation_score >= high_rep_threshold {
        let reduced_rate = base_rate_bps.saturating_sub(bonus_bps);
        let effective_rate = core::cmp::max(reduced_rate, min_discount_rate_bps);
        Ok(effective_rate)
    } else {
        let effective_rate = core::cmp::max(base_rate_bps, min_discount_rate_bps);
        Ok(effective_rate)
    }
}
