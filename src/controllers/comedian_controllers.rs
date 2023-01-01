use actix_web::HttpResponse;

pub fn get_comedians() {
    HttpResponse::Ok().body("Comedians okay!");
}
