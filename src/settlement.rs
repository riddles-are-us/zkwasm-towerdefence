use zkwasm_rest_abi::MERKLE_MAP;

use crate::config::GLOBAL;
pub struct SettleMentInfo(Vec<[u8; 4]>);


pub static mut SETTLEMENT: SettleMentInfo = SettleMentInfo(vec![]);

impl SettleMentInfo {
    pub fn append_settlement(info: [u8; 4]) {
        unsafe { SETTLEMENT.0.push(info) };
    }
    pub fn flush_settlement() -> Vec<u8> {
        zkwasm_rust_sdk::dbg!("flush settlement\n");
        let sinfo = unsafe { &mut SETTLEMENT };
        let mut bytes: Vec<u8> = Vec::with_capacity(sinfo.0.len() * 80);
        for settlement in &sinfo.0 {
            bytes.push(settlement[0]);
            bytes.push(settlement[1]);
            bytes.push(settlement[2]);
            bytes.push(settlement[3]);
        }
        sinfo.0 = vec![];
        let merkle_ref = unsafe {&mut MERKLE_MAP};
        let root = merkle_ref.merkle.root;
        zkwasm_rust_sdk::dbg!("pre merkle: {:?}", root);

        unsafe { GLOBAL.store() };
        let merkle_ref = unsafe {&mut MERKLE_MAP};
        let root = merkle_ref.merkle.root;
        zkwasm_rust_sdk::dbg!("post merkle: {:?}", root);
        bytes
    }
}
