//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { query, LeHexBN, ZKWasmAppRpc } from "zkwasm-ts-server";
import { Player } from "./api.js";

const player = new Player("2234");

async function main() {
  let data:any = await player.getState();
  //console.log(data.global.map.tiles);

  console.log("player info:");
  console.log(data.player);

  console.log("monsters info:");
  console.log(data.global.monsters);

  console.log("towers info:");
  console.log(JSON.stringify(data.global.towers));
  console.log(data.global.towers.length);

  /*
  let config = await rpc.query_config();
  console.log("config", config);
  */
}

main();

