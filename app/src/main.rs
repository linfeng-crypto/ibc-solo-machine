use app::app::App;
use iced::{Application, Settings};
use log::LevelFilter;

fn main() -> iced::Result {
    let _ = dotenv::dotenv();
    let log_level = LevelFilter::Debug;
    let log_file = "/tmp/log/debug.log";
    let mut config = fern::Dispatch::new();
    let filter_targets = vec![
        "wgpu_core",
        "wgpu",
        "gfx_backend_metal",
        "naga",
        "tracing",
        "iced_wgpu",
    ];
    for f in filter_targets {
        config = config.level_for(f, LevelFilter::Off);
    }
    config
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(fern::log_file(log_file).unwrap())
        .apply()
        .unwrap();
    App::run(Settings::default())
}
