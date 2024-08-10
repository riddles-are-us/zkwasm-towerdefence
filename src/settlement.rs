use zkwasm_rest_abi::WithdrawInfo;
use crate::config::GLOBAL;
pub struct SettlementInfo(Vec<u8>);

pub static mut SETTLEMENT: SettlementInfo = SettlementInfo(vec![]);

pub struct UpgradeInfo { // 32bits in total
    pub feature: u16, // 4
    pub level: u16,
    pub address: u32, // 20
}

impl UpgradeInfo {
    pub fn new(objid: u32, level: u16) -> Self {
        UpgradeInfo {
            feature: 2,
            level,
            address: objid,
        }
    }
}

impl FlushBytes for UpgradeInfo {
    fn flush(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.feature.to_le_bytes());
        bytes.extend_from_slice(&self.level.to_be_bytes());
        bytes.extend_from_slice(&self.address.to_be_bytes());
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
        let bytes = sinfo.0.clone();
        sinfo.0 = vec![];
        unsafe { GLOBAL.store() };
        bytes
    }
}
