use super::requests::{
    create_wallet, get_balance_of_address, mine_pending_transactions, new_transaction,
    show_blockchain,
};
use actix_web::{App, HttpServer};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(new_transaction)
            .service(mine_pending_transactions)
            .service(show_blockchain)
            .service(get_balance_of_address)
            .service(create_wallet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
