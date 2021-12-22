use std::io::Error;

use crate::domain::postal_code::{PostalCode, PostalCodeRepository, PostalCodes};
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};

// # Postal Code Model
pub struct PostalCodeRepositoryPG {
    pool: Pool<Postgres>,
}

impl PostalCodeRepositoryPG {
    pub fn new(p: Pool<sqlx::Postgres>) -> Self {
        PostalCodeRepositoryPG { pool: p }
    }
}

#[async_trait]
impl PostalCodeRepository for PostalCodeRepositoryPG {
    async fn list(&self) -> Result<PostalCodes, Error> {
        let query = "SELECT * FROM postal_code";

        // 1. Explicitly construct the object
        let rows: PostalCodes = sqlx::query(query)
            .map(|row: PgRow| PostalCode {
                id: row.get("id"),
                code: row.get("code"),
                neighborhood: row.get("neighborhood"),
                category: row.get("category"),
                city: row.get("city"),
                state: row.get("state"),
            })
            .fetch_all(&self.pool)
            .await
            .unwrap();

        Ok(rows)
    }

    async fn paginate(&self, limit: i64, offset: i64) -> Result<PostalCodes, Error> {
        let query = "SELECT * FROM postal_code LIMIT ($1) OFFSET ($2)";

        // 2. Select query_as (using derive FromRow)
        let rows: PostalCodes = sqlx::query_as::<_, PostalCode>(query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .unwrap();

        Ok(rows)
    }
}
