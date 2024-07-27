use zkwasm_rest_abi::MERKLE_MAP;

use crate::config::GLOBAL;
pub struct SettlementInfo(Vec<[u64; 3]>);

const WITHDRAW_OPCODE:[u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];


pub static mut SETTLEMENT: SettlementInfo = SettlementInfo(vec![]);

impl SettlementInfo {
    pub fn append_settlement(info: [u64; 3]) {
        unsafe { SETTLEMENT.0.push(info) };
    }
    pub fn flush_settlement() -> Vec<u8> {
        zkwasm_rust_sdk::dbg!("flush settlement\n");
        let sinfo = unsafe { &mut SETTLEMENT };
        let mut bytes: Vec<u8> = Vec::with_capacity(sinfo.0.len() * 32);
        for settlement in &sinfo.0 {
            for i in WITHDRAW_OPCODE {
                bytes.push(i)
            }
            for i in settlement[0].to_le_bytes() {
                bytes.push(i)
            }
            for i in settlement[1].to_le_bytes() {
                bytes.push(i)
            }
            for i in settlement[2].to_le_bytes() {
                bytes.push(i)
            }
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
