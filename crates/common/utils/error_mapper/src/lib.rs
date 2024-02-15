pub mod errors {
    /// Used for map errors to a custom error type
    /// Used for identify the error type to the client and the server
    /// Set the type error and describe the error for the client (ONLY INFORMATION ERROR, NO SERVER(SENSITIVE) INFORMATION)
    #[derive(Debug)]
    pub enum AerodisSpaceError {
        Internal(String),
        BadRequest(String),
        NotFound(String),
        InvalidField(String),
        AlreadyExists(String),
        Auth(String),
    }

    impl std::fmt::Display for AerodisSpaceError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                AerodisSpaceError::Internal(ref _err) => write!(f, "Internal Error: {}", _err),
                AerodisSpaceError::BadRequest(ref _err) => write!(f, "Bad Request: {}", _err),
                AerodisSpaceError::NotFound(ref _err) => write!(f, "Not Found: {}", _err),
                AerodisSpaceError::InvalidField(ref _err) => write!(f, "Invalid Field: {}", _err),
                AerodisSpaceError::AlreadyExists(ref _err) => write!(f, "Already Exists: {}", _err),
                AerodisSpaceError::Auth(ref _err) => write!(f, "Auth Error: {}", _err),
            }
        }
    }

    impl std::error::Error for AerodisSpaceError {}
}
