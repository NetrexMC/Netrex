#[macro_use]
pub mod colored_logger;
pub mod logger;

pub const PREFIX_DEBUG: &str = "[DEBUG]";
pub const PREFIX_WARN: &str = "[WARN]";
pub const PREFIX_INFO: &str = "[INFO]";
pub const PREFIX_ERROR: &str = "[ERROR]";
pub const PREFIX_FATAL: &str = "[FATAL]";

pub trait Loggable {
    fn debug(&self, message: &str) {
        self.log(format!("{} {}", PREFIX_DEBUG, message).as_str());
    }

    fn warn(&self, message: &str) {
        self.log(format!("{} {}", PREFIX_WARN, message).as_str());
    }

    /// This is the main logging function.
    /// This is the only one that needs to be implemented.
    /// DO NOT PREFIX THIS FUNCTION WITH ANYTHING.
    fn log(&self, message: &str);

    fn info(&self, message: &str) {
        self.log(format!("{} {}", PREFIX_INFO, message).as_str());
    }

    fn error(&self, message: &str) {
        self.log(format!("{} {}", PREFIX_ERROR, message).as_str());
    }

    fn fatal(&self, message: &str) {
        self.log(format!("{} {}", PREFIX_FATAL, message).as_str());
    }
}
