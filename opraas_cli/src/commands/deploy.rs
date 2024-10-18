use crate::config::config::Config;
use opraas_core::opstack;
use opraas_core::opstack::network::NetworkConfig;

pub async fn deploy(cfg: &Config, target: &str) {
    println!("Deploying {}...", target);

    match target {
        "contracts" => deploy_contracts(cfg).await,
        _ => panic!("Unknown target: {}", target),
    }
}

async fn deploy_contracts(cfg: &Config) {
    let net_config = NetworkConfig::create(
        cfg.accounts.admin_address.clone(),
        cfg.accounts.admin_address.clone(),
        cfg.network.max_sequencer_drift,
        cfg.network.sequencer_window_size,
        cfg.network.channel_timeout,
        cfg.network.l2_block_time,
        cfg.network.l1_block_time,
        cfg.network.l2_chain_id,
        cfg.network.p2p_sequencer_address.clone(),
        cfg.network.batch_inbox_address.clone(),
        cfg.network.batch_sender_address.clone(),
        cfg.network.l2_output_oracle_submission_interval,
        cfg.network.l2_output_oracle_starting_block_number,
        cfg.network.l2_output_oracle_proposer.clone(),
        cfg.network.l2_output_oracle_challenger.clone(),
        cfg.network.finalization_period_seconds,
        cfg.network.proxy_admin_owner.clone(),
        cfg.network.base_fee_vault_recipient.clone(),
        cfg.network.l1_fee_vault_recipient.clone(),
        cfg.network.sequencer_fee_vault_recipient.clone(),
        cfg.network.base_fee_vault_minimum_withdrawal_amount.clone(),
        cfg.network.l1_fee_vault_minimum_withdrawal_amount.clone(),
        cfg.network
            .sequencer_fee_vault_minimum_withdrawal_amount
            .clone(),
        cfg.network.base_fee_vault_withdrawal_network,
        cfg.network.l1_fee_vault_withdrawal_network,
        cfg.network.sequencer_fee_vault_withdrawal_network,
        cfg.network.enable_governance,
        cfg.network.governance_token_symbol.clone(),
        cfg.network.governance_token_name.clone(),
        cfg.network.governance_token_owner.clone(),
        cfg.network.l2_genesis_block_gas_limit.clone(),
        cfg.network.l2_genesis_block_base_fee_per_gas.clone(),
        cfg.network.eip1559_denominator,
        cfg.network.eip1559_elasticity,
        cfg.network.l2_genesis_regolith_time_offset.clone(),
        cfg.network.system_config_start_block,
        cfg.network.required_protocol_version.clone(),
        cfg.network.recommended_protocol_version.clone(),
        cfg.network.fund_dev_accounts,
        cfg.network.fault_game_absolute_prestate.clone(),
        cfg.network.fault_game_max_depth,
        cfg.network.fault_game_clock_extension,
        cfg.network.fault_game_max_clock_duration,
        cfg.network.fault_game_genesis_block,
        cfg.network.fault_game_genesis_output_root.clone(),
        cfg.network.fault_game_split_depth,
        cfg.network.fault_game_withdrawal_delay,
        cfg.network.preimage_oracle_min_proposal_size,
        cfg.network.preimage_oracle_challenge_period,
        cfg.network.proof_maturity_delay_seconds,
        cfg.network.dispute_game_finality_delay_seconds,
        cfg.network.respected_game_type,
        cfg.network.use_fault_proofs,
    )
    .await;

    opstack::contracts::deploy(&cfg.sources.op_repo_target, &net_config).unwrap()
}
