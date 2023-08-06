use bip39::Mnemonic;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use slip10::{BIP32Path, Curve, derive_key_from_path};

const SEED_PHRASE: &str = "";
const PASS_PHRASE: &str = "";

fn main() {
    let (private_key, public_key) = get_near_keypair(
        SEED_PHRASE,
        PASS_PHRASE,
        NearPath::Master
    );

    println!("{}", private_key);
    println!("{}", public_key);
}

fn get_near_keypair(seed_phrase: &str, pass_phrase: &str, path: NearPath) -> (String, String) {
    let mnemonic = seed_phrase.parse::<Mnemonic>().unwrap();
    let path = path.to_path().parse::<BIP32Path>().unwrap();
    let seed = mnemonic.to_seed(pass_phrase);
    let key = derive_key_from_path(&seed, Curve::Ed25519, &path).unwrap();

    let secret = SecretKey::from_bytes(&key.key).unwrap();
    let public = PublicKey::from(&secret);
    let keypair = Keypair { secret, public };

    let private_key = format!("ed25519:{}", bs58::encode(keypair.to_bytes()).into_string());
    let public_key = format!("ed25519:{}", bs58::encode(keypair.public.as_bytes()).into_string());

    return (private_key, public_key)
}

enum NearPath {
    #[allow(unused)]
    Master,
    #[allow(unused)]
    Ledger
}

impl NearPath {
    pub fn to_path(&self) -> String {
        match self {
            Self::Master => "m/44'/397'/0'".to_string(),
            Self::Ledger => "m/44'/397'/0'/0'/1'".to_string()
        }
    }
}
