# NEAR Seed Phrase
Similar to [near-seed-phrase](https://github.com/near/near-seed-phrase) but in Rust version

## Usage

### Basic
```rust
use near_seed_phrase::{derive_keypair, NearDerivationPath, NearSeedPhrase, ToEncodedKey};
```

```rust
fn example() {
    let phrase = NearSeedPhrase::generate(12).unwrap();
    let keypair = derive_keypair(&phrase, "", &NearDerivationPath::default()).unwrap();
    println!("{}", keypair.secret.to_encoded_key());
    println!("{}", keypair.public.to_encoded_key());
}
```

### Macro
```rust
use near_seed_phrase::keypair;
```

```rust
fn example() {
    let phrase = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
    
    let keypair = keypair!(phrase);
    
    assert_eq!(
        keypair.secret,
        "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv"
    );
    assert_eq!(
        keypair.public,
        "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828"
    );
    
    let keypair = keypair!(phrase, "password");
    
    assert_eq!(
        keypair.secret,
        "ed25519:2PzwsYGPX3XWwz67qwj4wqq4VWF7M4ENFyhbhDQsMKLUvzTKCQQx1srnmzRDBsCqpsTwdCFiQN2ZoYCngTxdRrri"
    );
    assert_eq!(
        keypair.public,
        "ed25519:FoccWVCwm6dn9e7i1JHXskvac52iwUngHpLZMbdMggMk"
    );
    
    let keypair = keypair!(phrase, "", "m/44'/397'/1'");
    
    assert_eq!(
        keypair.secret,
        "ed25519:2Q6EAt5ikTSVT2HvAB6fb5XLFukrahVXFdSzdYt8sVxXRasDktuAx2hTY5yBiPju4wdqbevnSgKxfayKvUBShsRv"
    );
    assert_eq!(
        keypair.public,
        "ed25519:5yszd4dR4jgNhVpSo9oYT2RXLLWdEqqJ5y1WjLiwodTS"
    );
}
```
