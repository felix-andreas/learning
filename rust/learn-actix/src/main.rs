use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!\n")
}

#[post("/")]
async fn echo(request_body: String) -> impl Responder {
    HttpResponse::Ok().body(request_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
