export function L1Selector(props: {
  value: number;
  onSelect: (chainId: number) => void;
}) {
  return (
    <select name="" id="" className="select select-bordered w-full">
      <option value="eth">Ethereum</option>
    </select>
  );
}
