use actix_web::web;
use actix_web::web::{delete, get, post, put};
use actix_web_middleware_keycloak_auth::{ DecodingKey, KeycloakAuth };
use crate::health_check::health_check_api::get_health_check;
use crate::todo::todo_api::{create_todo, delete_todo, get_page_todo, get_todo_by_id, get_todo_list, update_todo};

pub fn routes(cfg: &mut web::ServiceConfig, key: String) {

    let pub_key = format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", key);

    let keycloak_auth = KeycloakAuth::default_with_pk(DecodingKey::from_rsa_pem(pub_key.as_bytes()).unwrap());

    cfg.service(
        web::scope("/api")
            .service(web::scope("/healthcheck")
                .route("", get().to(get_health_check))
            )
            .service(
                web::scope("/todo")
                    .wrap(keycloak_auth)
                    .route("", get().to(get_todo_list))
                    .route("/params", get().to(get_page_todo))
                    .route("/{id}", get().to(get_todo_by_id))
                    .route("", post().to(create_todo))
                    .route("", put().to(update_todo))
                    .route("/{id}", delete().to(delete_todo)),
            )
    );
}