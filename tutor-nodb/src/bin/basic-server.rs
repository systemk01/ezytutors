//Module imports
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

//configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/bubi", web::get().to(bubi_handler));
}

//configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

pub async fn bubi_handler() -> impl Responder {
   HttpResponse::Ok().json("Hello this is Bubi Info page")
}
//instantiate and run the HTTP server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    //construct App and configure routes
    let app = move || App::new().configure(general_routes);
    //start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}