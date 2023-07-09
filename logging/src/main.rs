use env_logger::Builder;
use log::debug;
use log::error;
use log::info;
use log::warn;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::env;

fn main() {
    // LOG4RS=1 cargo run
    match env::var("LOG4RS") {
        Ok(_) => run_log4rs(),
        Err(_) => run_env_logger(),
    };
}

fn run_env_logger() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    debug!("Hello from env_logger.debug.");
    info!("{:?}", "Hello from env_logger.info.");
    warn!("{:#?}", "Hello from env_logger.warn.");
    error!("{}", "Hello from env_logger.error.");
}

fn run_log4rs() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    debug!("Hello from log4rs.debug.");
    info!("{:?}", "Hello from log4rs.info.");
    warn!("{:#?}", "Hello from log4rs.warn.");
    error!("{}", "Hello from log4rs.error.");
}