mod handlers;
mod models;
mod config;
mod response;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::{get_acticles, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    println!(
        "Starting server at http://{}:{}/",
        config.server.host, config.server.port
    );

    let app = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/articles", web::get().to(get_acticles))
    });

    app.bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
