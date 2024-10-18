pub struct NetworkConfig {
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

impl NetworkConfig {
    pub async fn create(
        final_system_owner: String,
        superchain_config_guardian: String,
        max_sequencer_drift: u32,
        sequencer_window_size: u32,
        channel_timeout: u32,
        l2_block_time: u32,
        l1_block_time: u32,
        l2_chain_id: u32,
        p2p_sequencer_address: String,
        batch_inbox_address: String,
        batch_sender_address: String,
        l2_output_oracle_submission_interval: u32,
        l2_output_oracle_starting_block_number: u32,
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
    ) -> NetworkConfig {
        let l1_chain_id = l2_chain_id - 1;
        let l2_output_oracle_starting_timestamp = l2_output_oracle_starting_block_number * l2_block_time;
        let l1_starting_block_tag = l2_output_oracle_starting_block_number.to_string();

        NetworkConfig {
            // computed
            l1_chain_id,
            l1_starting_block_tag,
            l2_output_oracle_starting_timestamp,

            //  user provided
            l2_block_time,
            l1_block_time,
            l2_chain_id,
            final_system_owner,
            superchain_config_guardian,
            max_sequencer_drift,
            sequencer_window_size,
            channel_timeout,
            p2p_sequencer_address,
            batch_inbox_address,
            batch_sender_address,
            l2_output_oracle_submission_interval,
            l2_output_oracle_starting_block_number,
            l2_output_oracle_proposer,
            l2_output_oracle_challenger,
            finalization_period_seconds,
            proxy_admin_owner,
            base_fee_vault_recipient,
            l1_fee_vault_recipient,
            sequencer_fee_vault_recipient,
            base_fee_vault_minimum_withdrawal_amount,
            l1_fee_vault_minimum_withdrawal_amount,
            sequencer_fee_vault_minimum_withdrawal_amount,
            base_fee_vault_withdrawal_network,
            l1_fee_vault_withdrawal_network,
            sequencer_fee_vault_withdrawal_network,
            enable_governance,
            governance_token_symbol,
            governance_token_name,
            governance_token_owner,
            l2_genesis_block_gas_limit,
            l2_genesis_block_base_fee_per_gas,
            eip1559_denominator,
            eip1559_elasticity,
            l2_genesis_regolith_time_offset,
            system_config_start_block,
            required_protocol_version,
            recommended_protocol_version,
            fund_dev_accounts,
            fault_game_absolute_prestate,
            fault_game_max_depth,
            fault_game_clock_extension,
            fault_game_max_clock_duration,
            fault_game_genesis_block,
            fault_game_genesis_output_root,
            fault_game_split_depth,
            fault_game_withdrawal_delay,
            preimage_oracle_min_proposal_size,
            preimage_oracle_challenge_period,
            proof_maturity_delay_seconds,
            dispute_game_finality_delay_seconds,
            respected_game_type,
            use_fault_proofs,
        }
    }
}
