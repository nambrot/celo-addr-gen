use bip39::{Mnemonic, Language, Seed};
extern crate hdwallet;
extern crate eth_checksum;

use hdwallet::{KeyChain, ExtendedPrivKey, ExtendedPubKey};
use hdwallet::{
    traits::{Serialize},
};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

use std::{fmt::Write};
use std::fs::File;
use std::io::prelude::*;
use std::env;

const CELO_DERIVATION_PATH: &str = "m/44'/52752'/0'/0/0";

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b);
    }
    s
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        panic!("Wrong number of arguments given")
    }
    let mut output_address = false;
    let mut output_private_key = false;
    if args.len() == 3 {
        if args[2] != "--output-public-key" && args[2] != "--output-private-key" {
            panic!("unknown 2nd argument")
        }

        if args[2] != "--output-public-key" {
            output_address = true;
        }

        if args[2] != "--output-private-key" {
            output_private_key = true;
        }
    }

    let mut file = File::open(&args[1]).expect("read file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let words: Vec<&str> = contents.split("\n").collect();
    let mnemonic_words: Vec<String> = words.iter().take(24).map(|x| x.to_string() ).collect();
    let phrase = mnemonic_words.join(" ");
    let mnemonic = Mnemonic::from_phrase(&phrase, Language::English).expect("Load mnemonic");
    let mut passphrase = "";
    if words.len() > 24 {
        passphrase = words[24];
    }
    let seed = Seed::new(&mnemonic, passphrase);
    let private_key = ExtendedPrivKey::with_seed(seed.as_bytes()).unwrap();
    let key_chain = hdwallet::DefaultKeyChain::new(private_key);
    let (child_key, _d) = key_chain.derive_private_key(CELO_DERIVATION_PATH.into()).unwrap();

    if output_private_key {
        println!("0x{}", &encode_hex(&child_key.serialize()));
        return
    }

    let pubchild_key = ExtendedPubKey::from_private_key(&child_key).public_key.serialize_uncompressed();

    if output_address {
        println!("0x{}", &encode_hex(&pubchild_key));
        return
    }

    let mut hasher = Sha3::keccak256();
    hasher.input(&pubchild_key[1..]);
    let mut address = "0x".to_string();
    address.push_str(&hasher.result_str()[24..]);

    println!("{}", &eth_checksum::checksum(&address));
}
