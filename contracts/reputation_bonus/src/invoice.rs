use crate::config::{get_config, ConfigError};
use crate::rate_logic::{calculate_effective_rate, RateError};
use soroban_sdk::{contracterror, contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum InvoiceStatus {
    Pending,
    Funded,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Invoice {
    pub id: u64,
    pub freelancer: Address,
    pub payer: Address,
    pub amount: i128,
    pub due_date: u64,
    pub base_discount_rate_bps: u32,
    pub effective_discount_rate_bps: u32,
    pub status: InvoiceStatus,
}

#[contracttype]
pub enum InvoiceKey {
    Invoice(u64),
    InvoiceCount,
    Reputation(Address),
}

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InvoiceError {
    ArithmeticError = 1,
    InvalidReputationScore = 2,
    ConfigErrorUnauthorized = 3,
    ConfigErrorInvalidBonusBps = 4,
    ConfigErrorInvalidMinDiscountRate = 5,
    RateErrorArithmeticUnderflow = 6,
    RateErrorArithmeticOverflow = 7,
}

impl From<ConfigError> for InvoiceError {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::Unauthorized => InvoiceError::ConfigErrorUnauthorized,
            ConfigError::InvalidBonusBps => InvoiceError::ConfigErrorInvalidBonusBps,
            ConfigError::InvalidMinDiscountRate => InvoiceError::ConfigErrorInvalidMinDiscountRate,
        }
    }
}

impl From<RateError> for InvoiceError {
    fn from(err: RateError) -> Self {
        match err {
            RateError::ArithmeticUnderflow => InvoiceError::RateErrorArithmeticUnderflow,
            RateError::ArithmeticOverflow => InvoiceError::RateErrorArithmeticOverflow,
        }
    }
}

pub fn get_reputation(env: &Env, address: &Address) -> u32 {
    env.storage()
        .persistent()
        .get(&InvoiceKey::Reputation(address.clone()))
        .unwrap_or(0)
}

pub fn set_reputation(env: &Env, address: &Address, score: u32) {
    env.storage()
        .persistent()
        .set(&InvoiceKey::Reputation(address.clone()), &score);
}

pub fn submit_invoice(
    env: &Env,
    freelancer: &Address,
    payer: &Address,
    amount: i128,
    due_date: u64,
    base_discount_rate_bps: u32,
) -> Result<Invoice, InvoiceError> {
    freelancer.require_auth();

    let config = get_config(env).map_err(InvoiceError::from)?;
    let rep_score = get_reputation(env, freelancer);

    let effective_rate = calculate_effective_rate(
        base_discount_rate_bps,
        rep_score,
        config.high_rep_threshold,
        config.bonus_bps,
        config.min_discount_rate_bps,
    )
    .map_err(InvoiceError::from)?;

    let count: u64 = env
        .storage()
        .instance()
        .get(&InvoiceKey::InvoiceCount)
        .unwrap_or(0);
    let next_id = count.checked_add(1).ok_or(InvoiceError::ArithmeticError)?;
    env.storage().instance().set(&InvoiceKey::InvoiceCount, &next_id);

    let invoice = Invoice {
        id: next_id,
        freelancer: freelancer.clone(),
        payer: payer.clone(),
        amount,
        due_date,
        base_discount_rate_bps,
        effective_discount_rate_bps: effective_rate,
        status: InvoiceStatus::Pending,
    };

    env.storage()
        .persistent()
        .set(&InvoiceKey::Invoice(next_id), &invoice);

    Ok(invoice)
}
