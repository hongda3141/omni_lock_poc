use ckb_hash::blake2b_256;
use ckb_sdk::unlock::OmniLockConfig;
use ckb_sdk::util::blake160;

use ckb_types::core::ScriptHashType;
use ckb_types::packed::Script;
use ckb_types::prelude::*;
use ckb_types::{H160, H256};

use hex_literal::hex;

const OMNILOCK_BIN: &[u8] = include_bytes!("omni_lock");

fn main() {
    let h160_addr = H160::from_slice(&[0u8; 20]).unwrap();
    let res = build_omnilock_script1(&h160_addr);
    println!("res: {:?}", res);
    let res = build_omnilock_script2(&h160_addr);
    println!("res: {:?}", res);
}

fn build_omnilock_script1(h160_addr: &H160) -> Script {
    let cfg = OmniLockConfig::new_pubkey_hash(blake160(h160_addr.as_bytes()));
    let temp = blake2b_256(OMNILOCK_BIN);
    let omnilock_data_hash = H256::from(temp);
    println!("omnilock_data_hash: {:?}", omnilock_data_hash);
    Script::new_builder()
        .code_hash(omnilock_data_hash.pack())
        .hash_type(ScriptHashType::Data1.into())
        .args(cfg.build_args().pack())
        .build()
}

fn build_omnilock_script2(h160_addr: &H160) -> Script {
    let cfg = OmniLockConfig::new_pubkey_hash(blake160(h160_addr.as_bytes()));
    // let omnilock_code_hash = "0x9b819793a64463aed77c615d6cb226eea5487ccfc0783043a587254cda2b6f26";
    // let omnilock_data_hash = H256::from(blake2b_256(OMNILOCK_BIN));
    // let omnilock_data_hash = H256::from("0x9b819793a64463aed77c615d6cb226eea5487ccfc0783043a587254cda2b6f26");
    let data = hex!("9b819793a64463aed77c615d6cb226eea5487ccfc0783043a587254cda2b6f26");
    println!("data: {:?}", data);
    // let omnilock_data_hash = H256::from_slice(data.as_ref()).unwrap();
    let omnilock_code_hash = H256::from(data);
    println!("omnilock_data_hash: {:?}", omnilock_code_hash);

    Script::new_builder()
        .code_hash(omnilock_code_hash.pack())
        .hash_type(ScriptHashType::Data1.into())
        .args(cfg.build_args().pack())
        .build()
}

fn build_omnilock_script3(h160_addr: &H160, is_mainnet: bool) -> Script {
    let cfg = OmniLockConfig::new_pubkey_hash(blake160(h160_addr.as_bytes()));
    // reference : https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0042-omnilock/0042-omnilock.md#notes 
    let hash = match is_mainnet {
        true => hex!("9b819793a64463aed77c615d6cb226eea5487ccfc0783043a587254cda2b6f26"),
        false => hex!("f329effd1c475a2978453c8600e1eaf0bc2087ee093c3ee64cc96ec6847752cb"),
    };
    let omnilock_code_hash = H256::from(hash);
    println!("omnilock_data_hash: {:?}", omnilock_code_hash);

    Script::new_builder()
        .code_hash(omnilock_code_hash.pack())
        .hash_type(ScriptHashType::Data1.into())
        .args(cfg.build_args().pack())
        .build()
}

fn convert_str_to_H256(s: &str) -> H256 {
    let mut bytes = [0u8; 32];
    let s = s.trim_start_matches("0x");
    let s = s.as_bytes();
    for i in 0..32 {
        let mut byte = [0u8; 2];
        byte[0] = s[i * 2];
        byte[1] = s[i * 2 + 1];
        bytes[i] = u8::from_str_radix(std::str::from_utf8(&byte).unwrap(), 16).unwrap();
    }
    H256::from(bytes)
}

fn convert_str_to_bytes(s: &str) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let s = s.trim_start_matches("0x");
    let s = s.as_bytes();
    for i in 0..32 {
        let mut byte = [0u8; 2];
        byte[0] = s[i * 2];
        byte[1] = s[i * 2 + 1];
        bytes[i] = u8::from_str_radix(std::str::from_utf8(&byte).unwrap(), 16).unwrap();
    }
    bytes
}

enum OmniLockScriptCodeHash {
    Mirana(String),
    Pudge(String),
    
}