use bcrypt::{hash, verify};
use eyre::Result;

const COST: u32 = 12;

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, COST)?)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    Ok(verify(password, hash)?)
}
