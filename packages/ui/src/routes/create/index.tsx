import { ConfigInput } from "@/components/config-input";
import { L1Selector } from "@/components/l1-selector";
import { Button } from "@/components/ui";
import { createFileRoute } from "@tanstack/react-router";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useCallback, useState } from "react";
import { ConfigSection } from "@/components/config-section";

export const Route = createFileRoute("/create/")({
  component: CreateChain,
});

const schema = z.object({
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
});

type FormValues = z.infer<typeof schema>;

const networkConfig: {
  id: string;
  title: string;
  description?: string;
  inputs: {
    title: keyof FormValues;
    description: string;
    defaultValue?: string;
    type?: string;
    recommendedValue?: string;
    notes?: string;
    standardConfigRequirement?: string;
  }[];
}[] = [
  {
    id: "proposal-fields",
    title: "Proposal fields",
    description:
      "These fields apply to output root proposals. The l2OutputOracleSubmissionInterval is configurable, see the section below for guidance.",
    inputs: [
      {
        title: "finalization_period_seconds",
        description:
          "Number of seconds that a proposal must be available to challenge before it is considered finalized by the OptimismPortal contract",
        type: "Number of seconds",
        notes:
          "Must not be 0. Recommend 12 on test networks, seven days on production ones",
        standardConfigRequirement:
          "7 days. High security. Excessively safe upper bound that leaves enough time to consider social layer solutions to a hack if necessary. Allows enough time for other network participants to challenge the integrity of the corresponding output root.",
      },
      {
        title: "l2_output_oracle_submission_interval",
        description:
          "Number of blocks between proposals to the L2OutputOracle. Will be removed with the addition of permissionless proposals.",
        type: "Number of blocks",
        notes: "Must not be 0. 120 (4 minutes) is suggested.",
      },
      {
        title: "l2_output_oracle_starting_block_number",
        description:
          "Block number of the first OP Stack block. Typically this should be zero, but this may be non-zero for networks that have been upgraded from a legacy system (like OP Mainnet). Will be removed with the addition of permissionless proposals.",
        type: "Number",
        recommendedValue: "0",
        notes: "Should be 0 for new chains.",
      },
      {
        // TODO: we can probably calculate
        title: "l2_output_oracle_starting_timestamp",
        description:
          "Timestamp of the first OP Stack block. This MUST be the timestamp corresponding to the block defined by the l1StartingBlockTag. Will be removed with the addition of permissionless proposals.",
        type: "Number",
        notes:
          "this MUST be the timestamp corresponding to the block defined by the l1StartingBlockTag",
      },
    ],
  },
  {
    id: "blocks",
    title: "Blocks",
    description:
      "These fields apply to L2 blocks: Their timing, when they need to be written to L1, and how they get written.",
    inputs: [
      {
        title: "l2_block_time",
        description:
          "Number of seconds between each L2 block. Must be < L1 block time (12 on mainnet and Sepolia).",
        type: "Number of seconds",
        notes:
          "Must not be 0. Must be less than the L1 blocktime and a whole number.",
        standardConfigRequirement: "1 or 2 seconds.",
      },
      {
        title: "max_sequencer_drift",
        description:
          "How far the L2 timestamp can differ from the actual L1 timestamp.",
        type: "Number of seconds",
        recommendedValue: "1800",
        notes:
          "Must not be 0. 1800 (30 minutes) is the constant that takes effect with the Fjord activation.",
      },
      {
        title: "sequencer_window_size",
        description:
          "Maximum number of L1 blocks that a Sequencer can wait to incorporate the information in a specific L1 block. For example, if the window is 10 then the information in L1 block n must be incorporated by L1 block n+10.",
        type: "Number of blocks",
        notes: "Must not be 0. 3600 (12 hours) is suggested.",
        standardConfigRequirement:
          "3_600 base layer blocks (12 hours for an L2 on Ethereum, assuming 12 second L1 blocktime). This is an important value for constraining the sequencer's ability to re-order transactions; higher values would pose a risk to user protections.",
      },
      {
        title: "channel_timeout",
        description:
          "Maximum number of L1 blocks that a transaction channel frame can be considered valid. A transaction channel frame is a chunk of a compressed batch of transactions. After the timeout, the frame is dropped.",
        type: "Number of blocks",
        defaultValue: "50",
        notes:
          "This default value was introduced in the Granite network upgrade",
      },
      {
        title: "system_config_start_block",
        description:
          "Maximum number of L1 blocks that a transaction channel frame can be considered valid. A transaction channel frame is a chunk of a compressed batch of transactions. After the timeout, the frame is dropped.",
        type: "Number of blocks",
        defaultValue: "50",
        notes:
          "This default value was introduced in the Granite network upgrade",
      },
      {
        title: "batch_inbox_address",
        description:
          "Address that Sequencer transaction batches are sent to on L1.",
        type: "L1 Address",
        standardConfigRequirement:
          "Convention is versionByte || keccak256(bytes32(chainId))[:19], where || denotes concatenation, versionByte is 0x00, and chainId is a uint256. This is to cover the full range of chain ids, to the full uint256 size.",
      },
    ],
  },
  {
    id: "gas",
    title: "Gas",
    description:
      "Set such that Fee Margin is between 0 and 50%. No higher than 200_000_000 gas. Chain operators are driven to maintain a stable and reliable chain. When considering a change to this value, careful deliberation is necessary.",
    inputs: [
      {
        title: "l2_genesis_block_gas_limit",
        description:
          "L2GenesisBlockGasLimit represents the chain's block gas limit.",
        type: "Number",
        notes:
          "Must not be 0. Must be greater than MaxResourceLimit + SystemTxMaxGas.",
      },
      {
        title: "l2_genesis_block_base_fee_per_gas",
        description:
          "L2GenesisBlockBaseFeePerGas represents the base fee per gas.",
        type: "Number",
        notes: " L2 genesis block base fee per gas cannot be nil.",
      },
      {
        title: "eip1559_denominator",
        description:
          "EIP1559Denominator is the denominator of EIP1559 base fee market.",
        type: "Number",
        notes: "Must not be 0.",
      },
      {
        title: "eip1559_elasticity",
        description:
          "EIP1559Elasticity is the elasticity of the EIP1559 fee market.",
        type: "Number",
        notes: "Must not be 0.",
      },
      {
        title: "eip1559_denominator_canyon",
        description:
          "EIP1559DenominatorCanyon is the denominator of EIP1559 base fee market when Canyon is active.",
        type: "Number",
        notes: "Must not be 0 if Canyon is activated.",
        recommendedValue: "250",
      },
    ],
  },
  {
    id: "governance",
    title: "Governance",
    inputs: [
      {
        title: "enable_governance",
        description:
          "EnableGovernance determines whether to include governance token predeploy.",
        type: "boolean",
        recommendedValue: "false",
      },
      {
        title: "governance_token_symbol",
        description:
          "GovernanceTokenSymbol represents the ERC20 symbol of the GovernanceToken.",
        type: "string",
      },
      {
        title: "governance_token_name",
        description:
          "GovernanceTokenName represents the ERC20 name of the GovernanceToken",
        type: "string",
      },
    ],
  },
  {
    id: "minimum-fees",
    title: "Minimum fees withdrawal amount",
    description:
      "Withdrawals to L1 are expensive and the minimum fee is to prevent overhead costs of continuous tiny withdrawals. If the withdrawal execution costs more than the fee-reward, then the fee Must not be collected economically.",
    inputs: [
      {
        title: "base_fee_vault_minimum_withdrawal_amount",
        description:
          "BaseFeeVaultMinimumWithdrawalAmount represents the minimum withdrawal amount for the BaseFeeVault.",
        type: "Number in wei",
      },
      {
        title: "l1_fee_vault_minimum_withdrawal_amount",
        description:
          "L1FeeVaultMinimumWithdrawalAmount represents the minimum withdrawal amount for the L1FeeVault.",
        type: "Number in wei",
      },
      {
        title: "sequencer_fee_vault_minimum_withdrawal_amount",
        description:
          "SequencerFeeVaultMinimumWithdrawalAmount represents the minimum withdrawal amount for the SequencerFeeVault.",
        type: "Number in wei",
      },
    ],
  },
  {
    id: "withdrawal-network",
    title: "Withdrawal network",
    inputs: [
      {
        title: "base_fee_vault_withdrawal_network",
        description:
          "BaseFeeVaultWithdrawalNetwork represents the withdrawal network for the BaseFeeVault. value of 0 will withdraw ETH to the recipient address on L1 and a value of 1 will withdraw ETH to the recipient address on L2.",
        type: "Number representing network enum",
        notes: "Withdrawals to Ethereum are more expensive.",
      },
      {
        title: "l1_fee_vault_withdrawal_network",
        description:
          "L1FeeVaultWithdrawalNetwork represents the withdrawal network for the L1FeeVault. A value of 0 will withdraw ETH to the recipient address on L1 and a value of 1 will withdraw ETH to the recipient address on L2.",
        type: "Number representing network enum",
        notes: "Withdrawals to Ethereum are more expensive.",
      },

      {
        title: "sequencer_fee_vault_withdrawal_network",
        description:
          "SequencerFeeVaultWithdrawalNetwork represents the withdrawal network for the SequencerFeeVault. A value of 0 will withdraw ETH to the recipient address on L1 and a value of 1 will withdraw ETH to the recipient address on L2.",
        type: "Number representing network enum",
        notes: "Withdrawals to Ethereum are more expensive.",
      },
    ],
  },
  {
    id: "offset-values",
    title: "Offset values",
    inputs: [
      {
        title: "l2_genesis_regolith_time_offset",
        description:
          "L2GenesisRegolithTimeOffset is the number of seconds after genesis block that Regolith hard fork activates. Set it to 0 to activate at genesis. Nil to disable Regolith.",
        type: "Number in seconds",
        defaultValue: "nil",
        recommendedValue: "0x0",
        standardConfigRequirement: "Network upgrade (hardfork) is activated.",
      },
      {
        title: "l2_genesis_canyon_time_offset",
        description:
          "L2GenesisCanyonTimeOffset is the number of seconds after genesis block that Canyon hard fork activates. Set it to 0 to activate at genesis. Nil to disable Canyon.",
        type: "Number of seconds",
        defaultValue: "nil",
        recommendedValue: "0x0",
        standardConfigRequirement: "Network upgrade (hardfork) is activated.",
      },
    ],
  },
  {
    id: "fault-proofs",
    title: "Fault proofs",
    inputs: [
      {
        title: "fault_game_absolute_prestate",
        description:
          "FaultGameAbsolutePrestate is the absolute prestate of Cannon. This is computed by generating a proof from the 0th -> 1st instruction and grabbing the prestate from the output JSON. All honest challengers should agree on the setup state of the program.",
        type: "Hash",
      },
      {
        title: "fault_game_max_depth",
        description:
          "FaultGameMaxDepth is the maximum depth of the position tree within the fault dispute game. 2^{FaultGameMaxDepth} is how many instructions the execution trace bisection game supports. Ideally, this should be conservatively set so that there is always enough room for a full Cannon trace.",
        type: "Number",
      },
      {
        title: "fault_game_clock_extension",
        description:
          "FaultGameClockExtension is the amount of time that the dispute game will set the potential grandchild claim's, clock to, if the remaining time is less than this value at the time of a claim's creation.",
        type: "Number",
      },
      {
        title: "fault_game_max_clock_duration",
        description:
          "FaultGameMaxClockDuration is the maximum amount of time that may accumulate on a team's chess clock before they may no longer respond.",
        type: "Number",
      },
      {
        title: "fault_game_genesis_block",
        description: "FaultGameGenesisBlock is the block number for genesis.",
        type: "Number",
      },
      {
        title: "fault_game_genesis_output_root",
        description:
          "FaultGameGenesisOutputRoot is the output root for the genesis block.",
        type: "Hash",
      },
      {
        title: "fault_game_split_depth",
        description:
          "FaultGameSplitDepth is the depth at which the fault dispute game splits from output roots to execution trace claims.",
        type: "Number",
      },
      {
        title: "fault_game_withdrawal_delay",
        description:
          "FaultGameWithdrawalDelay is the number of seconds that users must wait before withdrawing ETH from a fault game.",
        type: "Number",
      },
      {
        title: "preimage_oracle_min_proposal_size",
        description:
          "PreimageOracleMinProposalSize is the minimum number of bytes that a large preimage oracle proposal can be.",
        type: "Number",
      },
      {
        title: "preimage_oracle_challenge_period",
        description:
          "PreimageOracleChallengePeriod is the number of seconds that challengers have to challenge a large preimage proposal.",
        type: "Number of seconds",
      },
    ],
  },
  {
    id: "miscellaneous",
    title: "Miscellaneous",
    inputs: [
      {
        title: "required_protocol_version",
        description:
          "RequiredProtocolVersion indicates the protocol version that nodes are recommended to adopt, to stay in sync with the network.",
        type: "Hex string",
      },
      {
        title: "recommended_protocol_version",
        description:
          "RecommendedProtocolVersion indicates the protocol version that nodes are recommended to adopt, to stay in sync with the network.",
        type: "Hex string",
      },
      {
        title: "fund_dev_accounts",
        description:
          "FundDevAccounts determines whether to fund the dev accounts. Should only be used during devnet deployments.",
        type: "boolean",
      },
    ],
  },
];

function CreateChain() {
  const {
    register,
    setValue,
    formState: { errors },
  } = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
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
    },
  });
  const [chainId, setChainId] = useState<number>(1);

  const onL1ChainSelect = useCallback((chainId: number) => {
    setChainId(chainId);
  }, []);

  return (
    <div className="min-h-screen w-screen flex">
      <aside className="w-[600px] h-screen overflow-y-scroll sticky top-0 px-6 py-10 border-r border-gray-300 space-y-2">
        <h2 className="font-medium">Settings</h2>
        <div className="">
          <a href="#section-1" className="text-gray-700 font-medium text-base">
            L1 chain
          </a>
        </div>

        {networkConfig.map((nc) => (
          <div className="space-y-2">
            <a
              href={`#${nc.id}`}
              className="font-medium text-base text-gray-700"
            >
              {nc.title}
            </a>
            <ul className="space-y-1 text-sm">
              {nc.inputs.map((i) => (
                <li className="pl-4">
                  <a href={`#${i.title}`} className="text-gray-600">
                    {i.title}
                  </a>
                </li>
              ))}
            </ul>
          </div>
        ))}
      </aside>

      <div className="max-w- mx-14 w-full my-10 divide-y">
        <div className="py-6 space-y-2">
          <h1 className="text-xl font-bold">Setup Chain</h1>
          <p className="text-sm">
            Lorem ipsum dolor sit amet consectetur, adipisicing elit. Fuga dicta
            ullam, harum atque similique voluptates? Voluptates dolore ullam
            hic, nesciunt deserunt deleniti quae veritatis consectetur harum
            neque odit cumque illum?
          </p>
        </div>

        <div className="space-y-3 py-6">
          <h2 className="text-xl font-bold">Advanced mode</h2>
          <div className="space-y-3">
            <span className="block text-sm text-neutral">
              Enable advanced mode to fine tune all parameters that define your
              chain
            </span>

            <select name="" id="" className="select select-bordered w-full">
              <option value="eth">Disabled</option>
              <option value="eth">Enabled</option>
            </select>
          </div>
        </div>

        <div className="space-y-3 py-6">
          <h2 className="text-xl font-bold">L1 chain</h2>
          <div className="space-y-3">
            <span className="block text-sm text-neutral">
              Lorem ipsum dolor sit amet consectetur adipisicing elit. Fuga
              cumque tenetur voluptatum sint repellat id qui nisi, quidem,
              aliquid facilis reprehenderit earum numquam corporis quaerat dolor
              itaque tempore vitae autem?
            </span>

            <L1Selector value={chainId} onSelect={onL1ChainSelect} />
          </div>
        </div>

        <div className="py-6">
          <h2 className="text-xl font-bold">Your chain</h2>

          {networkConfig.map((nc) => (
            <ConfigSection
              id={nc.title}
              title={nc.title}
              description={nc.description}
            >
              {nc.inputs.map((i) => (
                <ConfigInput
                  {...register(i.title)}
                  id={i.title}
                  title={i.title}
                  description={i.description}
                  type={i.type}
                  notes={i.notes}
                  standardConfigRequirement={i.standardConfigRequirement}
                />
              ))}
            </ConfigSection>
          ))}
        </div>

        <div className="py-6">
          <Button>Next</Button>
        </div>
      </div>
    </div>
  );
}
