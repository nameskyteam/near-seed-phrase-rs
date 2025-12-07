# NEAR Seed Phrase
Similar to [near-seed-phrase](https://github.com/near/near-seed-phrase) but in Rust version

## Install
```shell
cargo add near-seed-phrase
```

## Usage

### Basic
```rust
use near_seed_phrase::{derive_key, NearMnemonic, NearDerivationPath};
```

```rust
fn example() {
    let mnemonic = NearMnemonic::generate().unwrap();
    let private_key = derive_key(&mnemonic, "", &NearDerivationPath::default()).unwrap();
    println!("{}", private_key);
    println!("{}", private_key.get_public_key());
}
```

### Macro
```rust
use near_seed_phrase::derive_key;
```

```rust
fn example() {
    let phrase = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
    let private_key = derive_key!(phrase);
    
    assert_eq!(
        private_key.to_string(),
        "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv"
    );
    assert_eq!(
        private_key.get_public_key().to_string(),
        "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828"
    );
}
```
