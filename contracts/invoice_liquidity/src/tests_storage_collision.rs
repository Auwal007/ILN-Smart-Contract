#![cfg(test)]

use crate::storage::DataKey;
use soroban_sdk::{testutils::Address as _, Address, Env, IntoVal};

#[test]
fn test_datakey_serialization_isolation() {
    let env = Env::default();
    
    let admin_key = DataKey::Admin;
    let config_key = DataKey::Config;
    
    // Convert keys to raw Val to verify their serialized representation
    let admin_val = admin_key.into_val(&env);
    let config_val = config_key.into_val(&env);
    
    assert_ne!(admin_val.get_payload(), config_val.get_payload(), "Admin and Config must serialize differently");
}

#[test]
fn test_parameterized_datakey_isolation() {
    let env = Env::default();
    
    let invoice_1 = DataKey::Invoice(1);
    let invoice_2 = DataKey::Invoice(2);
    
    let inv1_val = invoice_1.into_val(&env);
    let inv2_val = invoice_2.into_val(&env);
    
    assert_ne!(inv1_val.get_payload(), inv2_val.get_payload(), "Invoice(1) and Invoice(2) must not collide");
    
    let addr_1 = Address::generate(&env);
    let addr_2 = Address::generate(&env);
    
    let payer_1 = DataKey::PayerScore(addr_1.clone());
    let payer_2 = DataKey::PayerScore(addr_2.clone());
    
    let p1_val = payer_1.into_val(&env);
    let p2_val = payer_2.into_val(&env);
    
    assert_ne!(p1_val.get_payload(), p2_val.get_payload(), "Payer scores for different addresses must not collide");
    
    let lp_1 = DataKey::LpScore(addr_1.clone());
    let lp_val = lp_1.into_val(&env);
    
    assert_ne!(p1_val.get_payload(), lp_val.get_payload(), "PayerScore and LpScore for the same address must not collide");
}
