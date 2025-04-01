use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
use std::sync::Once;

static LOGGING_STATE: Once = Once::new();

pub fn setup_logging() {
    LOGGING_STATE.call_once(|| {
        let logger = TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        );
        CombinedLogger::init(vec![logger]).unwrap();
    });
}
