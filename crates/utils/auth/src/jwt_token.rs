pub mod jwt {
    use error_mapper::errors::AerodisSpaceError;
    pub use jsonwebtoken;
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

    pub fn generate_jwt_token<T>(salt_secret: &str, claim_struch: T) -> Result<String, AerodisSpaceError>
    where
        T: serde::Serialize,
    {
        encode(&Header::default(), &claim_struch, &EncodingKey::from_secret(salt_secret.as_ref()))
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))
    }

    pub fn verify_jwt_token<T>(salt_secret: &str, token: &str) -> Result<TokenData<T>, AerodisSpaceError>
    where
        T: serde::de::DeserializeOwned,
    {
        decode::<T>(token, &DecodingKey::from_secret(salt_secret.as_ref()), &Validation::default())
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))
    }
}
