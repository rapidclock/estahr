extern crate blake2;
extern crate blake3;
extern crate digest;
extern crate hex;
extern crate md5;
extern crate sha2;
extern crate sha3;

use std::{fs, io};

use blake2::{Blake2b, Blake2s};
use digest::Digest;
use md5::Md5;
use sha2::{Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};

#[derive(Debug)]
pub enum HashAlgorithm {
    MD5,
    SHA2_256,
    SHA2_512,
    SHA3_256,
    SHA3_512,
    BLAKE2S,
    BLAKE2B,
    BLAKE3,
}

fn hash_bytes<D: Digest>(input: &[u8], as_hex: bool) -> String {
    let mut hasher = D::new();
    hasher.input(input);
    let result = hasher.result();
    if as_hex {
        hex::encode(result.as_ref())
    } else {
        String::from_utf8_lossy(result.as_ref()).to_string()
    }
}

fn blake3_bytes(input: &[u8], as_hex: bool) -> String {
    let hash = blake3::hash(input);
    if as_hex {
        hash.to_hex().to_string()
    } else {
        String::from_utf8_lossy(hash.as_bytes()).to_string()
    }
}

pub fn hash_string(algo: HashAlgorithm, input: &str, as_hex: bool) -> String {
    match algo {
        HashAlgorithm::MD5 => hash_bytes::<Md5>(input.as_bytes(), as_hex),
        HashAlgorithm::SHA2_256 => hash_bytes::<Sha256>(input.as_bytes(), as_hex),
        HashAlgorithm::SHA2_512 => hash_bytes::<Sha512>(input.as_bytes(), as_hex),
        HashAlgorithm::SHA3_256 => hash_bytes::<Sha3_256>(input.as_bytes(), as_hex),
        HashAlgorithm::SHA3_512 => hash_bytes::<Sha3_512>(input.as_bytes(), as_hex),
        HashAlgorithm::BLAKE2S => hash_bytes::<Blake2s>(input.as_bytes(), as_hex),
        HashAlgorithm::BLAKE2B => hash_bytes::<Blake2b>(input.as_bytes(), as_hex),
        HashAlgorithm::BLAKE3 => blake3_bytes(input.as_bytes(), as_hex),
    }
}

fn hash_stream<D: Digest + io::Write>(mut stream: &mut dyn io::Read, as_hex: bool) -> Result<String, io::Error> {
    let mut hasher = D::new();
    io::copy(&mut stream, &mut hasher)?;
    let result = hasher.result();
    if as_hex {
        Ok(hex::encode(result.as_ref()))
    } else {
        Ok(String::from_utf8_lossy(result.as_ref()).to_string())
    }
}

fn blake3_stream(mut stream: &mut dyn io::Read, as_hex: bool) -> Result<String, io::Error> {
    let mut hasher = blake3::Hasher::new();
    io::copy(&mut stream, &mut hasher)?;
    let result = hasher.finalize();
    if as_hex {
        Ok(result.to_hex().to_string())
    } else {
        Ok(String::from_utf8_lossy(result.as_bytes()).to_string())
    }
}

pub fn hash_file(algo: HashAlgorithm, path: &str, as_hex: bool) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    match algo {
        HashAlgorithm::MD5 => hash_stream::<Md5>(&mut file, as_hex),
        HashAlgorithm::SHA2_256 => hash_stream::<Sha256>(&mut file, as_hex),
        HashAlgorithm::SHA2_512 => hash_stream::<Sha512>(&mut file, as_hex),
        HashAlgorithm::SHA3_256 => hash_stream::<Sha3_256>(&mut file, as_hex),
        HashAlgorithm::SHA3_512 => hash_stream::<Sha3_512>(&mut file, as_hex),
        HashAlgorithm::BLAKE2S => hash_stream::<Blake2s>(&mut file, as_hex),
        HashAlgorithm::BLAKE2B => hash_stream::<Blake2b>(&mut file, as_hex),
        HashAlgorithm::BLAKE3 => blake3_stream(&mut file, as_hex),
    }
}
