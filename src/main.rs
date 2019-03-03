mod config;
mod error;

use error::*;
use log::*;

use actix_web::{actix::System, server::HttpServer, App};

fn main() -> Result<()> {
    env_logger::init();
    let config = config::read_config()?;
    debug!("Read configuration file: {:?}", config);

    let system = System::new("axochat");

    HttpServer::new(move || App::new())
        .bind(config.net.address)?
        .start();

    info!("Started server at {}", config.net.address);
    system.run();

    Ok(())
}
