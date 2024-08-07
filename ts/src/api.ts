import { query, ZKWasmAppRpc, LeHexBN } from "zkwasm-ts-server";

const CMD_PLACE_TOWER = 1n;
const CMD_WITHDRAW_TOWER = 2n;
const CMD_MINT_TOWER = 3n;
const CMD_DROP_TOWER = 4n;
const CMD_UPGRADE_TOWER = 5n;

function createCommand(nonce: bigint, command: bigint, feature: bigint) {
  return (nonce << 16n) + (feature << 8n) + command;
}

interface MapSize {
  width: number,
  height: number,
}

const rpc = new ZKWasmAppRpc("http://localhost:3000");
//const rpc = new ZKWasmAppRpc("http://114.119.187.224:8085");

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

  async dropTower(towerId: bigint, pos: bigint) {
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
}

