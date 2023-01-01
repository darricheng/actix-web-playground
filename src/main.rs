mod controllers;
mod routes;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use routes::config_routes;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    test(String::from("Bob"));
    HttpServer::new(|| {
        App::new()
            .configure(config_routes)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Debug)]
struct TestStruct {
    name: String,
    age: u8,
}

fn test(some_var: String) {
    println!("{}", some_var);
    let some_struct = TestStruct {
        name: String::from("John Doe"),
        age: 27,
    };
    println!("{some_struct:?}")
}
