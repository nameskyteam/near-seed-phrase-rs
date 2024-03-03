# NEAR Seed Phrase
Similar to [near-seed-phrase](https://github.com/near/near-seed-phrase) but in Rust version

## Install
```shell
cargo add near-seed-phrase
```

## Usage

### Basic
```rust
use near_seed_phrase::{derive_key, NearDerivationPath, NearSeedPhrase};
```

```rust
fn example() {
    let phrase = NearSeedPhrase::generate(12).unwrap();
    let secret_key = derive_key(&phrase, "", &NearDerivationPath::default()).unwrap();
    println!("{}", secret_key);
    println!("{}", secret_key.to_public_key());
}
```

### Macro
```rust
use near_seed_phrase::{derive_key, ToEncodedKey};
```

```rust
fn example() {
    let phrase = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
    let secret_key = derive_key!(phrase);
    
    assert_eq!(
        secret_key.to_encoded_key(),
        "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv"
    );
    assert_eq!(
        secret_key.to_public_key().to_encoded_key(),
        "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828"
    );
}
```
