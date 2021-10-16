/*================================================================*\
** Butterscotch | Copyright 2021 NotVeryMoe, projects@notvery.moe **
\*================================================================*/

use chrono::Timelike;

mod game;

fn main() {
    setup_logger().unwrap();

    game::run();
}

fn setup_logger() -> Result<(), fern::InitError> {
    #[cfg(debug_assertions)]
    std::fs::remove_dir_all("logs")?;

    std::fs::create_dir_all("logs")?;

    fern::Dispatch::new()
        .format(format_log_message)
        .chain(fern::Dispatch::new()
            .level(log::LevelFilter::Debug)
            .filter(|metadata| !metadata.target().starts_with("wgpu"))
            .chain(std::io::stdout())
        ).chain(fern::Dispatch::new()
            .level(log::LevelFilter::Debug)
            .chain(fern::log_file(chrono::Local::now().format("logs/%y-%m-%d_%H-%M-%S-%3f.log").to_string())?)
        ).apply()?;

    Ok(())
}

fn format_log_message(out: fern::FormatCallback, message: &core::fmt::Arguments, record: &log::Record) {
    let now = chrono::Local::now();
    out.finish(format_args!(
        "{:05}.{:03} | {} | {}: {}",
        now.num_seconds_from_midnight(),
        now.timestamp_subsec_millis() % 1000,
        match record.level() {
            log::Level::Error => "e",
            log::Level::Warn  => "w",
            log::Level::Info  => "i",
            log::Level::Debug => "d",
            log::Level::Trace => "t",
        },
        record.target(),
        message
    ))
}