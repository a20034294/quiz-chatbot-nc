use sha2::{Digest, Sha256};
extern crate base64;
extern crate hex;

pub fn get_token(payload: String) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(payload.clone());
    hasher.update(super::config::PRIVATE_KEY);

    // read hash digest and consume hasher
    let result = hasher.finalize();
    let token = (base64::encode(payload) + ".") + base64::encode(result).as_str();

    token
}
pub fn verify_token(token: String) -> bool {
    let v: Vec<&str> = token.split(".").collect();
    if v.len() != 2 {
        return false;
    }
    let (payload, digest) = (v[0], v[1]);
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(base64::decode(payload).unwrap());
    hasher.update(super::config::PRIVATE_KEY);

    // read hash digest and consume hasher
    let result = hasher.finalize();

    if base64::encode(result).as_str() != digest {
        return false;
    }

    true
}
