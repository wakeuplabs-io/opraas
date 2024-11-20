use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NetworkConfig {
    #[serde(default = "defaults::l1_rpc_url", skip_serializing)]
    pub l1_rpc_url: String,
    
    // accounts
    #[serde(default = "defaults::sequencer_address")]
    pub p2p_sequencer_address: String,
    #[serde(default = "defaults::batcher_address")]
    pub batch_sender_address: String,
    #[serde(default = "defaults::proposer_address")]
    pub l2_output_oracle_proposer: String,
    #[serde(default = "defaults::admin_address")]
    pub l2_output_oracle_challenger: String,
    #[serde(default = "defaults::admin_address")]
    pub proxy_admin_owner: String,
    #[serde(default = "defaults::admin_address")]
    pub final_system_owner: String,
    #[serde(default = "defaults::admin_address")]
    pub base_fee_vault_recipient: String,
    #[serde(default = "defaults::admin_address")]
    pub l1_fee_vault_recipient: String,
    #[serde(default = "defaults::admin_address")]
    pub sequencer_fee_vault_recipient: String,
    #[serde(default = "defaults::admin_address")]
    pub governance_token_owner: String,
    #[serde(default = "defaults::admin_address")]
    pub superchain_config_guardian: String,
    #[serde(default = "defaults::admin_address")]
    pub clique_signer_address: String,

    // rollop
    pub max_sequencer_drift: u32,
    pub sequencer_window_size: u32,
    pub channel_timeout: u32,
    pub l2_block_time: u32,
    pub l1_block_time: u32,
    pub l1_chain_id: u32,
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
    pub gas_price_oracle_overhead: u32,
    pub gas_price_oracle_scalar: u32,
    pub eip1559_denominator_canyon: u32,
    pub l2_genesis_canyon_time_offset: String,
    pub l1_starting_block_tag: String,
    pub l2_output_oracle_starting_timestamp: i32,
    pub l1_use_clique: bool,
    pub l1_genesis_block_timestamp: String,
    pub batch_inbox_address: String,
}

mod defaults {
    use std::env;

    pub fn l1_rpc_url() -> String {
        env::var("L1_RPC_URL").expect("L1_RPC_URL must be set")
    }

    pub fn admin_address() -> String {
        env::var("ADMIN_ADDRESS").expect("ADMIN_ADDRESS must be set")
    }

    pub fn batcher_address() -> String {
        env::var("BATCHER_ADDRESS").expect("BATCHER_ADDRESS must be set")
    }

    pub fn proposer_address() -> String {
        env::var("PROPOSER_ADDRESS").expect("PROPOSER_ADDRESS must be set")
    }

    pub fn sequencer_address() -> String {
        env::var("SEQUENCER_ADDRESS").expect("SEQUENCER_ADDRESS must be set")
    }
}

impl NetworkConfig {
    pub fn null() -> Self {
        Self {
            l1_rpc_url: "http://127.0.0.1:8545".to_string(),
            p2p_sequencer_address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            batch_sender_address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            l2_output_oracle_proposer: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            l2_output_oracle_challenger: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            proxy_admin_owner: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            final_system_owner: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            base_fee_vault_recipient: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            l1_fee_vault_recipient: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            sequencer_fee_vault_recipient: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            governance_token_owner: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            superchain_config_guardian: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            clique_signer_address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            l1_chain_id: 1,
            l2_chain_id: 101,
            max_sequencer_drift: 600,
            sequencer_window_size: 3600,
            channel_timeout: 300,
            l2_block_time: 2,
            l1_block_time: 12,
            l2_output_oracle_submission_interval: 120,
            l2_output_oracle_starting_block_number: 0,
            finalization_period_seconds: 12,
            base_fee_vault_minimum_withdrawal_amount: "0x8ac7230489e80000".to_string(),
            l1_fee_vault_minimum_withdrawal_amount: "0x8ac7230489e80000".to_string(),
            sequencer_fee_vault_minimum_withdrawal_amount: "0x8ac7230489e80000".to_string(),
            base_fee_vault_withdrawal_network: 0,
            l1_fee_vault_withdrawal_network: 0,
            sequencer_fee_vault_withdrawal_network: 0,
            enable_governance: false,
            governance_token_symbol: "OP".to_string(),
            governance_token_name: "Optimism".to_string(),
            l2_genesis_block_gas_limit: "0x2faf080".to_string(),
            l2_genesis_block_base_fee_per_gas: "0x3b9aca00".to_string(),
            eip1559_denominator: 50,
            eip1559_elasticity: 10,
            l2_genesis_regolith_time_offset: "0x0".to_string(),
            system_config_start_block: 0,
            required_protocol_version:
                "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            recommended_protocol_version:
                "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            fund_dev_accounts: false,
            fault_game_absolute_prestate:
                "0x03c7ae758795765c6664a5d39bf63841c71ff191e9189522bad8ebff5d4eca98".to_string(),
            fault_game_max_depth: 30,
            fault_game_clock_extension: 0,
            fault_game_max_clock_duration: 1200,
            fault_game_genesis_block: 0,
            fault_game_genesis_output_root:
                "0xDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEF".to_string(),
            fault_game_split_depth: 14,
            fault_game_withdrawal_delay: 604800,
            preimage_oracle_min_proposal_size: 10000,
            preimage_oracle_challenge_period: 120,
            gas_price_oracle_overhead: 2100,
            gas_price_oracle_scalar: 1000000,
            eip1559_denominator_canyon: 250,
            l2_genesis_canyon_time_offset: "0x40".to_string(),
            l1_starting_block_tag:
                "0x9e6f90926f2f96c342298a504cb82d66fb43f8c8aa60768d78ea4648b4908ee4".to_string(),
            l2_output_oracle_starting_timestamp: -1,
            l1_use_clique: true,
            l1_genesis_block_timestamp: "0x673c1c29".to_string(),
            batch_inbox_address: "0xff69000000000000000000000000001201101712".to_string(),
        }
    }
}
