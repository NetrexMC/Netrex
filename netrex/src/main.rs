pub mod config;
pub mod log;
pub mod network;
pub mod player;
pub mod plugin;
pub mod server;
pub mod util;

use crate::log::Loggable;

#[tokio::main]
async fn main() {
    let logger = log::colored_logger::ColoredLogger::default();

    logger.info("Starting Netrex...");
    logger.info("Loading config...");
    logger.error("Failed to load config!");
}
