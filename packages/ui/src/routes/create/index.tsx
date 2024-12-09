import { ConfigInput } from "@/components/config-input";
import { L1ChainSettings, L1Selector } from "@/components/l1-selector";
import { Button } from "@/components/ui";
import { createFileRoute } from "@tanstack/react-router";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useCallback } from "react";

export const Route = createFileRoute("/create/")({
  component: CreateChain,
});

const schema = z.object({
  max_sequencer_drift: z.number(),
  sequencer_window_size: z.number(),
  channel_timeout: z.number(),
  l2_block_time: z.number(),
  l1_block_time: z.number(),
  l1_chain_id: z.number(),
  l2_chain_id: z.number(),
  l2_output_oracle_submission_interval: z.number(),
  l2_output_oracle_starting_block_number: z.number(),
  finalization_period_seconds: z.number(),
  base_fee_vault_minimum_withdrawal_amount: z.string(),
  l1_fee_vault_minimum_withdrawal_amount: z.string(),
  sequencer_fee_vault_minimum_withdrawal_amount: z.string(),
  base_fee_vault_withdrawal_network: z.number(),
  l1_fee_vault_withdrawal_network: z.number(),
  sequencer_fee_vault_withdrawal_network: z.number(),
  enable_governance: z.boolean(),
  governance_token_symbol: z.string(),
  governance_token_name: z.string(),
  l2_genesis_block_gas_limit: z.string(),
  l2_genesis_block_base_fee_per_gas: z.string(),
  l1_genesis_block_gas_limit: z.string(),
  l1_genesis_block_base_fee_per_gas: z.string(),
});

/*

*/

{
/* 
l1
l1_block_time = 12
l1_chain_id = 1
finalization_period_seconds = 12

l2
l2_block_time = 2
l2_genesis_block_gas_limit = "0x2faf080"
l2_genesis_block_base_fee_per_gas = "0x3b9aca00"

l2-governance
enable_governance = false
governance_token_symbol = "OP"
governance_token_name = "Optimism"



fees
base_fee_vault_minimum_withdrawal_amount = "0x8ac7230489e80000"
l1_fee_vault_minimum_withdrawal_amount = "0x8ac7230489e80000"
sequencer_fee_vault_minimum_withdrawal_amount = "0x8ac7230489e80000"
base_fee_vault_withdrawal_network = 0
l1_fee_vault_withdrawal_network = 0
sequencer_fee_vault_withdrawal_network = 0

advanced
l2_genesis_regolith_time_offset = "0x0"

max_sequencer_drift = 600
sequencer_window_size = 3600
channel_timeout = 300
l2_output_oracle_submission_interval = 120
l2_output_oracle_starting_block_number = 0
eip1559_denominator = 50
eip1559_elasticity = 10
system_config_start_block = 0
required_protocol_version = "0x0000000000000000000000000000000000000000000000000000000000000000"
recommended_protocol_version = "0x0000000000000000000000000000000000000000000000000000000000000000"
fund_dev_accounts = false
fault_game_absolute_prestate = "0x03c7ae758795765c6664a5d39bf63841c71ff191e9189522bad8ebff5d4eca98"
fault_game_max_depth = 30
fault_game_clock_extension = 0
fault_game_max_clock_duration = 1200
fault_game_genesis_block = 0
fault_game_genesis_output_root = "0xDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEF"
fault_game_split_depth = 14
fault_game_withdrawal_delay = 604800
preimage_oracle_min_proposal_size = 10000
preimage_oracle_challenge_period = 120
gas_price_oracle_overhead = 2100
gas_price_oracle_scalar = 1000000
eip1559_denominator_canyon = 250
l2_genesis_canyon_time_offset = "0x40"
l1_starting_block_tag = "0x9e6f90926f2f96c342298a504cb82d66fb43f8c8aa60768d78ea4648b4908ee4"
l2_output_oracle_starting_timestamp = -1
l1_use_clique = true
l1_genesis_block_timestamp = "0x673c1c29"
batch_inbox_address = "0xff69000000000000000000000000001201101712" */
}

type FormValues = z.infer<typeof schema>;

function CreateChain() {
  const {
    register,
    setValue,
    formState: { errors },
  } = useForm<FormValues>({
    resolver: zodResolver(schema),
  });

  const onL1ChainSelect = useCallback((chainData: L1ChainSettings) => {
    Object.entries(chainData).forEach(([key, value]) => {
      setValue(key as keyof FormValues, value as any);
    });
  }, [])

  return (
    <div className="max-w-xl mx-auto w-full space-y-10 my-10">
      <h1 className="text-xl font-bold">My Chain</h1>

      <section className="space-y-4">
        <h2 className="text-lg font-bold">Select the L1</h2>
        <L1Selector onSelect={onL1ChainSelect} />
      </section>

      <section className="space-y-4">
        <h2 className="text-lg font-bold">Configure your chain</h2>
        <div className="space-y-4">
          <ConfigInput
            {...register("l2_chain_id")}
            title="Chain id"
            defaultValue="1201101712"
            description="The chain id of your L2. Must be unique to the ecosystem."
            placeholder="Chain id"
          />
          <ConfigInput
            {...register("l2_chain_id")}
            title="Chain id"
            defaultValue="1201101712"
            description="The chain id of your L2. Must be unique to the ecosystem."
            placeholder="Chain id"
          />
        </div>
      </section>

      <div>
        <Button>Create</Button>
      </div>
    </div>
  );
}
