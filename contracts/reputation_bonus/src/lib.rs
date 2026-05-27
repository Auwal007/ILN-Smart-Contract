#![no_std]

pub mod config;
pub mod invoice;
pub mod rate_logic;

use soroban_sdk::{contract, contractimpl, Address, Env};
use crate::config::{Config, ConfigError, update_config, get_config, set_config, set_admin};
use crate::invoice::{Invoice, InvoiceError, submit_invoice, set_reputation, get_reputation};

#[contract]
pub struct ReputationBonusContract;

#[contractimpl]
impl ReputationBonusContract {
    pub fn init(env: Env, admin: Address) {
        set_admin(&env, &admin);
    }

    pub fn set_config(env: Env, config: Config) -> Result<(), ConfigError> {
        set_config(&env, &config)
    }

    pub fn get_config(env: Env) -> Result<Config, ConfigError> {
        get_config(&env)
    }

    pub fn update_config(
        env: Env,
        caller: Address,
        high_rep_threshold: u32,
        bonus_bps: u32,
        min_discount_rate_bps: u32,
    ) -> Result<(), ConfigError> {
        update_config(&env, &caller, high_rep_threshold, bonus_bps, min_discount_rate_bps)
    }

    pub fn set_reputation(env: Env, address: Address, score: u32) {
        set_reputation(&env, &address, score);
    }

    pub fn get_reputation(env: Env, address: Address) -> u32 {
        get_reputation(&env, &address)
    }

    pub fn submit_invoice(
        env: Env,
        freelancer: Address,
        payer: Address,
        amount: i128,
        due_date: u64,
        base_discount_rate_bps: u32,
    ) -> Result<Invoice, InvoiceError> {
        submit_invoice(&env, &freelancer, &payer, amount, due_date, base_discount_rate_bps)
    }
}
