mod model;
mod routes;
mod services;
use actix_cors::Cors;
use actix_web::web::scope;
use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use routes::generate::generate;

use crate::services::db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let db = Database::init().await;
    let db_data = Data::new(db);

    let server = HttpServer::new(move || {
        App::new().app_data(db_data.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .service(scope("/api").service(generate))
        // .app_data(db_data.clone())
    })
    .bind(("0.0.0.0", 5003))?;

    // Log a message indicating that the server is running
    println!("Server is running on port 5003");

    server.run().await
    // println!("Hello, world!");
}
