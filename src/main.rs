use num_bigint::BigInt;
use sha3::{Digest, Sha3_224};
use std::collections::HashMap;
use std::env;
use std::ops::Add;

const PRIME: &str = "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183995497CEA956AE515D2261898FA051015728E5A8AACAA68FFFFFFFFFFFFFFFF";
const PREFIX: usize = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pub_key_alice = BigInt::parse_bytes(&args[1].as_bytes(), 16).unwrap();
    let pub_key_bob = BigInt::parse_bytes(&args[2].as_bytes(), 16).unwrap();

    println!("Public key of Alice: {}", pub_key_alice);
    println!("Public key of Bob: {}", pub_key_bob);

    // initialization
    let mut priv_key_by_alice_hash: HashMap<String, String> = HashMap::new(); // in communication with Alice
    let mut priv_key_by_bob_hash: HashMap<String, String> = HashMap::new(); // in communication with Bob

    let mut alice_hash = String::new();
    let mut bob_hash = String::new();

    let prime: BigInt = BigInt::parse_bytes(&PRIME[..].as_bytes(), 16).unwrap();

    let mut attacker_priv_key: BigInt = BigInt::from(1);
    println!("Attacker private key: {}", attacker_priv_key);

    loop {
        alice_hash =
            get_hashed_shared_key(&pub_key_alice, &attacker_priv_key, &prime)[..PREFIX].to_string();
        bob_hash =
            get_hashed_shared_key(&pub_key_bob, &attacker_priv_key, &prime)[..PREFIX].to_string();

        if priv_key_by_bob_hash.contains_key(&alice_hash) {
            println!(
                "Private key for Alice: {}",
                attacker_priv_key.to_str_radix(16).to_lowercase()
            );
            println!(
                "Private key for Bob: {}",
                priv_key_by_bob_hash.get(&alice_hash).unwrap()
            );
            // println!("Alice hash: {}", alice_hash);
            // println!("Bob hash: {}", bob_hash);
            // println!("Alice map: {:?}", priv_key_by_alice_hash);
            // println!("Bob map: {:?}", priv_key_by_bob_hash);
            break;
        }
        priv_key_by_alice_hash.insert(
            alice_hash.clone(),
            attacker_priv_key.to_str_radix(16).to_lowercase(),
        );

        if priv_key_by_alice_hash.contains_key(&bob_hash) {
            println!(
                "Private key for Alice: {}",
                priv_key_by_alice_hash.get(&bob_hash).unwrap()
            );
            println!(
                "Private key for Bob: {}",
                attacker_priv_key.to_str_radix(16).to_lowercase()
            );
            // println!("Alice hash: {}", alice_hash);
            // println!("Bob hash: {}", bob_hash);
            // println!("Alice map: {:?}", priv_key_by_alice_hash);
            // println!("Bob map: {:?}", priv_key_by_bob_hash);
            break;
        }
        priv_key_by_bob_hash.insert(
            bob_hash.clone(),
            attacker_priv_key.to_str_radix(16).to_lowercase(),
        );

        attacker_priv_key = attacker_priv_key.add(1 as i32);
        // println!(
        //     "Updated private key: {}",
        //     attacker_priv_key.to_str_radix(16).to_lowercase()
        // );
        // println!("Alice hash: {}", alice_hash);
        // println!("Bob hash: {}", bob_hash);
        // println!("Alice map: {:?}", priv_key_by_alice_hash);
        // println!("Bob map: {:?}", priv_key_by_bob_hash);
        // break;
    }
}

// a^p mod m
fn get_hashed_shared_key(a: &BigInt, p: &BigInt, m: &BigInt) -> String {
    let mut hasher = Sha3_224::new();
    let shared_key = &a.modpow(&p, &m);
    // println!("Shared key: {}", &shared_key.to_str_radix(16));
    hasher.update(&shared_key.to_str_radix(16).to_lowercase());
    let result = hasher.finalize();
    // println!("Hashed shared key: {:x}", result);
    format!("{:x}", result)
}
