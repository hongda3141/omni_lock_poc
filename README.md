start test demo:
`cargo run`

source code:
`src/main.rs/build_omnilock_script`

code:
```rust
fn build_omnilock_script(h160_addr: &H160) -> Script {
    let cfg = OmniLockConfig::new_pubkey_hash(blake160(h160_addr.as_bytes()));
    let omnilock_data_hash = H256::from(blake2b_256(OMNILOCK_BIN));
    Script::new_builder()
        .code_hash(omnilock_data_hash.pack())
        .hash_type(ScriptHashType::Data1.into())
        .args(cfg.build_args().pack())
        .build()
}
```
