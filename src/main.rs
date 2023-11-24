use actix_cors::Cors;
use actix_web::{App, http::header, HttpServer};
use actix_web::middleware::Logger;
use awc::Client;
use crate::key::KeyCloakKey;

mod health_check;
mod todo;
mod routes;
pub mod context;
mod key;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis log init fail");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let client = Client::new();

    let response = client
        .get("http://127.0.0.1:8888/realms/learn-rust-realm/")    // <- Create request builder
        .insert_header(("User-Agent", "Actix-web"))
        .send()                             // <- Send http request
        .await;

    let key_cloak = response.unwrap().json::<KeyCloakKey>().await.unwrap();

    println!("🚀 Server started successfully: http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()

            .configure(|conf| routes::routes(conf, key_cloak.public_key.clone()))
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}