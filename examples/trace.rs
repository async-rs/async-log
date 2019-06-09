fn setup_logger() {
    env_logger::Builder::new()
        .filter(None, log::Level::Trace.to_level_filter())
        .try_init()
        .unwrap();
}

fn main() {
    setup_logger();

    async_log::span!("main", {
        let x = "foo";
        log::info!("this {}", x);

        async_log::span!("inner", "x={}", x, {
            log::info!("we must go deeper {}", x);
        });
    })
}
