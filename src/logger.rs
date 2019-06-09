use crate::backtrace::async_log_capture_caller;
use log::{LevelFilter, Log, Metadata, Record};

/// A Logger that wraps other loggers to extend it with async functionality.
#[derive(Debug)]
pub struct Logger<L: Log + 'static, F>
where
    F: Fn() -> (u64, Option<u64>) + Send + Sync + 'static,
{
    logger: L,
    with: F,
    depth: u8,
    filter: LevelFilter,
}

impl<L: Log + 'static, F> Logger<L, F>
where
    F: Fn() -> (u64, Option<u64>) + Send + Sync + 'static,
{
    /// Wrap an existing logger, extending it with async functionality.
    pub fn wrap(logger: L, depth: u8, with: F) -> Self {
        Self {
            filter: LevelFilter::Off,
            logger,
            depth,
            with,
        }
    }

    /// Set the filter level
    pub fn filter(mut self, filter: LevelFilter) -> Self {
        self.filter = filter;
        self
    }

    /// Start logging.
    pub fn start(self) -> Result<(), log::SetLoggerError> {
        log::set_boxed_logger(Box::new(self))
    }

    /// Call the `self.with` closure, and return its results.
    fn with(&self) -> (u64, Option<u64>) {
        (self.with)()
    }
}

impl<L: Log, F> log::Log for Logger<L, F>
where
    F: Fn() -> (u64, Option<u64>) + Send + Sync + 'static,
{
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        self.logger.enabled(metadata)
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let (curr_id, parent_id) = self.with();
            let symbol = async_log_capture_caller(self.depth);

            let task_id = format!("task_id={}", curr_id);
            let parent_id = parent_id
                .map(|pid| format!("task_parent_id={}", pid))
                .unwrap_or_else(|| String::from(""));

            let (line, filename) = match symbol {
                Some(symbol) => {
                    let line = symbol
                        .lineno
                        .map(|l| format!(", line={}", l))
                        .unwrap_or_else(|| String::from(""));

                    let filename = symbol
                        .filename
                        .map(|f| format!(", filename={}", f.to_string_lossy()))
                        .unwrap_or_else(|| String::from(""));

                    (line, filename)
                }
                None => (String::from(""), String::from("")),
            };

            // This is done this way b/c `Record` + `format_args` needs to be built inline. See:
            // https://stackoverflow.com/q/56304313/1541707
            self.logger.log(
                &log::Record::builder()
                    .args(record.args().clone())
                    .args(format_args!(
                        "{}{}{}{}{}",
                        record.args(),
                        line,
                        filename,
                        task_id,
                        parent_id
                    ))
                    .metadata(record.metadata().clone())
                    .level(record.level())
                    .target(record.target())
                    .module_path(record.module_path())
                    .file(record.file())
                    .line(record.line())
                    .build(),
            )
        }
    }
    fn flush(&self) {}
}
