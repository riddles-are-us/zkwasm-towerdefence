//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { query, ZKWasmAppRpc, LeHexBN } from "zkwasm-ts-server";

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

async function getNonce(): Promise<bigint> {
  let state:any = await rpc.queryState(account);
  rpc.query_config();

  let nonce = 0n;
  if (state.data) {
    let data = JSON.parse(state.data);
    if (data.player) {
      nonce = BigInt(data.player.nonce);
    }
  }
  return nonce;
}

async function mintTower(towerId: bigint, nonce: bigint) {
  let accountInfo = new LeHexBN(query(account).pkx).toU64Array();
  console.log("account info:", accountInfo);
  try {
    let processStamp = await rpc.sendTransaction([createCommand(nonce, CMD_MINT_TOWER, 0n), towerId, accountInfo[1], accountInfo[2]], account);
    console.log("processed at:", processStamp);
  } catch(e) {
    console.log("mintTower error at id:", towerId);
  }
}


async function main() {
  //sending_transaction([0n,0n,0n,0n], "1234");
  let x = 0n;
  for (let y=0n; y<6n; y++) {
    let pos = (x<<32n) + y;
    let towerId = 1038n + y;
    let nonce = await getNonce();
    mintTower(towerId, nonce);
    nonce = await getNonce();
    try {
      let processStamp = await rpc.sendTransaction([createCommand(nonce, CMD_PLACE_TOWER, 0n), towerId, pos, 0n], account);
        console.log("place tower processed at:", processStamp);
    } catch(e) {
      console.log("place tower error:", pos, towerId);
    }
  }
  let state:any = await rpc.queryState(account);
  let data = JSON.parse(state.data);
  console.log(`player final state is ${data}`);
}

main();
// sending_transaction([2n<<32n,2n + (1n<<8n) + (3n<<16n),0n,0n], "1234");


