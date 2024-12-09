
export type L1ChainSettings = {
  chainId: number;
  gasToken: string;
}

export function L1Selector(props: { onSelect: (chain: L1ChainSettings) => void }) {
  return (
    <div className="border p-4 rounded-xl space-y-4">
      <select name="" id="" className="select select-bordered w-full">
        <option value="eth">Ethereum</option>
      </select>

      <hr />

      <div className="space-y-2">
        <div className="space-x-4">
          <span>Gas token:</span>
          <span>ETH</span>
        </div>

        <div className="space-x-4">
          <span>Chain Id:</span>
          <span>1</span>
        </div>
      </div>
    </div>
  );
}
