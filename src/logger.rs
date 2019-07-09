use crate::backtrace::async_log_capture_caller;
use log::{set_boxed_logger, LevelFilter, Log, Metadata, Record};

use std::thread;

/// A Logger that wraps other loggers to extend it with async functionality.
#[derive(Debug)]
pub struct Logger<L: Log + 'static, F>
where
    F: Fn() -> u64 + Send + Sync + 'static,
{
    backtrace: bool,
    logger: L,
    with: F,
}

impl<L: Log + 'static, F> Logger<L, F>
where
    F: Fn() -> u64 + Send + Sync + 'static,
{
    /// Wrap an existing logger, extending it with async functionality.
    pub fn wrap(logger: L, with: F) -> Self {
        let backtrace = std::env::var_os("RUST_BACKTRACE")
            .map(|x| &x != "0")
            .unwrap_or(false);
        Self {
            logger,
            backtrace,
            with,
        }
    }

    /// Start logging.
    pub fn start(self, filter: LevelFilter) -> Result<(), log::SetLoggerError> {
        let res = set_boxed_logger(Box::new(self));
        if res.is_ok() {
            log::set_max_level(filter);
        }
        res
    }

    /// Call the `self.with` closure, and return its results.
    fn with(&self) -> u64 {
        (self.with)()
    }

    /// Compute which stack frame to log based on an offset defined inside the log message.
    /// This message is then stripped from the resulting record.
    fn compute_stack_depth(&self, _record: &Record<'_>) -> u8 {
        4
    }
}

/// Get the thread id. Useful because ThreadId doesn't implement Display.
fn thread_id() -> String {
    let mut string = format!("{:?}", thread::current().id());
    string.replace_range(0..9, "");
    string.pop();
    string
}

impl<L: Log, F> log::Log for Logger<L, F>
where
    F: Fn() -> u64 + Send + Sync + 'static,
{
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        self.logger.enabled(metadata)
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let curr_id = self.with();
            let depth = self.compute_stack_depth(&record);
            let symbol = async_log_capture_caller(depth);

            let thread_id = format!(", thread_id={}", thread_id());
            let task_id = format!(", task_id={}", curr_id);

            let (line, filename, fn_name) = if self.backtrace {
                match symbol {
                    Some(symbol) => {
                        let line = symbol
                            .lineno
                            .map(|l| format!(", line={}", l))
                            .unwrap_or_else(|| String::from(""));

                        let filename = symbol
                            .filename
                            .map(|f| format!(", filename={}", f.to_string_lossy()))
                            .unwrap_or_else(|| String::from(""));

                        let fn_name = symbol
                            .name
                            .map(|l| format!(", fn_name={}", l))
                            .unwrap_or_else(|| String::from(""));

                        (line, filename, fn_name)
                    }
                    None => (String::from(""), String::from(""), String::from("")),
                }
            } else {
                (String::from(""), String::from(""), String::from(""))
            };

            // This is done this way b/c `Record` + `format_args` needs to be built inline. See:
            // https://stackoverflow.com/q/56304313/1541707
            self.logger.log(
                &log::Record::builder()
                    .args(format_args!(
                        "{}{}{}{}{}{}",
                        record.args(),
                        filename,
                        line,
                        fn_name,
                        task_id,
                        thread_id
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
