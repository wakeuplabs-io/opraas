use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    #[serde(default = "defaults::l1_rpc_url")]
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
