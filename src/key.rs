use serde::Deserialize;

#[derive(Deserialize)]
pub struct KeyCloakKey {
    #[serde(rename = "realm")]
    pub realm: String,
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "token-service")]
    pub token_service: String,
    #[serde(rename = "account-service")]
    pub account_service: String,
    #[serde(rename = "tokens-not-before")]
    pub tokens_not_before: u32
}