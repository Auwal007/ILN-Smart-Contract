use soroban_sdk::{contracttype, Address, Env};
use crate::errors::ContractError;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub high_rep_threshold: u32,
    pub bonus_bps: u32,
    pub min_discount_rate_bps: u32,
    pub decay_rate_bps: u32,           // Basis points to decay per period (e.g., 50 = 0.5%)
    pub decay_period_ledgers: u64,     // Ledger count between decay applications
    pub dispute_timeout_ledgers: u64,  // Ledger count after which a dispute can be auto-resolved
}



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConfigError {
    Unauthorized,
    InvalidBonusBps,
    InvalidMinDiscountRate,
}

const MAX_BONUS_BPS: u32 = 500;



pub fn update_config(
    env: &Env,
    caller: &Address,
    high_rep_threshold: u32,
    bonus_bps: u32,
    min_discount_rate_bps: u32,
    decay_rate_bps: u32,
    decay_period_ledgers: u64,
    dispute_timeout_ledgers: u64,
) -> Result<(), ConfigError> {
    let admin = crate::storage::get_admin(env).ok_or(ConfigError::Unauthorized)?;
    caller.require_auth();
    if caller != &admin {
        return Err(ConfigError::Unauthorized);
    }

    if bonus_bps > MAX_BONUS_BPS {
        return Err(ConfigError::InvalidBonusBps);
    }
    if min_discount_rate_bps == 0 {
        return Err(ConfigError::InvalidMinDiscountRate);
    }

    let new_config = Config {
        high_rep_threshold,
        bonus_bps,
        min_discount_rate_bps,
        decay_rate_bps,
        decay_period_ledgers,
        dispute_timeout_ledgers,
    };

    crate::storage::set_config(env, &new_config);
    Ok(())
}

