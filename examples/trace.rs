use async_log::span;
use log::info;

fn setup_logger() {
    let logger = env_logger::Builder::new()
        .filter(None, log::LevelFilter::Trace)
        .build();

    async_log::Logger::wrap(logger, || 12)
        .start(log::LevelFilter::Trace)
        .unwrap();
}

fn main() {
    setup_logger();

    span!("main", {
        let x = "foo";
        info!("this {}", x);

        span!("inner, x={}", x, {
            info!("we must go deeper {}", x);
        })
    })
}
