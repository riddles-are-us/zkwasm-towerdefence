//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { query, LeHexBN, ZKWasmAppRpc } from "zkwasm-ts-server";

let account = "1234";

//const rpc = new ZKWasmAppRpc("http://localhost:3000");
const rpc = new ZKWasmAppRpc("http://114.119.187.224:8085");

async function main() {
  //sending_transaction([0n,0n,0n,0n], "1234");

  const tasks: Promise<JSON>[] = [];
  for (let i = 0; i < 100; i++) {
      tasks.push(rpc.queryState(account));
  }

  console.log("pressure test: query test started");
  await Promise.all(tasks);
  console.log("pressure test: query test finished");
}

main();
// sending_transaction([2n<<32n,2n + (1n<<8n) + (3n<<16n),0n,0n], "1234");


