import { useQuery } from "@apollo/client"
import { useState, useEffect } from "react"
import { GET_CHAINHOOK_STORE } from "./queries"


export default function App() {
  const [blocks, setBlocks] = useState({});
  const { loading, data } = useQuery(GET_CHAINHOOK_STORE, {

    fetchPolicy: 'network-only', // Used for first execution
    nextFetchPolicy: 'network-only',
    // notifyOnNetworkStatusChange: true,
    pollInterval: 2000,
  });

  useEffect(() => {
    if (!data) return
    console.log(data);
    const store = JSON.parse(data.chainhookStore.store);
    console.log(store);
    const storeBlocks = {};
    Object.keys(store).forEach((height) => {
      const block = JSON.parse(store[height]);
      storeBlocks[height] = block;
    });
    setBlocks(storeBlocks)
  }, [data]);
  console.log("loading", loading);
  console.log(blocks);
  return (
    <div className="hiro">
      <h1 className="text-white pt-20 pl-10 text-4xl">Chainhook Block Explorer</h1>
      <div className="mt-5 flex flex-col justify-left px-10 space-y-10">
        {
          Object.keys(blocks).reverse().map((height) => {
            return <Card props={blocks[height]} />
          })
        }
      </div>
    </div>
  )
}


function Card({ props }) {
  let li = [];
  props.transactions.forEach((transaction) => {
    li.push(
      <li className="pl-10 px-6 py-4 ">
        <p className="font-medium">TX #{transaction.metadata.index + 1}</p>
        <div className="grid grid-cols-3 gap-4 place-content-between text-xs">
          <div className="col-span-2">({transaction.transaction_identifier.hash})</div>
          <div class="text-right">{transaction.metadata.outputs.length} {transaction.metadata.outputs.length === 1 ? "Output" : "Outputs"}</div>
          <div>Fee {transaction.metadata.fee}</div>
        </div>
      </li>
    );
    transaction.metadata.outputs.forEach((output, i) => {
      li.push(
        <li className="pl-14 px-6 py-4 ">
          <p className="font-medium">Output #{i + 1}</p>
          <div className="grid grid-cols-3 gap-4 place-content-between text-xs">
            <div className="col-span-2">Script Pubkey: {output.script_pubkey}</div>
            <div class="text-right">Value: {output.value}</div>
          </div>
        </li>
      );
    })
  });

  return (
    <div key={props.block_identifier.hash} className="overflow-hidden rounded-md bg-white shadow w-2/3 m-a border border-grey">
      <ul role="list" className="divide-y divide-grey ">
        <li className="px-6 py-4">
          <p className="font-medium">Block #{props.block_identifier.index}</p>
          <div className="grid grid-cols-3 gap-4 place-content-between text-xs">
            <div className="col-span-2">({props.block_identifier.hash})</div>
            <div className="text-right">{props.transactions.length} {props.transactions.length === 1 ? "Transaction" : "Transactions"}</div>
          </div>
        </li>
        {li}
      </ul>
    </div>
  )
}