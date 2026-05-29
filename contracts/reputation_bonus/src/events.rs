use soroban_sdk::{contractevent, Address, Env, Symbol};

/// Stable audit identifiers for governance-controlled reputation parameters.
/// Keep these strings unique and unchanged unless the audit schema changes.
pub const PARAM_HIGH_REP_THRESHOLD: &str = "high_rep_threshold";
pub const PARAM_BONUS_BPS: &str = "bonus_bps";
pub const PARAM_MIN_DISCOUNT_RATE_BPS: &str = "min_discount_rate_bps";

#[contractevent(topics = ["parameter_updated"])]
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterUpdated {
    #[topic]
    pub param_name: Symbol,
    pub old_value: i128,
    pub new_value: i128,
    #[topic]
    pub updated_by: Address,
}

pub fn emit_parameter_updated(
    env: &Env,
    param_name: &str,
    old_value: i128,
    new_value: i128,
    updated_by: &Address,
) {
    env.events().publish_event(&ParameterUpdated {
        param_name: Symbol::new(env, param_name),
        old_value,
        new_value,
        updated_by: updated_by.clone(),
    });
}
