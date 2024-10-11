use actix_files as fs;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/", "./static/html").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
