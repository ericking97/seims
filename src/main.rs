use inventory_management::{
    db::{main::create_connection, postal_code::PostalCodeRepositoryPG},
    domain::postal_code::PostalCodeRepository,
};

#[tokio::main]
async fn main() {
    let pool = create_connection().await;
    let repo = PostalCodeRepositoryPG::new(pool);

    let codes = repo.list().await.unwrap();
    println!("{:#?}", codes);

    let codes = repo.paginate(10, 5).await.unwrap();
    println!("{:#?}", codes);
}
