use actix_web::web;
use actix_web::web::{delete, get, post, put};
use actix_web_middleware_keycloak_auth::{ DecodingKey, KeycloakAuth };
use crate::health_check::health_check_api::get_health_check;
use crate::todo::todo_api::{create_todo, delete_todo, get_page_todo, get_todo_by_id, get_todo_list, update_todo};

// FROM KEYCLOAK check for correct <<kid>>
const KEYCLOAK_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIICrzCCAZcCBgGL6UCfsTANBgkqhkiG9w0BAQsFADAbMRkwFwYDVQQDDBBsZWFybi1ydXN0LXJlYWxtMB4XDTIzMTExOTIwMjE0OFoXDTMzMTExOTIwMjMyOFowGzEZMBcGA1UEAwwQbGVhcm4tcnVzdC1yZWFsbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAJNBrgNu0acHZlSz+3z1uKZKXZ3SM2/TGvq1erNtFJKtjASY+WfH62jl+aBWT0OtOtTMqMLCrYi6yrWH7COG73VZ29HefFrWGbuaFwdeev84qHTT6QWp/5Xp5o4HYU/lab4Vvwvjs5Al+oUXKR3ntZCwlfn/sLs1U4uUyvgZmegxGfZjv8zZZFrFIKZJotdHGvOkc2r473Hnq7soL8mNXztOfyqNwseIBGP0sAv8X4nC8y+Wt+5iupZ3Iw/jJzthUXEIzUCyI2i7nXpeEGb63h0cEGPxy2w12I2J0D3W9dC18FnIKA/y3XokIvPDSutRb2ZP3FDhXvs7c+f8bWsusD0CAwEAATANBgkqhkiG9w0BAQsFAAOCAQEAMnAxGpF5UY8sZmWCHoKZTXOtU36gmEnA9lVYrPTFdM4WUDORchSYunlYK+34FlxTr/OfiN43d2BTXMy+ajxlk29vIcSK8Gkpqe9NzqtXojv8Jfv8/hIxfFHzYqj9sr5fYv9gn0S9ruP9rUrgvYoxF3eXoZOcMi7BMxok/Z5iWiYpFjMms+QlKrbfEXFq2JuZtQjk2Ppyak+Thz1Ao6LHgjZHXqpVOeJmzOPxhFeb7dle/4KRcl3174X/WQq+b4ZTmjF5+7preCQNPFMBb8I0PCgP2QEyKe2xW8jvenXEBmW4pEFxWJaqOjYcpF7WwK71AVEGbcUMjq/Kyrefsvu67w==
-----END PUBLIC KEY-----";
pub fn routes(cfg: &mut web::ServiceConfig) {

    let keycloak_auth = KeycloakAuth::default_with_pk(DecodingKey::from_rsa_pem(KEYCLOAK_PUBLIC_KEY.as_bytes()).unwrap());

    /****
    let keycloak_auth = KeycloakAuth {
        detailed_responses: true,
        passthrough_policy: AlwaysReturnPolicy,
        keycloak_oid_public_key: DecodingKey::from_rsa_pem(KEYCLOAK_PUBLIC_KEY.as_bytes()).unwrap(),
        required_roles: vec![Role::Realm {
            role: "test".to_owned(),
        }],
    };
    ****/

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