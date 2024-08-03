use zkwasm_rest_abi::MERKLE_MAP;

use crate::config::GLOBAL;
pub struct WithdrawInfo { // 32bits in total
    feature: u32, // 4
    address: [u8; 20], // 20
    amount: u64, // 8
}

impl WithdrawInfo {
    pub fn new(limbs: &[u64; 3]) -> Self {
        let mut address = ((limbs[0] >> 32) as u32).to_le_bytes().to_vec();
        address.extend_from_slice(&limbs[1].to_le_bytes());
        address.extend_from_slice(&limbs[2].to_le_bytes());

        WithdrawInfo {
            feature: 1,
            address: address.try_into().unwrap(),
            amount: limbs[0] & 0xffffffff
        }
    }
}
pub struct SettlementInfo(Vec<WithdrawInfo>);

const WITHDRAW_OPCODE:[u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];

pub static mut SETTLEMENT: SettlementInfo = SettlementInfo(vec![]);

impl SettlementInfo {
    pub fn append_settlement(info: WithdrawInfo) {
        unsafe { SETTLEMENT.0.push(info) };
    }
    pub fn flush_settlement() -> Vec<u8> {
        zkwasm_rust_sdk::dbg!("flush settlement\n");
        let sinfo = unsafe { &mut SETTLEMENT };
        let mut bytes: Vec<u8> = Vec::with_capacity(sinfo.0.len() * 32);
        for s in &sinfo.0 {
            bytes.extend_from_slice(&s.feature.to_le_bytes());
            bytes.extend_from_slice(&s.address);
            bytes.extend_from_slice(&s.amount.to_le_bytes());
        }
        sinfo.0 = vec![];
        unsafe { GLOBAL.store() };
        bytes
    }
}
