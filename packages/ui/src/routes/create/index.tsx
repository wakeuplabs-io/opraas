import { ConfigInput } from "@/components/config-input";
import { L1ChainSettings, L1Selector } from "@/components/l1-selector";
import { Button } from "@/components/ui";
import { createFileRoute } from "@tanstack/react-router";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useCallback } from "react";
import { ConfigSection } from "@/components/config-section";

export const Route = createFileRoute("/create/")({
  component: CreateChain,
});

const schema = z.object({
  // l1 chain
  l1_block_time: z.number(),
  l1_chain_id: z.number(),
  l1_starting_block_tag: z.string().startsWith("0x"),
  l1_use_clique: z.boolean(),
  l1_genesis_block_timestamp: z.string().startsWith("0x"),

  // proposal fields
  finalization_period_seconds: z.number(),
  l2_output_oracle_submission_interval: z.number(),
  l2_output_oracle_starting_block_number: z.number(),
  l2_output_oracle_starting_timestamp: z.number(),
  // blocks
  l2_block_time: z.number(),
  max_sequencer_drift: z.number(),
  sequencer_window_size: z.number(),
  channel_timeout: z.number(),
  system_config_start_block: z.number(),
  batch_inbox_address: z.string().startsWith("0x"),
  // gas
  l2_genesis_block_gas_limit: z.string().startsWith("0x"),
  l2_genesis_block_base_fee_per_gas: z.string().startsWith("0x"),
  eip1559_denominator: z.number(),
  eip1559_elasticity: z.number(),
  eip1559_denominator_canyon: z.number(),
  // governance
  enable_governance: z.boolean(),
  governance_token_symbol: z.string(),
  governance_token_name: z.string(),
  // minimum fees withdrawal amount
  base_fee_vault_minimum_withdrawal_amount: z.string().startsWith("0x"),
  l1_fee_vault_minimum_withdrawal_amount: z.string().startsWith("0x"),
  sequencer_fee_vault_minimum_withdrawal_amount: z.string().startsWith("0x"),
  // withdrawal network
  base_fee_vault_withdrawal_network: z.number(),
  l1_fee_vault_withdrawal_network: z.number(),
  sequencer_fee_vault_withdrawal_network: z.number(),
  // offset values
  l2_genesis_regolith_time_offset: z.string().startsWith("0x"),
  l2_genesis_canyon_time_offset: z.string().startsWith("0x"),
  // miscellaneous
  required_protocol_version: z.string().startsWith("0x"),
  recommended_protocol_version: z.string().startsWith("0x"),
  fund_dev_accounts: z.boolean(),
  // fault proofs
  fault_game_absolute_prestate: z.string().startsWith("0x"),
  fault_game_genesis_output_root: z.string().startsWith("0x"),
  fault_game_max_depth: z.number(),
  fault_game_clock_extension: z.number(),
  fault_game_max_clock_duration: z.number(),
  fault_game_genesis_block: z.number(),
  fault_game_split_depth: z.number(),
  fault_game_withdrawal_delay: z.number(),
  preimage_oracle_min_proposal_size: z.number(),
  preimage_oracle_challenge_period: z.number(),

  // deprecated
  gas_price_oracle_overhead: z.number(),
  gas_price_oracle_scalar: z.number(),
});

type FormValues = z.infer<typeof schema>;

function CreateChain() {
  const {
    register,
    setValue,
    formState: { errors },
  } = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
      l1_block_time: 12,
      l1_chain_id: 1,
      l1_starting_block_tag:
        "0x9e6f90926f2f96c342298a504cb82d66fb43f8c8aa60768d78ea4648b4908ee4",
      l1_use_clique: true,
      l1_genesis_block_timestamp: "0x673c1c29",

      finalization_period_seconds: 12,
      l2_output_oracle_submission_interval: 120,
      l2_output_oracle_starting_block_number: 0,
      l2_output_oracle_starting_timestamp: -1,

      l2_block_time: 2,
      max_sequencer_drift: 600,
      sequencer_window_size: 3600,
      channel_timeout: 300,
      system_config_start_block: 0,
      batch_inbox_address: "0xff69000000000000000000000000001201101712",

      l2_genesis_block_gas_limit: "0x2faf080",
      l2_genesis_block_base_fee_per_gas: "0x3b9aca00",
      eip1559_denominator: 50,
      eip1559_elasticity: 10,
      eip1559_denominator_canyon: 250,

      enable_governance: false,
      governance_token_symbol: "OP",
      governance_token_name: "Optimism",

      base_fee_vault_minimum_withdrawal_amount: "0x8ac7230489e80000",
      l1_fee_vault_minimum_withdrawal_amount: "0x8ac7230489e80000",
      sequencer_fee_vault_minimum_withdrawal_amount: "0x8ac7230489e80000",

      base_fee_vault_withdrawal_network: 0,
      l1_fee_vault_withdrawal_network: 0,
      sequencer_fee_vault_withdrawal_network: 0,

      l2_genesis_regolith_time_offset: "0x0",
      l2_genesis_canyon_time_offset: "0x40",

      required_protocol_version:
        "0x0000000000000000000000000000000000000000000000000000000000000000",
      recommended_protocol_version:
        "0x0000000000000000000000000000000000000000000000000000000000000000",
      fund_dev_accounts: false,

      fault_game_absolute_prestate:
        "0x03c7ae758795765c6664a5d39bf63841c71ff191e9189522bad8ebff5d4eca98",
      fault_game_max_depth: 30,
      fault_game_clock_extension: 0,
      fault_game_max_clock_duration: 1200,
      fault_game_genesis_block: 0,
      fault_game_genesis_output_root:
        "0xDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEF",
      fault_game_split_depth: 14,
      fault_game_withdrawal_delay: 604800,
      preimage_oracle_min_proposal_size: 10000,
      preimage_oracle_challenge_period: 120,

      gas_price_oracle_overhead: 2100,
      gas_price_oracle_scalar: 1000000,
    },
  });

  const onL1ChainSelect = useCallback((chainData: L1ChainSettings) => {
    Object.entries(chainData).forEach(([key, value]) => {
      setValue(key as keyof FormValues, value as any);
    });

    // also update with defaults
  }, []);

  return (
    <div className="max-w-xl mx-auto w-full space-y-10 my-10">
      <h1 className="text-xl font-bold">My Chain</h1>

      <hr />

      <div className="space-y-4">
        <h2 className="text-xl font-bold">L1 chain</h2>
        <L1Selector onSelect={onL1ChainSelect} />
      </div>

      <hr />

      <div className="">
        <h2 className="text-xl font-bold">Your chain</h2>

        <ConfigSection
          title="Proposal fields"
          description="These fields apply to output root proposals. The l2OutputOracleSubmissionInterval is configurable, see the section below for guidance."
        >
          <ConfigInput
            {...register("finalization_period_seconds")}
            title="finalization_period_seconds"
            description="Number of seconds that a proposal must be available to challenge before it is considered finalized by the OptimismPortal contract."
            type="Number of seconds"
            notes="Must not be 0. Recommend 12 on test networks, seven days on production ones."
            standardConfigRequirement="7 days. High security. Excessively safe upper bound that leaves enough time to consider social layer solutions to a hack if necessary. Allows enough time for other network participants to challenge the integrity of the corresponding output root."
          />

          <ConfigInput
            {...register("l2_output_oracle_submission_interval")}
            title="l2_output_oracle_submission_interval"
            description="Number of blocks between proposals to the L2OutputOracle. Will be removed with the addition of permissionless proposals."
            type="Number of blocks"
            notes="Must not be 0. 120 (4 minutes) is suggested."
          />

          <ConfigInput
            {...register("l2_output_oracle_starting_block_number")}
            title="l2_output_oracle_starting_block_number"
            description="Block number of the first OP Stack block. Typically this should be zero, but this may be non-zero for networks that have been upgraded from a legacy system (like OP Mainnet). Will be removed with the addition of permissionless proposals."
            type="Number"
            recommendedValue="0"
            notes="Should be 0 for new chains."
          />

          {/* TODO: this we should calculate */}
          <ConfigInput
            {...register("l2_output_oracle_starting_timestamp")}
            title="l2_output_oracle_starting_timestamp"
            description="Timestamp of the first OP Stack block. This MUST be the timestamp corresponding to the block defined by the l1StartingBlockTag. Will be removed with the addition of permissionless proposals."
            type="Number"
            notes="this MUST be the timestamp corresponding to the block defined by the l1StartingBlockTag"
          />
        </ConfigSection>

        <ConfigSection
          title="Blocks"
          description="These fields apply to L2 blocks: Their timing, when they need to be written to L1, and how they get written."
        >
          <ConfigInput
            {...register("l2_block_time")}
            title="l2_block_time"
            description="Number of seconds between each L2 block. Must be < L1 block time (12 on mainnet and Sepolia)."
            type="Number of seconds"
            notes="Must not be 0. Must be less than the L1 blocktime and a whole number."
            standardConfigRequirement="1 or 2 seconds."
          />

          <ConfigInput
            {...register("max_sequencer_drift")}
            title="max_sequencer_drift"
            description="How far the L2 timestamp can differ from the actual L1 timestamp."
            type="Number of seconds"
            recommendedValue="1800"
            notes="Must not be 0. 1800 (30 minutes) is the constant that takes effect with the Fjord activation."
          />

          <ConfigInput
            {...register("sequencer_window_size")}
            title="sequencer_window_size"
            description="Maximum number of L1 blocks that a Sequencer can wait to incorporate the information in a specific L1 block. For example, if the window is 10 then the information in L1 block n must be incorporated by L1 block n+10."
            type="Number of blocks"
            notes="Must not be 0. 3600 (12 hours) is suggested."
            standardConfigRequirement="3_600 base layer blocks (12 hours for an L2 on Ethereum, assuming 12 second L1 blocktime). This is an important value for constraining the sequencer's ability to re-order transactions; higher values would pose a risk to user protections."
          />

          <ConfigInput
            {...register("channel_timeout")}
            title="channel_timeout"
            description="Maximum number of L1 blocks that a transaction channel frame can be considered valid. A transaction channel frame is a chunk of a compressed batch of transactions. After the timeout, the frame is dropped."
            type="Number of blocks"
            defaultValue="50"
            notes="This default value was introduced in the Granite network upgrade"
          />

          <ConfigInput
            {...register("system_config_start_block")}
            title="system_config_start_block"
            description="Maximum number of L1 blocks that a transaction channel frame can be considered valid. A transaction channel frame is a chunk of a compressed batch of transactions. After the timeout, the frame is dropped."
            type="Number of blocks"
            defaultValue="50"
            notes="This default value was introduced in the Granite network upgrade"
          />

          <ConfigInput
            {...register("batch_inbox_address")}
            title="batch_inbox_address"
            description="Address that Sequencer transaction batches are sent to on L1."
            type="L1 Address"
            standardConfigRequirement="Convention is versionByte || keccak256(bytes32(chainId))[:19], where || denotes concatenation, versionByte is 0x00, and chainId is a uint256. This is to cover the full range of chain ids, to the full uint256 size."
          />
        </ConfigSection>

        <ConfigSection
          title="Gas"
          description="Set such that Fee Margin is between 0 and 50%. No higher than 200_000_000 gas. Chain operators are driven to maintain a stable and reliable chain. When considering a change to this value, careful deliberation is necessary."
        >
          <ConfigInput
            {...register("l2_genesis_block_gas_limit")}
            title="l2_genesis_block_gas_limit"
            description="L2GenesisBlockGasLimit represents the chain's block gas limit."
            type="Number"
            notes="Must not be 0. Must be greater than MaxResourceLimit + SystemTxMaxGas."
          />

          <ConfigInput
            {...register("l2_genesis_block_base_fee_per_gas")}
            title="l2_genesis_block_base_fee_per_gas"
            description="L2GenesisBlockBaseFeePerGas represents the base fee per gas."
            type="Number"
            notes=" L2 genesis block base fee per gas cannot be nil."
          />

          <ConfigInput
            {...register("eip1559_denominator")}
            title="eip1559_denominator"
            description="EIP1559Denominator is the denominator of EIP1559 base fee market."
            type="Number"
            notes="Must not be 0."
          />

          <ConfigInput
            {...register("eip1559_elasticity")}
            title="eip1559_elasticity"
            description="EIP1559Elasticity is the elasticity of the EIP1559 fee market."
            type="Number"
            notes="Must not be 0."
          />

          <ConfigInput
            {...register("eip1559_denominator_canyon")}
            title="eip1559_denominator_canyon"
            description="EIP1559DenominatorCanyon is the denominator of EIP1559 base fee market when Canyon is active."
            type="Number"
            notes="Must not be 0 if Canyon is activated."
            recommendedValue="250"
          />
        </ConfigSection>

        <ConfigSection title="Governance">
          <ConfigInput
            {...register("enable_governance")}
            title="enable_governance"
            description="EnableGovernance determines whether to include governance token predeploy."
            type="boolean"
            recommendedValue="false"
          />

          <ConfigInput
            {...register("governance_token_symbol")}
            title="governance_token_symbol"
            description="GovernanceTokenSymbol represents the ERC20 symbol of the GovernanceToken."
            type="string"
          />

          <ConfigInput
            {...register("governance_token_name")}
            title="governance_token_name"
            description="GovernanceTokenName represents the ERC20 name of the GovernanceToken"
            type="string"
          />
        </ConfigSection>

        <ConfigSection
          title="Minimum fees withdrawal amount"
          description="Withdrawals to L1 are expensive and the minimum fee is to prevent overhead costs of continuous tiny withdrawals. If the withdrawal execution costs more than the fee-reward, then the fee Must not be collected economically."
        >
          <ConfigInput
            {...register("base_fee_vault_minimum_withdrawal_amount")}
            title="base_fee_vault_minimum_withdrawal_amount"
            description="BaseFeeVaultMinimumWithdrawalAmount represents the minimum withdrawal amount for the BaseFeeVault."
            type="Number in wei"
          />

          <ConfigInput
            {...register("l1_fee_vault_minimum_withdrawal_amount")}
            title="l1_fee_vault_minimum_withdrawal_amount"
            description="L1FeeVaultMinimumWithdrawalAmount represents the minimum withdrawal amount for the L1FeeVault."
            type="Number in wei"
          />

          <ConfigInput
            {...register("sequencer_fee_vault_minimum_withdrawal_amount")}
            title="sequencer_fee_vault_minimum_withdrawal_amount"
            description="SequencerFeeVaultMinimumWithdrawalAmount represents the minimum withdrawal amount for the SequencerFeeVault."
            type="Number in wei"
          />
        </ConfigSection>

        <ConfigSection title="Withdrawal network">
          <ConfigInput
            {...register("base_fee_vault_withdrawal_network")}
            title="base_fee_vault_withdrawal_network"
            description="BaseFeeVaultWithdrawalNetwork represents the withdrawal network for the BaseFeeVault. value of 0 will withdraw ETH to the recipient address on L1 and a value of 1 will withdraw ETH to the recipient address on L2."
            type="Number representing network enum"
            notes="Withdrawals to Ethereum are more expensive."
          />

          <ConfigInput
            {...register("l1_fee_vault_withdrawal_network")}
            title="l1_fee_vault_withdrawal_network"
            description="L1FeeVaultWithdrawalNetwork represents the withdrawal network for the L1FeeVault. A value of 0 will withdraw ETH to the recipient address on L1 and a value of 1 will withdraw ETH to the recipient address on L2."
            type="Number representing network enum"
            notes="Withdrawals to Ethereum are more expensive."
          />

          <ConfigInput
            {...register("sequencer_fee_vault_withdrawal_network")}
            title="sequencer_fee_vault_withdrawal_network"
            description="SequencerFeeVaultWithdrawalNetwork represents the withdrawal network for the SequencerFeeVault. A value of 0 will withdraw ETH to the recipient address on L1 and a value of 1 will withdraw ETH to the recipient address on L2."
            type="Number representing network enum"
            notes="Withdrawals to Ethereum are more expensive."
          />
        </ConfigSection>

        <ConfigSection title="Offset values">
          <ConfigInput
            {...register("l2_genesis_regolith_time_offset")}
            title="l2_genesis_regolith_time_offset"
            description="L2GenesisRegolithTimeOffset is the number of seconds after genesis block that Regolith hard fork activates. Set it to 0 to activate at genesis. Nil to disable Regolith."
            type="Number in seconds"
            defaultValue="nil"
            recommendedValue="0x0"
            standardConfigRequirement="Network upgrade (hardfork) is activated."
          />

          <ConfigInput
            {...register("l2_genesis_canyon_time_offset")}
            title="l2_genesis_canyon_time_offset"
            description="L2GenesisCanyonTimeOffset is the number of seconds after genesis block that Canyon hard fork activates. Set it to 0 to activate at genesis. Nil to disable Canyon."
            type="Number of seconds"
            defaultValue="nil"
            recommendedValue="0x0"
            standardConfigRequirement="Network upgrade (hardfork) is activated."
          />
        </ConfigSection>

        <ConfigSection title="Fault proofs">
          <ConfigInput
            {...register("fault_game_absolute_prestate")}
            title="fault_game_absolute_prestate"
            description="FaultGameAbsolutePrestate is the absolute prestate of Cannon. This is computed by generating a proof from the 0th -> 1st instruction and grabbing the prestate from the output JSON. All honest challengers should agree on the setup state of the program."
            type="Hash"
          />

          <ConfigInput
            {...register("fault_game_max_depth")}
            title="fault_game_max_depth"
            description="FaultGameMaxDepth is the maximum depth of the position tree within the fault dispute game. 2^{FaultGameMaxDepth} is how many instructions the execution trace bisection game supports. Ideally, this should be conservatively set so that there is always enough room for a full Cannon trace."
            type="Number"
          />

          <ConfigInput
            {...register("fault_game_clock_extension")}
            title="fault_game_clock_extension"
            description="FaultGameClockExtension is the amount of time that the dispute game will set the potential grandchild claim's, clock to, if the remaining time is less than this value at the time of a claim's creation."
            type="Number"
          />

          <ConfigInput
            {...register("fault_game_max_clock_duration")}
            title="fault_game_max_clock_duration"
            description="FaultGameMaxClockDuration is the maximum amount of time that may accumulate on a team's chess clock before they may no longer respond."
            type="Number"
          />

          <ConfigInput
            {...register("fault_game_genesis_block")}
            title="fault_game_genesis_block"
            description="FaultGameGenesisBlock is the block number for genesis."
            type="Number"
          />

          <ConfigInput
            {...register("fault_game_genesis_output_root")}
            title="fault_game_genesis_output_root"
            description="FaultGameGenesisOutputRoot is the output root for the genesis block."
            type="Hash"
          />

          <ConfigInput
            {...register("fault_game_split_depth")}
            title="fault_game_split_depth"
            description="FaultGameSplitDepth is the depth at which the fault dispute game splits from output roots to execution trace claims."
            type="Number"
          />

          <ConfigInput
            {...register("fault_game_withdrawal_delay")}
            title="fault_game_withdrawal_delay"
            description="FaultGameWithdrawalDelay is the number of seconds that users must wait before withdrawing ETH from a fault game."
            type="Number"
          />

          <ConfigInput
            {...register("preimage_oracle_min_proposal_size")}
            title="preimage_oracle_min_proposal_size"
            description="PreimageOracleMinProposalSize is the minimum number of bytes that a large preimage oracle proposal can be."
            type="Number"
          />

          <ConfigInput
            {...register("preimage_oracle_challenge_period")}
            title="preimage_oracle_challenge_period"
            description="PreimageOracleChallengePeriod is the number of seconds that challengers have to challenge a large preimage proposal."
            type="Number of seconds"
          />
        </ConfigSection>

        <ConfigSection title="Miscellaneous">
          <ConfigInput
            {...register("required_protocol_version")}
            title="required_protocol_version"
            description="RequiredProtocolVersion indicates the protocol version that nodes are recommended to adopt, to stay in sync with the network."
            type="Hex string"
          />

          <ConfigInput
            {...register("recommended_protocol_version")}
            title="recommended_protocol_version"
            description="RecommendedProtocolVersion indicates the protocol version that nodes are recommended to adopt, to stay in sync with the network."
            type="Hex string"
          />

          <ConfigInput
            {...register("fund_dev_accounts")}
            title="fund_dev_accounts"
            description="FundDevAccounts determines whether to fund the dev accounts. Should only be used during devnet deployments."
            type="boolean"
          />
        </ConfigSection>
      </div>

      <div>
        <Button>Create</Button>
      </div>
    </div>
  );
}
