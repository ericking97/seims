use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use std::env;
use std::io;
use std::result::Result;

const DATABASE: &str = "INVENTORY_MANAGEMENT_DB";

#[tokio::main]
async fn main() {
  let database_uri = match env::var(DATABASE) {
    Ok(v) => v,
    Err(e) => panic!("${} is not set ({})", DATABASE, e),
  };

  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_uri)
    .await;

  let pool = match pool {
    Ok(p) => p,
    Err(err) => panic!("Problem creating connection with postgresql {:?}", err),
  };

  let repo = PostalCodeRepositoryPG { pool: pool };

  let codes = repo.list().await.unwrap();
  println!("Codigos: {:?}", codes);

  let codes = repo.paginate(10, 0).await.unwrap();
  println!("{:?}", codes)
}

// async fn create_pool() -> sqlx::Pool<sqlx::Postgres> {
//   let pool = sqlx::postgres::PgPoolOptions::new()
//     .max_connections(5)
//     .connect(DATABASE)
//     .await;

//   let pool = match pool {
//     Ok(p) => p,
//     Err(err) => panic!("Problem creating connection with postgresql {:?}", err),
//   };

//   pool
// }

#[derive(Debug, FromRow)]
pub struct PostalCode {
  id: i32,
  code: String,
  neighborhood: String,
  category: String,
  city: String,
  state: String,
}

impl ToString for PostalCode {
  fn to_string(&self) -> String {
    let str = format!(
      "Info: {} {} {} {}",
      self.code, self.neighborhood, self.city, self.state
    );
    str
  }
}

#[async_trait::async_trait]
pub trait PostalCodeRepository {
  async fn list(&self) -> Result<Vec<PostalCode>, io::Error>;
  async fn paginate(&self, limit: i64, offset: i64) -> Result<Vec<PostalCode>, io::Error>;
}

struct PostalCodeRepositoryPG {
  pool: sqlx::Pool<sqlx::Postgres>,
}

#[async_trait::async_trait]
impl PostalCodeRepository for PostalCodeRepositoryPG {
  // Ways to convert to objects
  async fn list(&self) -> Result<Vec<PostalCode>, io::Error> {
    let query = "SELECT * FROM postal_code";

    // 1. Explicitly construct the object
    let rows: Vec<PostalCode> = sqlx::query(query)
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

  async fn paginate(&self, limit: i64, offset: i64) -> Result<Vec<PostalCode>, io::Error> {
    let query = "SELECT * FROM postal_code LIMIT ($1) OFFSET ($2)";

    // 2. Select query_as (using derive FromRow)
    let rows: Vec<PostalCode> = sqlx::query_as::<_, PostalCode>(query)
      .bind(limit)
      .bind(offset)
      .fetch_all(&self.pool)
      .await
      .unwrap();

    Ok(rows)
  }
}
