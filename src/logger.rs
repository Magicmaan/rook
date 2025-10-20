use ftail::{self, Config};
use std::time::SystemTime;

pub struct Logger {
    config: Config,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.config.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards");

        println!(
            "{} [{}] {}",
            since_epoch.as_secs(),
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {}
}
