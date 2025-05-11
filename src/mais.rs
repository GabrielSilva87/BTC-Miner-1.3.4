use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Digest, Sha256};
use ripemd160::Ripemd160;
use base58::ToBase58;

fn private_key_to_wif(private_key: &[u8]) -> String {
    let mut extended_key = Vec::with_capacity(private_key.len() + 2);
    extended_key.push(0x80);
    extended_key.extend_from_slice(private_key);
    let checksum = double_sha256(&extended_key)[0..4].to_vec();
    extended_key.extend_from_slice(&checksum);
    extended_key.to_base58()
}

fn double_sha256(data: &[u8]) -> Vec<u8> {
    let hash1 = Sha256::digest(data);
    let hash2 = Sha256::digest(&hash1);
    hash2.to_vec()
}

fn public_key_to_p2pkh_address(public_key: &PublicKey) -> String {
    let pubkey_serialized = public_key.serialize();
    let sha256_hash = Sha256::digest(&pubkey_serialized);
    let ripemd_hash = Ripemd160::digest(&sha256_hash);
    let mut extended_ripemd = Vec::with_capacity(ripemd_hash.len() + 1);
    extended_ripemd.push(0x00);
    extended_ripemd.extend_from_slice(&ripemd_hash);
    let checksum = double_sha256(&extended_ripemd)[0..4].to_vec();
    extended_ripemd.extend_from_slice(&checksum);
    extended_ripemd.to_base58()
}

fn main() {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
    let wif = private_key_to_wif(&secret_key[..]);
    println!("Chave privada (WIF): {}", wif);
    let address = public_key_to_p2pkh_address(&public_key);
    println!("Endereço Bitcoin (P2PKH): {}", address);
}
