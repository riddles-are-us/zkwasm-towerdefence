//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { query, ZKWasmAppRpc, LeHexBN } from "zkwasm-ts-server";
import BN from 'bn.js';
import {Player} from "./api.js";

let player = new Player("1234");

async function main() {
  //sending_transaction([0n,0n,0n,0n], "1234");
  let map = await player.getMap();
  let x = 0n;
  let y = 6n;
  let pos = x + y * BigInt(map.width);
  let towerId = 10038n + pos;
  await player.collectRewardsFromTower(towerId);
  await player.dropTower(towerId);
  let state:any = await player.getState();
  console.log(`player final state is ${state}`);
}

main();
// sending_transaction([2n<<32n,2n + (1n<<8n) + (3n<<16n),0n,0n], "1234");
