use inventory_management::{
    db::{main::create_connection, postal_code::PostalCodeRepositoryPG},
    domain::postal_code::{PostalCode, PostalCodeInDTO, PostalCodeRepository},
};

#[tokio::main]
async fn main() {
    let pool = create_connection().await;
    let repo = PostalCodeRepositoryPG::new(pool);

    let codes = repo.list().await.unwrap();
    println!("{:#?}", codes);

    let codes = repo.paginate(10, 5).await.unwrap();
    println!("{:#?}", codes);

    let mut code = PostalCode::new(PostalCodeInDTO {
        code: String::from("57178"),
        neighborhood: String::from("Las Armas"),
        category: String::from("Urbano"),
        city: String::from("Nezahualcoyotl"),
        state: String::from("Estado de México"),
    });

    println!("{:#?}", code);

    let _ = repo.create(&mut code).await;

    println!("{:#?}", code);
}
