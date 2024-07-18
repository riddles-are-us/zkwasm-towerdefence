//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { query, ZKWasmAppRpc } from "zkwasm-ts-server";
import { LeHexBN } from "./sign.js";

const CMD_PLACE_TOWER = 1n;
const CMD_WITHDRAW_TOWER = 2n;
const CMD_MINT_TOWER = 3n;
const CMD_DROP_TOWER = 4n;
const CMD_UPGRADE_TOWER = 5n;

function createCommand(nonce: bigint, command: bigint, feature: bigint) {
  return (nonce << 16n) + (feature << 8n) + command;
}

let account = "1234";

//const rpc = new ZKWasmAppRpc("http://localhost:3000");
const rpc = new ZKWasmAppRpc("http://114.119.187.224:8085");

async function mintTower() {
}


async function main() {
  //sending_transaction([0n,0n,0n,0n], "1234");
  let towerId = 0n;
  let x = 0n;
  let y = 0n;
  let state = rpc.query_state([1n], account);
  rpc.query_config();

  let nonce = 0n;
  if (state.data) {
    let data = JSON.parse(state.data);
    if (data.player) {
      nonce = BigInt(data.player.nonce);
    }
  }

  let accountInfo = new LeHexBN(query(account).pkx).toU64Array();
  console.log("account info:", accountInfo);
  rpc.send_transaction([createCommand(nonce, CMD_MINT_TOWER, 0n), towerId, accountInfo[1], accountInfo[2]], account);


  // position of the tower we would like to place
  state = rpc.query_state([1n], account);

  nonce = BigInt(JSON.parse(state.data).player.nonce);

  let pos = x<<32n + y;
  rpc.send_transaction([createCommand(nonce, CMD_PLACE_TOWER, 0n), towerId, pos, 0n], account);

  state = rpc.query_state([1n], account);
  nonce = BigInt(JSON.parse(state.data).player.nonce);
  console.log(`player nonce is ${nonce}`);
}

main();
// sending_transaction([2n<<32n,2n + (1n<<8n) + (3n<<16n),0n,0n], "1234");


