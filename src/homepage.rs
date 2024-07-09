use actix_web::{Responder, HttpResponse, get};
//use tracing::{Level, event, instrument};

// Root page of the blog
#[get("/")]
//#[instrument]
pub async fn home() -> impl Responder {
    tracing::info!("HTTP GET ; '/'");
    HttpResponse::Ok().body("Bonjour et bienvenue on my new blog ! :)")
}
