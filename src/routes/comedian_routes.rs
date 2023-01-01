use actix_web::{get, HttpResponse, Responder};

#[get("/get_comedian")]
async fn get_data() -> impl Responder {
    HttpResponse::Ok().body("hello!")
}
