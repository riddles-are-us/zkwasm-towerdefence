//import initHostBind, * as hostbind from "./wasmbind/hostbind.js";
import { query, ZKWasmAppRpc, LeHexBN } from "zkwasm-ts-server";
import BN from 'bn.js';

const CMD_PLACE_TOWER = 1n;
const CMD_WITHDRAW_TOWER = 2n;
const CMD_MINT_TOWER = 3n;
const CMD_DROP_TOWER = 4n;
const CMD_UPGRADE_TOWER = 5n;
const CMD_COLLECT_REWARDS = 6n;
const CMD_WITHDRAW_REWARDS = 7n;

function createCommand(nonce: bigint, command: bigint, feature: bigint) {
  return (nonce << 16n) + (feature << 8n) + command;
}

let account = "1234";

const rpc = new ZKWasmAppRpc("http://localhost:3000");
//const rpc = new ZKWasmAppRpc("http://114.119.187.224:8085");

interface MapSize {
  width: number,
  height: number,
}

async function getMap(): Promise<MapSize> {
  let state:any = await rpc.queryState(account);
  let map = JSON.parse(state.data).global.map;
  return map;

}

async function getNonce(): Promise<bigint> {
  let state:any = await rpc.queryState(account);
  let nonce = 0n;
  if (state.data) {
    let data = JSON.parse(state.data);
    if (data.player) {
      nonce = BigInt(data.player.nonce);
    }
  }
  return nonce;
}

function bytesToHex(bytes: Array<number>): string  {
    return Array.from(bytes, byte => byte.toString(16).padStart(2, '0')).join('');
}

async function withdrawRewards(address: string, amount: bigint, nonce: bigint) {
  let addressBN = new BN(address, 16);
  let a = addressBN.toArray("be", 20); // 20 bytes = 160 bits and split into 4, 8, 8

  console.log("address is", address);
  console.log("address be is", a);


  /*
  (32 bit amount | 32 bit highbit of address)
  (64 bit mid bit of address (be))
  (64 bit tail bit of address (be))
  */


  let firstLimb = BigInt('0x' + bytesToHex(a.slice(0,4).reverse()));
  let sndLimb = BigInt('0x' + bytesToHex(a.slice(4,12).reverse()));
  let thirdLimb = BigInt('0x' + bytesToHex(a.slice(12, 20).reverse()));


  console.log("first is", firstLimb);
  console.log("snd is", sndLimb);
  console.log("third is", thirdLimb);

  try {
    let processStamp = await rpc.sendTransaction(
      [
        createCommand(nonce, CMD_WITHDRAW_REWARDS, 0n),
        (firstLimb << 32n) + amount,
        sndLimb,
        thirdLimb
      ], account);
    console.log("withdraw rewards processed at:", processStamp);
  } catch(e) {
    if (e instanceof Error) {
      console.log(e.message);
    }
    console.log("collect reward error at address:", address);
  }
}

async function collectRewardsFromTower(towerId: bigint, nonce: bigint) {
  let accountInfo = new LeHexBN(query(account).pkx).toU64Array();
  console.log("account info:", accountInfo);
  try {
    let processStamp = await rpc.sendTransaction([createCommand(nonce, CMD_COLLECT_REWARDS, 0n), towerId, accountInfo[1], accountInfo[2]], account);
    console.log("collect rewards processed at:", processStamp);
  } catch(e) {
    if (e instanceof Error) {
      console.log(e.message);
    }
    console.log("collect reward error at id:", towerId);
  }
}


async function main() {
  //sending_transaction([0n,0n,0n,0n], "1234");
  for (let y=0n; y<6n; y++) {
    let towerId = 1038n + y;
    let nonce = await getNonce();
    await collectRewardsFromTower(towerId, nonce);
  }
  let state:any = await rpc.queryState(account);
  let data = JSON.parse(state.data);
  console.log(`player final state is ${data}`);
  let nonce = await getNonce();
  await withdrawRewards("c177d1d314C8FFe1Ea93Ca1e147ea3BE0ee3E470", 1n, nonce);
  state = await rpc.queryState(account);
  data = JSON.parse(state.data);
  console.log(`player final state is ${data}`);
}

main();
// sending_transaction([2n<<32n,2n + (1n<<8n) + (3n<<16n),0n,0n], "1234");
