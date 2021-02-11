use crate::envy_load;
/// External uses
use serde::Deserialize;
use zksync_types::AccountId;
use zksync_types::Address;

// There are two types of configs:
// The original one (with tx_interval_scaling_factor)
// And the public one (with max_tx_interval)

// It's easier for humans to think in factors
// But the rest of the codebase does not
// really care about the factor, it only needs the max_tx_interval

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct ForcedExitRequestsInternalConfig {
    pub enabled: bool,
    pub max_tokens_per_request: u8,
    pub recomended_tx_interval: i64,
    pub tx_interval_scaling_factor: f64,
    pub price_per_token: i64,
    pub digits_in_id: u8,
    pub wait_confirmations: i64,
    pub sender_private_key: String,
    pub sender_account_address: Address,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ForcedExitRequestsConfig {
    pub enabled: bool,
    pub max_tokens_per_request: u8,
    pub recomended_tx_interval: i64,
    pub max_tx_interval: i64,
    pub price_per_token: i64,
    pub digits_in_id: u8,
    pub wait_confirmations: i64,
    pub sender_private_key: String,
    pub sender_account_address: Address,
}

impl ForcedExitRequestsConfig {
    pub fn from_env() -> Self {
        let config: ForcedExitRequestsInternalConfig =
            envy_load!("forced_exit_requests", "FORCED_EXIT_REQUESTS_");

        let max_tx_interval: f64 =
            (config.recomended_tx_interval as f64) * config.tx_interval_scaling_factor;

        ForcedExitRequestsConfig {
            enabled: config.enabled,
            max_tokens_per_request: config.max_tokens_per_request,
            recomended_tx_interval: config.recomended_tx_interval,
            max_tx_interval: max_tx_interval.round() as i64,
            digits_in_id: config.digits_in_id,
            price_per_token: config.price_per_token,
            wait_confirmations: config.wait_confirmations,
            sender_private_key: config.sender_private_key,
            sender_account_address: config.sender_account_address,
        }
    }
}
