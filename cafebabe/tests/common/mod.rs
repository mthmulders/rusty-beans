use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};

pub fn setup_logging() {
    let logger = TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    CombinedLogger::init(vec![logger]).unwrap();
}
