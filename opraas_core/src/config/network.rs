use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NetworkConfig {
    #[serde(default = "defaults::l1_rpc_url", skip_serializing)]
    pub l1_rpc_url: String,
    pub max_sequencer_drift: u32,
    pub sequencer_window_size: u32,
    pub channel_timeout: u32,
    pub l2_block_time: u32,
    pub l1_block_time: u32,
    pub l2_chain_id: u32,
    pub l2_output_oracle_submission_interval: u32,
    pub l2_output_oracle_starting_block_number: u32,
    pub finalization_period_seconds: u32,
    pub base_fee_vault_minimum_withdrawal_amount: String,
    pub l1_fee_vault_minimum_withdrawal_amount: String,
    pub sequencer_fee_vault_minimum_withdrawal_amount: String,
    pub base_fee_vault_withdrawal_network: u32,
    pub l1_fee_vault_withdrawal_network: u32,
    pub sequencer_fee_vault_withdrawal_network: u32,
    pub enable_governance: bool,
    pub governance_token_symbol: String,
    pub governance_token_name: String,
    pub l2_genesis_block_gas_limit: String,
    pub l2_genesis_block_base_fee_per_gas: String,
    pub eip1559_denominator: u32,
    pub eip1559_elasticity: u32,
    pub l2_genesis_regolith_time_offset: String,
    pub system_config_start_block: u32,
    pub required_protocol_version: String,
    pub recommended_protocol_version: String,
    pub fund_dev_accounts: bool,
    pub fault_game_absolute_prestate: String,
    pub fault_game_max_depth: u32,
    pub fault_game_clock_extension: u32,
    pub fault_game_max_clock_duration: u32,
    pub fault_game_genesis_block: u32,
    pub fault_game_genesis_output_root: String,
    pub fault_game_split_depth: u32,
    pub fault_game_withdrawal_delay: u32,
    pub preimage_oracle_min_proposal_size: u32,
    pub preimage_oracle_challenge_period: u32,
    pub proof_maturity_delay_seconds: u32,
    pub dispute_game_finality_delay_seconds: u32,
    pub respected_game_type: u32,
    pub use_fault_proofs: bool,
}

mod defaults {
    use std::env;

    // accounts
    pub fn l1_rpc_url() -> String {
        env::var("L1_RPC_URL").expect("L1_RPC_URL must be set")
    }
}

impl NetworkConfig {
    pub fn null() -> Self {
        NetworkConfig {
            l1_rpc_url: defaults::l1_rpc_url(),
            max_sequencer_drift: 10,
            sequencer_window_size: 10,
            channel_timeout: 60,
            l2_block_time: 1,
            l1_block_time: 1,
            l2_chain_id: 101,
            l2_output_oracle_submission_interval: 1,
            l2_output_oracle_starting_block_number: 0,
            finalization_period_seconds: 10,
            base_fee_vault_minimum_withdrawal_amount: "0".to_string(),
            l1_fee_vault_minimum_withdrawal_amount: "0".to_string(),
            sequencer_fee_vault_minimum_withdrawal_amount: "0".to_string(),
            base_fee_vault_withdrawal_network: 0,
            l1_fee_vault_withdrawal_network: 0,
            sequencer_fee_vault_withdrawal_network: 0,
            enable_governance: false,
            governance_token_symbol: "OP".to_string(),
            governance_token_name: "Optimism".to_string(),
            l2_genesis_block_gas_limit: "0".to_string(),
            l2_genesis_block_base_fee_per_gas: "0".to_string(),
            eip1559_denominator: 1,
            eip1559_elasticity: 1,
            l2_genesis_regolith_time_offset: "0".to_string(),
            system_config_start_block: 0,
            required_protocol_version: "0".to_string(),
            recommended_protocol_version: "0".to_string(),
            fund_dev_accounts: false,
            fault_game_absolute_prestate: "0".to_string(),
            fault_game_max_depth: 0,
            fault_game_clock_extension: 0,
            fault_game_max_clock_duration: 0,
            fault_game_genesis_block: 0,
            fault_game_genesis_output_root: "0".to_string(),
            fault_game_split_depth: 0,
            fault_game_withdrawal_delay: 0,
            preimage_oracle_min_proposal_size: 0,
            preimage_oracle_challenge_period: 0,
            proof_maturity_delay_seconds: 0,
            dispute_game_finality_delay_seconds: 0,
            respected_game_type: 0,
            use_fault_proofs: false,
        }
    }
}