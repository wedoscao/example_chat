use std::env;

use actix_files::Files;
use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};
use anyhow::Result;
use example_chat::component;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let address = env::var("ADDRESS")?;
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/public", "./public").prefer_utf8(true).show_files_listing())
            .service(component::index)
            .wrap(Compress::default())
            .wrap(Logger::default())
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}
