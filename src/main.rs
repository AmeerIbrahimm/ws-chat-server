use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

mod server;

use server::MyWebSocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(hello))
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn echo_ws(req: HttpRequest, stream:  web::Payload) -> impl Responder {
    ws::start(MyWebSocket::new(), &req, stream)
}
