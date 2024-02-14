extern crate bcrypt;
use bcrypt::{hash, verify, DEFAULT_COST};
use error_mapper::errors::AerodisSpaceError;

pub fn generate_hash(str: &String) -> Result<String, AerodisSpaceError> {
    let hash = hash(str, DEFAULT_COST).map_err(|_err| AerodisSpaceError::Auth(_err.to_string()))?;
    Ok(hash)
}

pub fn verify_hash(str: &String, hash: &String) -> Result<bool, AerodisSpaceError> {
    let result = verify(str, &hash).map_err(|_err| AerodisSpaceError::Auth(_err.to_string()))?;
    Ok(result)
}
