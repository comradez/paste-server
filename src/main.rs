mod errors;
mod services;
mod utils;

use actix_files::Files;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use services::*;
use utils::Pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Unable to locate database");
    let file_cache_dir = std::env::var("FILECACHE_DIR").expect("Unable to locate filecache url");
    let database = Pool::builder()
        .max_size(8)
        .build(redis::Client::open(database_url).expect("Unable to connect to redis"))
        .expect("Unable to open database");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(database.clone()))
            .app_data(Data::new(file_cache_dir.clone()))
            .service(get_message)
            .service(post_message)
            .service(delete_message)
            .service(
                web::scope("/v2")
                    .service(
                        web::resource("/")
                            .app_data(web::PayloadConfig::default().limit(1024 * 1024 * 100))
                            .route(web::post().to(upload_file)),
                    )
                    .service(delete_file)
                    .service(Files::new("", file_cache_dir.as_str())),
            )
    })
    .bind("127.0.0.1:7230")?
    .run()
    .await
}
