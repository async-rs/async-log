use crate::backtrace::async_log_capture_caller;
use log::{Log, Metadata, Record};

// static ASYNC_LOGGER: AsyncLogger<L> = AsyncLogger;

/// Wrap an async logger, extending it with async functionality.
#[derive(Debug)]
pub struct AsyncLogger<L: Log, F>
where
    F: Fn() -> (u64, Option<u64>) + Send + Sync + 'static,
{
    logger: L,
    with: F,
    depth: u8,
}

impl<L: Log, F> AsyncLogger<L, F>
where
    F: Fn() -> (u64, Option<u64>) + Send + Sync + 'static,
{
    /// Wrap an existing logger, extending it with async functionality.
    pub fn wrap(logger: L, depth: u8, with: F) -> Self {
        Self {
            logger,
            depth,
            with,
        }
    }

    fn with(&self) -> (u64, Option<u64>) {
        (self.with)()
    }
}

impl<L: Log, F> log::Log for AsyncLogger<L, F>
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