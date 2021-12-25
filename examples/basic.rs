fn main() {
    let formatter = syslog::Formatter3164::default();
    let sys_logger = syslog::unix(formatter).expect("Failed to init unix socket");

    let syslog_writer = flexi_syslog::Builder::new()
        .max_log_level(log::LevelFilter::Info)
        .build(sys_logger);

    let logger = flexi_logger::Logger::try_with_str("info")
        .expect("Failed to init logger")
        .log_to_writer(Box::new(syslog_writer));

    logger.start().expect("Failed to start logger");

    log::info!("Info gets through");
    log::trace!("Trace is filtered");
}
