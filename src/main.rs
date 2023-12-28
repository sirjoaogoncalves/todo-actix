mod models;
mod config;

use crate::models::Status;
use actix_web::{App, HttpServer, web, Responder};
use std::io;
use dotenv::dotenv;

async fn status() -> impl Responder {
    actix_web::HttpResponse::Ok().json(Status { status: "OK".to_string() }) 
}


#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    println!("Starting server on http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(|| {
       App::new()
           .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
