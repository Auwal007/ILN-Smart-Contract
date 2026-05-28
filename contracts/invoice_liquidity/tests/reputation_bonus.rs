#![cfg(test)]

use invoice_liquidity::config::{get_config, set_admin, set_config, update_config, Config, ConfigError};
use invoice_liquidity::invoice::{get_reputation, set_reputation, submit_invoice, InvoiceError};
use invoice_liquidity::rate_logic::calculate_effective_rate;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_rate_calculation_bonus_applied() {
    let base_rate = 1000;
    let rep_score = 80;
    let threshold = 80;
    let bonus = 200;
    let min_rate = 100;
    
    let res = calculate_effective_rate(base_rate, rep_score, threshold, bonus, min_rate).unwrap();
    assert_eq!(res, 800);
}

#[test]
fn test_rate_calculation_no_bonus() {
    let base_rate = 1000;
    let rep_score = 79;
    let threshold = 80;
    let bonus = 200;
    let min_rate = 100;
    
    let res = calculate_effective_rate(base_rate, rep_score, threshold, bonus, min_rate).unwrap();
    assert_eq!(res, 1000);
}

#[test]
fn test_rate_calculation_floor_enforced() {
    let base_rate = 300;
    let rep_score = 90;
    let threshold = 80;
    let bonus = 250;
    let min_rate = 100;
    
    let res = calculate_effective_rate(base_rate, rep_score, threshold, bonus, min_rate).unwrap();
    assert_eq!(res, 100);
}

#[test]
fn test_exact_threshold_match() {
    let base_rate = 500;
    let rep_score = 50;
    let threshold = 50;
    let bonus = 100;
    let min_rate = 50;
    
    let res = calculate_effective_rate(base_rate, rep_score, threshold, bonus, min_rate).unwrap();
    assert_eq!(res, 400);
}

#[test]
fn test_zero_reputation() {
    let base_rate = 500;
    let rep_score = 0;
    let threshold = 50;
    let bonus = 100;
    let min_rate = 50;
    
    let res = calculate_effective_rate(base_rate, rep_score, threshold, bonus, min_rate).unwrap();
    assert_eq!(res, 500);
}

#[test]
fn test_maximum_bonus_application() {
    let base_rate = 600;
    let rep_score = 99;
    let threshold = 50;
    let bonus = 500;
    let min_rate = 50;
    
    let res = calculate_effective_rate(base_rate, rep_score, threshold, bonus, min_rate).unwrap();
    assert_eq!(res, 100);
}

#[test]
fn test_zero_base_rate() {
    let base_rate = 0;
    let rep_score = 90;
    let threshold = 50;
    let bonus = 200;
    let min_rate = 50;
    
    let res = calculate_effective_rate(base_rate, rep_score, threshold, bonus, min_rate).unwrap();
    assert_eq!(res, 50);
}

#[test]
fn test_governance_setters_and_access_control() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    
    set_admin(&env, &admin);
    
    let initial_config = Config {
        high_rep_threshold: 80,
        bonus_bps: 200,
        min_discount_rate_bps: 100,
        decay_rate_bps: 0,
        decay_period_ledgers: 0,
        dispute_timeout_ledgers: 0,
    };
    
    assert!(set_config(&env, &initial_config).is_ok());
    
    let update_res = update_config(&env, &non_admin, 90, 300, 150, 0, 0, 0);
    assert_eq!(update_res, Err(ConfigError::Unauthorized));

    let update_res_admin = update_config(&env, &admin, 90, 300, 150, 0, 0, 0);
    assert!(update_res_admin.is_ok());
    
    let config = get_config(&env).unwrap();
    assert_eq!(config.high_rep_threshold, 90);
    assert_eq!(config.bonus_bps, 300);
    assert_eq!(config.min_discount_rate_bps, 150);

    let invalid_bonus_res = update_config(&env, &admin, 90, 501, 150, 0, 0, 0);
    assert_eq!(invalid_bonus_res, Err(ConfigError::InvalidBonusBps));

    let invalid_min_rate_res = update_config(&env, &admin, 90, 300, 0, 0, 0, 0);
    assert_eq!(invalid_min_rate_res, Err(ConfigError::InvalidMinDiscountRate));
}

#[test]
fn test_submit_invoice_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    set_admin(&env, &admin);

    let config = Config {
        high_rep_threshold: 80,
        bonus_bps: 150,
        min_discount_rate_bps: 50,
        decay_rate_bps: 0,
        decay_period_ledgers: 0,
        dispute_timeout_ledgers: 0,
    };
    set_config(&env, &config).unwrap();

    let freelancer = Address::generate(&env);
    let payer = Address::generate(&env);

    set_reputation(&env, &freelancer, 50);
    let inv1 = submit_invoice(&env, &freelancer, &payer, 10_000, 1700000000, 400).unwrap();
    assert_eq!(inv1.base_discount_rate_bps, 400);
    assert_eq!(inv1.effective_discount_rate_bps, 400);

    set_reputation(&env, &freelancer, 80);
    let inv2 = submit_invoice(&env, &freelancer, &payer, 10_000, 1700000000, 400).unwrap();
    assert_eq!(inv2.base_discount_rate_bps, 400);
    assert_eq!(inv2.effective_discount_rate_bps, 250);
}
