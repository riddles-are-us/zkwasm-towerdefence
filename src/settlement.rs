use zkwasm_rest_abi::WithdrawInfo;
use crate::config::GLOBAL;
use crate::config::SERVER_ID;
pub struct SettlementInfo(Vec<u8>);

const UPGRADE_OPCODE: [u8; 4]  = [2, 0, 0, 0];

pub static mut SETTLEMENT: SettlementInfo = SettlementInfo(vec![]);
pub static mut UPGRADES: UpgradeInfo = UpgradeInfo {
    info: vec![]
};

pub struct UpgradeInfo { // 32bits in total
    pub info: Vec<u32>, // 20
}

impl UpgradeInfo {
    pub fn append(objid: u32, level: u8) {
        let upgrades = unsafe { &mut UPGRADES };
        upgrades.info.push(level as u32 + (objid << 8));
    }
}

impl FlushBytes for UpgradeInfo {
    fn flush(&self, bytes: &mut Vec<u8>) {
        self.info.chunks(4).for_each(|x| {
            let mut opcode = unsafe {UPGRADE_OPCODE.clone()};
            opcode[2] = x.len() as u8;
            opcode[1] = SERVER_ID;
            bytes.extend_from_slice(&opcode);
            for xi in x {
                bytes.extend_from_slice(&xi.to_be_bytes());
            }
        });
    }
}

pub trait FlushBytes {
    fn flush(&self, data: &mut Vec<u8>);
}

impl FlushBytes for WithdrawInfo {
    fn flush(&self, data: &mut Vec<u8>) {
        self.flush(data);
    }
}

impl SettlementInfo {
    pub fn append_settlement<T: FlushBytes>(info: T) {
        let bytes = unsafe { &mut SETTLEMENT.0 };
        info.flush(bytes);
    }
    pub fn flush_settlement() -> Vec<u8> {
        zkwasm_rust_sdk::dbg!("flush settlement\n");
        let sinfo = unsafe { &mut SETTLEMENT };
        let mut bytes = sinfo.0.clone();
        unsafe { UPGRADES.flush(&mut bytes)};
        sinfo.0 = vec![];
        unsafe { GLOBAL.store() };
        bytes
    }
}
