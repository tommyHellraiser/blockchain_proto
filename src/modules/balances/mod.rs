use actix_web::{get, HttpResponse, HttpResponseBuilder, web};
use actix_web::http::StatusCode;

/// base_path/api/balances
pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(balance_get);
}

#[get("/test")]
async fn balance_get() -> HttpResponse {

    println!("Got to the only endpoint available lmao");

    HttpResponseBuilder::new(StatusCode::OK).json("Hola ke ase")
}
