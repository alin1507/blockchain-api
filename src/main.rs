mod api;
mod blockchain;
use actix_web::{App, HttpServer};
use api::requests::{
    create_wallet, get_wallet_balance, get_wallet_transactions, mine_pending_transactions,
    create_transaction, show_blockchain,
};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_transaction)
            .service(mine_pending_transactions)
            .service(show_blockchain)
            .service(get_wallet_balance)
            .service(create_wallet)
            .service(get_wallet_transactions)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
