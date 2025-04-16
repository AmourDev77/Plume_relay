use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to this api, please feel free to visit Github Wiki to get a reference")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))? // Bind address and port of the api, once in production the url will be replaced by 0.0.0.0 to enable requests from all origins
    .run()
    .await
}
