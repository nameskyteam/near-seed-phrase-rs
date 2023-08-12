# NEAR Seed Phrase
Similar to [near-seed-phrase](https://github.com/near/near-seed-phrase) but in Rust version

## Usage

### Generate
```rust
use near_seed_phrase::{NearSeedPhrase, WordCount};
```

```rust
fn example() {
    let phrase = NearSeedPhrase::generate(WordCount::W12).unwrap();
    println!("{}", phrase);
}
```

### Convert
```rust
use near_seed_phrase::{public, secret};
```

```rust
fn example() {
    let phrase = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
    
    let secret = secret!(phrase);
    let public = public!(phrase);
    
    assert_eq!(secret, "ed25519:G94YBVktAVUFZWvYBtYmfpvVMNCtSf2x73bMfTCM9CfzyrUyN5X6VpTqr8QTCHYBTdUfzufDsTy3cR9CfNf74Bv");
    assert_eq!(public, "ed25519:2PQENDq3KABdr7cw1TH5B4AdXLqcyNXTTpWbdZh7k828");
}
```

### Convert With Custom Password (Passphrase)
```rust
fn example() {
    let phrase = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
    let password = "password";
    
    let secret = secret!(phrase, password);
    let public = public!(phrase, password);
    
    assert_eq!(secret, "ed25519:2PzwsYGPX3XWwz67qwj4wqq4VWF7M4ENFyhbhDQsMKLUvzTKCQQx1srnmzRDBsCqpsTwdCFiQN2ZoYCngTxdRrri");
    assert_eq!(public, "ed25519:FoccWVCwm6dn9e7i1JHXskvac52iwUngHpLZMbdMggMk");
}
```

### Convert With Custom Derivation Path
```rust
fn example() {
    let phrase = "fortune conduct light unusual gloom process wrap spare season exact anchor devote";
    let path = "m/44'/397'/1'";
    
    let secret = secret!(phrase, "", path);
    let public = public!(phrase, "", path);
    
    assert_eq!(secret, "ed25519:2Q6EAt5ikTSVT2HvAB6fb5XLFukrahVXFdSzdYt8sVxXRasDktuAx2hTY5yBiPju4wdqbevnSgKxfayKvUBShsRv");
    assert_eq!(public, "ed25519:5yszd4dR4jgNhVpSo9oYT2RXLLWdEqqJ5y1WjLiwodTS");
}
```
