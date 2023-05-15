use ckb_hash::blake2b_256;
use ckb_sdk::unlock::OmniLockConfig;
use ckb_sdk::util::blake160;

use ckb_types::core::ScriptHashType;
use ckb_types::packed::Script;
use ckb_types::prelude::*;
use ckb_types::{H160, H256};

const OMNILOCK_BIN: &[u8] = include_bytes!("omni_lock");

fn main() {
    let h160_addr = H160::from_slice(&[0u8; 20]).unwrap();

    let res = build_omnilock_script(&h160_addr);
    println!("res: {:?}", res);
}

fn build_omnilock_script(h160_addr: &H160) -> Script {
    let cfg = OmniLockConfig::new_pubkey_hash(blake160(h160_addr.as_bytes()));
    let omnilock_data_hash = H256::from(blake2b_256(OMNILOCK_BIN));
    Script::new_builder()
        .code_hash(omnilock_data_hash.pack())
        .hash_type(ScriptHashType::Data1.into())
        .args(cfg.build_args().pack())
        .build()
}
