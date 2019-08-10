use async_log::{instrument, span};
use log::info;

fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || /* get the task id here */ 0)
        .start(log::LevelFilter::Trace)
        .unwrap();
}

fn main() {
    setup_logger();

    span!("level {}", 1, {
        let x = "beep";
        info!("look at this value: {}", x);

        span!("level {}", 2, {
            inner("boop");
        })
    })
}

#[instrument]
fn inner(y: &str) {
    info!("another nice value: {}", y);
}
