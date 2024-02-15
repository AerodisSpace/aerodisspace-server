use database::scylla::CachingSession;
use error_mapper::errors::AerodisSpaceError;

/// Repository trait that defines the methods that a repository should implement
#[async_trait::async_trait]
pub trait Repository<T> {
    /// Get one item from the repository
    async fn get_one(field: String, session: &CachingSession) -> Result<T, AerodisSpaceError>;
    /// Get all items from the repository with a specific field
    async fn get_all(field: Option<String>, session: &CachingSession) -> Result<Vec<T>, AerodisSpaceError>;
    /// Create a new item in the repository
    async fn create(data: &mut T, session: &CachingSession) -> Result<T, AerodisSpaceError>;
    /// Update an item in the repository
    async fn update(data: T, session: &CachingSession) -> Result<T, AerodisSpaceError>;
    /// Delete an item from the repository
    async fn delete(field: String, session: &CachingSession) -> Result<(), AerodisSpaceError>;
}
