use crate::{ansii_aqua, ansii_gray, ansii_red, ansii_yellow};

use super::{logger::Logger, Loggable};
use super::{PREFIX_DEBUG, PREFIX_ERROR, PREFIX_INFO, PREFIX_WARN};

/// Logger that prints colored messages to the console.
/// This logger uses any logger passed to it, otherwise it uses the default logger.
pub struct ColoredLogger {
    logger: Box<dyn Loggable>,
}

impl ColoredLogger {
    pub fn new() -> Self {
        Self {
            logger: Box::new(Logger::default()),
        }
    }

    pub fn with_logger(logger: Box<dyn Loggable>) -> Self {
        Self { logger }
    }
}

impl Default for ColoredLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Loggable for ColoredLogger {
    fn log(&self, message: &str) {
        self.logger.log(message);
    }

    // fn debug(&self, message: &str) {
    // 	self.logger.log(format!("{} {}", ansii_dark_gray!(PREFIX_DEBUG), message).as_str());
    // }

    // fn warn(&self, message: &str) {
    // 	self.logger.log(format!("{} {}", ansii_yellow!(PREFIX_WARN), message).as_str());
    // }

    // fn info(&self, message: &str) {
    // 	self.logger.log(format!("{} {}", ansii_aqua!(PREFIX_INFO), message).as_str());
    // }

    // fn error(&self, message: &str) {
    // 	self.logger.log(format!("{} {}", ansii_red!(PREFIX_INFO), message).as_str());
    // }

    fn debug(&self, message: &str) {
        self.logger
            .log(ansii_gray!(format!("{} {}", PREFIX_DEBUG, message).as_str()).as_str());
    }

    fn warn(&self, message: &str) {
        self.logger
            .log(ansii_yellow!(format!("{} {}", PREFIX_WARN, message).as_str()).as_str());
    }

    fn info(&self, message: &str) {
        self.logger
            .log(ansii_aqua!(format!("{} {}", PREFIX_INFO, message).as_str()).as_str());
    }

    fn error(&self, message: &str) {
        self.logger
            .log(ansii_red!(format!("{} {}", PREFIX_ERROR, message).as_str()).as_str());
    }
}
