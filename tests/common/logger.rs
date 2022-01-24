use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

const LOGFILE: &'static str = "logs/test.log";

pub fn init_logger() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{l}|{d}|{f}:{L}] - {m}\n")))
        .build(LOGFILE)
        .expect("Failed to find logfile.");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .expect("Failed to init log config.");

    log4rs::init_config(config)
        .expect("Failed to init logger.");
}
