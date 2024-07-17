//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { query, verify_sign, LeHexBN } from "./sign.js";
import { ZKWasmAppRpc } from "./rpc.js";

const msgHash = new LeHexBN("0xb8f4201833cfcb9dffdd8cf875d6e1328d99b683e8373617a63f41d436a19f7c");
const pkx = new LeHexBN("0x7137da164bacaa9332b307e25c1abd906c5c240dcb27e520b84522a1674aab01");
const pky = new LeHexBN("0x33b51854d1cde428aa0379606752a341b85cf1d47803e22330a0c9d41ce59c2b");
const sigx = new LeHexBN("0x8a4414099b0a433851f7fb34e43febb1298193da413a35bb6951baac25f6ea24");
const sigy = new LeHexBN("0x7e243e753a4b0a792c02c9d550c5569fe5b46a9f710d9d123cd2865e0184fb12");
const sigr = new LeHexBN("0x1e900bb388808fe40f0a11a149a322f576448d497040d72c7b3ebc832d02e701");

let checksign = verify_sign(msgHash, pkx, pky, sigx, sigy, sigr);
console.log("checking signature ...", checksign);

const CMD_PLACE_TOWER = 1n;
const CMD_WITHDRAW_TOWER = 2n;
const CMD_MINT_TOWER = 3n;
const CMD_DROP_TOWER = 4n;
const CMD_UPGRADE_TOWER = 4n;

function createCommand(nonce: bigint, command: bigint, feature: bigint) {
  return (nonce << 48n) + (feature << 8n) + command;
}

let account = "1234";

const rpc = new ZKWasmAppRpc("http://localhost:3000");
//const rpc = new ZKWasmAppRpc("http://114.119.187.224:8085");

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
  if (state.player) {
    nonce = state.player.nonce;
  }

  let accountInfo = new LeHexBN(query(account).pkx).toU64Array();
  console.log("account info:", accountInfo);
  rpc.send_transaction([createCommand(nonce, CMD_MINT_TOWER, 0n), towerId, accountInfo[1], accountInfo[2]], account);


  // position of the tower we would like to place
  state = rpc.query_state([1n], account);
  nonce = state.player.nonce;
  let pos = x<<32n + y;
  rpc.send_transaction([createCommand(nonce, CMD_PLACE_TOWER, 0n), towerId, pos, 0n], account);


  state = rpc.query_state([1n], account);
  console.log(`player nonce is ${state.player.nonce}`);
}

main();
// sending_transaction([2n<<32n,2n + (1n<<8n) + (3n<<16n),0n,0n], "1234");


