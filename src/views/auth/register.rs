use actix_web::{Responder, HttpResponse};

pub async fn register() -> impl Responder {
    return HttpResponse::Ok()
}