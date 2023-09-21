use std::env;

use actix_files::Files;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use anyhow::Result;
use example_chat::component;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let address = env::var("ADDRESS")?;
    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .default_service(web::to(component::default))
            .service(Files::new("/public", "./public").prefer_utf8(true))
            .service(component::index)
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}
