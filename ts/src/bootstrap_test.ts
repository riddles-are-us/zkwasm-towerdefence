//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { query, ZKWasmAppRpc, LeHexBN } from "zkwasm-ts-server";
import { Player } from "./api.js";

let account = "2234";
let player = new Player(account);

async function main() {
  //sending_transaction([0n,0n,0n,0n], "1234");
  let map = await player.getMap();
  for (let x=0n; x<8n; x++) {
    for (let y=0n; y<8n; y++) {
      let pos = x + y * BigInt(map.width);
      let towerId = 1038n + y;
      await player.mintTower(towerId);
      await player.placeTower(towerId, pos, 0n);
    }
  }
}

main();
// sending_transaction([2n<<32n,2n + (1n<<8n) + (3n<<16n),0n,0n], "1234");


