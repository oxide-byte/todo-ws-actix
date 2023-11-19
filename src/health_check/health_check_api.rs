use actix_web::{HttpResponse, Responder};

use crate::health_check::health_check::HealthCheck;

pub async fn get_health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheck::ok())
}