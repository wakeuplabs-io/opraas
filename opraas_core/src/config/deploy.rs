use crate::config::{AccountsConfig, NetworkConfig};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployConfig {
    final_system_owner: String,
    superchain_config_guardian: String,
    l1_starting_block_tag: String,
    l1_chain_id: u32,
    l2_chain_id: u32,
    l2_block_time: u32,
    l1_block_time: u32,
    max_sequencer_drift: u32,
    sequencer_window_size: u32,
    channel_timeout: u32,
    p2p_sequencer_address: String,
    batch_inbox_address: String,
    batch_sender_address: String,
    l2_output_oracle_submission_interval: u32,
    l2_output_oracle_starting_block_number: u32,
    l2_output_oracle_starting_timestamp: u32,
    l2_output_oracle_proposer: String,
    l2_output_oracle_challenger: String,
    finalization_period_seconds: u32,
    proxy_admin_owner: String,
    base_fee_vault_recipient: String,
    l1_fee_vault_recipient: String,
    sequencer_fee_vault_recipient: String,
    base_fee_vault_minimum_withdrawal_amount: String,
    l1_fee_vault_minimum_withdrawal_amount: String,
    sequencer_fee_vault_minimum_withdrawal_amount: String,
    base_fee_vault_withdrawal_network: u32,
    l1_fee_vault_withdrawal_network: u32,
    sequencer_fee_vault_withdrawal_network: u32,
    enable_governance: bool,
    governance_token_symbol: String,
    governance_token_name: String,
    governance_token_owner: String,
    l2_genesis_block_gas_limit: String,
    l2_genesis_block_base_fee_per_gas: String,
    eip1559_denominator: u32,
    eip1559_elasticity: u32,
    l2_genesis_regolith_time_offset: String,
    system_config_start_block: u32,
    required_protocol_version: String,
    recommended_protocol_version: String,
    fund_dev_accounts: bool,
    fault_game_absolute_prestate: String,
    fault_game_max_depth: u32,
    fault_game_clock_extension: u32,
    fault_game_max_clock_duration: u32,
    fault_game_genesis_block: u32,
    fault_game_genesis_output_root: String,
    fault_game_split_depth: u32,
    fault_game_withdrawal_delay: u32,
    preimage_oracle_min_proposal_size: u32,
    preimage_oracle_challenge_period: u32,
    proof_maturity_delay_seconds: u32,
    dispute_game_finality_delay_seconds: u32,
    respected_game_type: u32,
    use_fault_proofs: bool,
}

impl DeployConfig {
    pub async fn create(
        accounts_cfg: &AccountsConfig,
        network_cfg: &NetworkConfig,
    ) -> DeployConfig {
        let transport = web3::transports::Http::new(&network_cfg.l1_rpc_url).unwrap();
        let web3 = web3::Web3::new(transport);

        let l1_chain_id = web3.eth().chain_id().await.unwrap().as_u32();

        let finalized_block = web3
            .eth()
            .block({ web3::types::BlockId::Number(web3::types::BlockNumber::Finalized) })
            .await
            .unwrap()
            .unwrap();
        let l2_output_oracle_starting_timestamp = finalized_block.timestamp.as_u32();
        let l1_starting_block_tag = finalized_block.hash.unwrap();

        DeployConfig {
            // accounts
            final_system_owner: accounts_cfg.admin_address.clone(),
            superchain_config_guardian: accounts_cfg.admin_address.clone(),
            l2_output_oracle_challenger: accounts_cfg.admin_address.clone(),
            proxy_admin_owner: accounts_cfg.admin_address.clone(),
            base_fee_vault_recipient: accounts_cfg.admin_address.clone(),
            l1_fee_vault_recipient: accounts_cfg.admin_address.clone(),
            sequencer_fee_vault_recipient: accounts_cfg.admin_address.clone(),
            governance_token_owner: accounts_cfg.admin_address.clone(),
            p2p_sequencer_address: accounts_cfg.sequencer_address.clone(),
            batch_sender_address: accounts_cfg.batcher_address.clone(),
            l2_output_oracle_proposer: accounts_cfg.proposer_address.clone(),
            
            // TODO: this should be computed as a keccak
            batch_inbox_address: network_cfg.batch_inbox_address.clone(),

            // l1 params
            l1_chain_id,
            l1_block_time: network_cfg.l1_block_time,
            l1_starting_block_tag: format!("{:#x}", l1_starting_block_tag),
            
            // l2 params
            l2_chain_id: network_cfg.l2_chain_id,
            l2_output_oracle_starting_timestamp,
            l2_block_time: network_cfg.l2_block_time,

            // other params
            max_sequencer_drift: network_cfg.max_sequencer_drift,
            sequencer_window_size: network_cfg.sequencer_window_size,
            channel_timeout: network_cfg.channel_timeout,
            l2_output_oracle_submission_interval: network_cfg.l2_output_oracle_submission_interval,
            l2_output_oracle_starting_block_number: network_cfg
                .l2_output_oracle_starting_block_number,
            finalization_period_seconds: network_cfg.finalization_period_seconds,
            base_fee_vault_minimum_withdrawal_amount: network_cfg
                .base_fee_vault_minimum_withdrawal_amount
                .clone(),
            l1_fee_vault_minimum_withdrawal_amount: network_cfg
                .l1_fee_vault_minimum_withdrawal_amount
                .clone(),
            sequencer_fee_vault_minimum_withdrawal_amount: network_cfg
                .sequencer_fee_vault_minimum_withdrawal_amount
                .clone(),
            base_fee_vault_withdrawal_network: network_cfg.base_fee_vault_withdrawal_network,
            l1_fee_vault_withdrawal_network: network_cfg.l1_fee_vault_withdrawal_network,
            sequencer_fee_vault_withdrawal_network: network_cfg
                .sequencer_fee_vault_withdrawal_network,
            enable_governance: network_cfg.enable_governance,
            governance_token_symbol: network_cfg.governance_token_symbol.clone(),
            governance_token_name: network_cfg.governance_token_name.clone(),
            l2_genesis_block_gas_limit: network_cfg.l2_genesis_block_gas_limit.clone(),
            l2_genesis_block_base_fee_per_gas: network_cfg
                .l2_genesis_block_base_fee_per_gas
                .clone(),
            eip1559_denominator: network_cfg.eip1559_denominator,
            eip1559_elasticity: network_cfg.eip1559_elasticity,
            l2_genesis_regolith_time_offset: network_cfg.l2_genesis_regolith_time_offset.clone(),
            system_config_start_block: network_cfg.system_config_start_block,
            required_protocol_version: network_cfg.required_protocol_version.clone(),
            recommended_protocol_version: network_cfg.recommended_protocol_version.clone(),
            fund_dev_accounts: network_cfg.fund_dev_accounts,
            fault_game_absolute_prestate: network_cfg.fault_game_absolute_prestate.clone(),
            fault_game_max_depth: network_cfg.fault_game_max_depth,
            fault_game_clock_extension: network_cfg.fault_game_clock_extension,
            fault_game_max_clock_duration: network_cfg.fault_game_max_clock_duration,
            fault_game_genesis_block: network_cfg.fault_game_genesis_block,
            fault_game_genesis_output_root: network_cfg.fault_game_genesis_output_root.clone(),
            fault_game_split_depth: network_cfg.fault_game_split_depth,
            fault_game_withdrawal_delay: network_cfg.fault_game_withdrawal_delay,
            preimage_oracle_min_proposal_size: network_cfg.preimage_oracle_min_proposal_size,
            preimage_oracle_challenge_period: network_cfg.preimage_oracle_challenge_period,
            proof_maturity_delay_seconds: network_cfg.proof_maturity_delay_seconds,
            dispute_game_finality_delay_seconds: network_cfg.dispute_game_finality_delay_seconds,
            respected_game_type: network_cfg.respected_game_type,
            use_fault_proofs: network_cfg.use_fault_proofs,
        }
    }
}
