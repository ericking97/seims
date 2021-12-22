use async_trait::async_trait;
use sqlx::FromRow;
use std::io;
use std::result::Result;

// # Postal Codes
// Vector of Postal Code structs
pub type PostalCodes = Vec<PostalCode>;

// # Postal Code.
// Useful struct to store any postal code
#[derive(Debug, FromRow)]
pub struct PostalCode {
    pub id: i32,
    pub code: String,
    pub neighborhood: String,
    pub category: String,
    pub city: String,
    pub state: String,
}

// # Postal Code Repository
// Defines the specific requirements for any struct to be considered a valid repository to manipulate postal
// codes.
#[async_trait]
pub trait PostalCodeRepository {
    async fn list(&self) -> Result<PostalCodes, io::Error>;
    async fn paginate(&self, offset: i64, limit: i64) -> Result<PostalCodes, io::Error>;
}
