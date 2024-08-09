import { query, ZKWasmAppRpc, LeHexBN } from "zkwasm-ts-server";
import BN from 'bn.js';

function bytesToHex(bytes: Array<number>): string  {
    return Array.from(bytes, byte => byte.toString(16).padStart(2, '0')).join('');
}

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

interface MapSize {
  width: number,
  height: number,
}

//const rpc = new ZKWasmAppRpc("http://localhost:3000");
const rpc = new ZKWasmAppRpc("http://114.119.187.224:8085");

export class Player {
  processingKey: string;
  constructor(key: string) {
    this.processingKey = key
  }
  async getMap(): Promise<MapSize> {
    let state:any = await rpc.queryState(this.processingKey);
    let map = JSON.parse(state.data).global.map;
    return map;
  }

  async getState(): Promise<any> {
    let state:any = await rpc.queryState(this.processingKey);
    return JSON.parse(state.data);
  }
  async getNonce(): Promise<bigint> {
    let state:any = await rpc.queryState(this.processingKey);
    let nonce = 0n;
    if (state.data) {
      let data = JSON.parse(state.data);
      if (data.player) {
        nonce = BigInt(data.player.nonce);
      }
    }
    return nonce;
  }
  async mintTower(towerId: bigint) {
    let nonce = await this.getNonce();
    let accountInfo = new LeHexBN(query(this.processingKey).pkx).toU64Array();
    console.log("account info:", accountInfo);
    try {
      let processStamp = await rpc.sendTransaction(
        [createCommand(nonce, CMD_MINT_TOWER, 2n), towerId, accountInfo[1], accountInfo[2]],
        this.processingKey
      );
      console.log("mintTower processed at:", processStamp);
    } catch(e) {
      if (e instanceof Error) {
        console.log(e.message);
      }
      console.log("mintTower error at id:", towerId);
    }
  }

  async placeTower(towerId: bigint, pos: bigint, feature: bigint) {
    let nonce = await this.getNonce();
    let accountInfo = new LeHexBN(query(this.processingKey).pkx).toU64Array();
    try {
      let processStamp = await rpc.sendTransaction(
        [createCommand(nonce, CMD_PLACE_TOWER, feature), towerId, pos, 0n],
        this.processingKey
      );
      console.log("placeTower processed at:", processStamp);
    } catch(e) {
      if (e instanceof Error) {
        console.log(e.message);
      }
      console.log("placeTower error at id:", towerId);
    }
  }

  async dropTower(towerId: bigint) {
    let nonce = await this.getNonce();
    try {
      let processStamp = await rpc.sendTransaction(
        [createCommand(nonce, CMD_DROP_TOWER, 0n), towerId, 0n, 0n],
        this.processingKey
      );
      console.log("drop processed at:", processStamp);
    } catch(e) {
      if (e instanceof Error) {
        console.log(e.message);
      }
      console.log("dropTower error at id:", towerId);
    }
  }

  async upgradeTower(towerId: bigint, pos: bigint, nonce: bigint) {
    try {
      let processStamp = await rpc.sendTransaction(
        [createCommand(nonce, CMD_UPGRADE_TOWER, 0n), towerId, 0n, 0n],
        this.processingKey
      );
      console.log("upgrade processed at:", processStamp);
    } catch(e) {
      if (e instanceof Error) {
        console.log(e.message);
      }
      console.log("upgradeTower error at id:", towerId);
    }
  }

  async withdrawRewards(address: string, amount: bigint) {
    let nonce = await this.getNonce();
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
        ], this.processingKey);
      console.log("withdraw rewards processed at:", processStamp);
    } catch(e) {
      if (e instanceof Error) {
        console.log(e.message);
      }
      console.log("collect reward error at address:", address);
    }
  }

  async collectRewardsFromTower(towerId: bigint) {
    let accountInfo = new LeHexBN(query(this.processingKey).pkx).toU64Array();
    let nonce = await this.getNonce();
    try {
      let processStamp = await rpc.sendTransaction([createCommand(nonce, CMD_COLLECT_REWARDS, 0n), towerId, accountInfo[1], accountInfo[2]], this.processingKey);
      console.log("collect rewards processed at:", processStamp);
    } catch(e) {
      if (e instanceof Error) {
        console.log(e.message);
      }
      console.log("collect reward error at id:", towerId);
    }
  }


}

