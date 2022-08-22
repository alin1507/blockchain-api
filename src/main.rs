mod api;
mod blockchain;
use actix_web::{App, HttpServer};
use api::requests::{
    create_transaction, create_wallet, get_wallet_balance, get_wallet_transactions,
    mine_pending_transactions, show_blockchain, add_coins,
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
            .service(add_coins)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
