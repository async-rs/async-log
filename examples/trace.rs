use async_log::span;
use log::info;

fn setup_logger() {
    env_logger::Builder::new()
        .filter(None, log::Level::Trace.to_level_filter())
        .try_init()
        .unwrap();
}

fn main() {
    setup_logger();

    span!("main", {
        let x = "foo";
        info!("this {}", x);

        span!("inner, x={}", x, {
            info!("we must go deeper {}", x);
        });
    })
}
