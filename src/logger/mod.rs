use log::*;
use simplelog::*;
use std::fs::OpenOptions;

#[derive(Clone)]
pub struct Logger {
    prefix: String,
}

impl Logger {
    pub fn new(prefix: String) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("server.log");

        let config = ConfigBuilder::new().set_time_to_local(true).build();

        CombinedLogger::init(vec![
            TermLogger::new(
                LevelFilter::Info,
                config.clone(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(LevelFilter::Info, config, file.unwrap()),
        ])
        .unwrap();

        Self { prefix }
    }

    pub fn info(&mut self, msg: &str) {
        info!("{}: {}", self.prefix, msg);
    }

    pub fn warn(&mut self, msg: &str) {
        warn!("{}: {}", self.prefix, msg);
    }

    pub fn error(&mut self, msg: &str) {
        error!("{}: {}", self.prefix, msg);
    }

    pub fn trace(&mut self, msg: &str) {
        trace!("{}: {}", self.prefix, msg);
    }

    pub fn debug(&mut self, msg: &str) {
        debug!("{}: {}", self.prefix, msg);
    }
}
