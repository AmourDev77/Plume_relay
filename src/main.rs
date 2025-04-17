use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to this api, please feel free to visit Github Wiki to get a reference")
}

const BIND_ADDRESS: &str = "127.0.0.1";
const BIND_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on {BIND_ADDRESS}:{BIND_PORT}");
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind((BIND_ADDRESS, BIND_PORT))? // Bind address and port of the api, once in production the url will be replaced by 0.0.0.0 to enable requests from all origins
    .run()
    .await
}
