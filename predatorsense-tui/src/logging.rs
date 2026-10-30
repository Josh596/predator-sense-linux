use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use predatorsense::store;

const LOG_FILE: &str = "predatorsense.log";

/// Logs to `predatorsense.log` in the state directory, next to the lighting
/// profile. Our own crates log at debug, everything else at warn.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let path = store::state_dir()?.join(LOG_FILE);

    // Creates the state directory if it does not exist yet.
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
        .build(path)?;

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(Logger::builder().build("predatorsense", LevelFilter::Debug))
        .build(Root::builder().appender("file").build(LevelFilter::Warn))?;

    log4rs::init_config(config)?;

    Ok(())
}
