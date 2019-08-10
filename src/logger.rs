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
fn thread_id() -> u64 {
    let mut string = format!("{:?}", thread::current().id());
    string.replace_range(0..9, "");
    string.pop();
    string.parse().unwrap()
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
            let depth = self.compute_stack_depth(&record);
            let symbol = async_log_capture_caller(depth);

            let key_values = KeyValues {
                thread_id: thread_id(),
                task_id: self.with(),
                kvs: record.key_values(),
            };

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
                        "{}{}{}{}",
                        record.args(),
                        filename,
                        line,
                        fn_name,
                    ))
                    .metadata(record.metadata().clone())
                    .key_values(&key_values)
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

struct KeyValues<'a> {
    thread_id: u64,
    task_id: u64,
    kvs: &'a dyn log::kv::Source,
}
impl<'a> log::kv::Source for KeyValues<'a> {
    fn visit<'kvs>(
        &'kvs self,
        visitor: &mut dyn log::kv::Visitor<'kvs>,
    ) -> Result<(), log::kv::Error> {
        self.kvs.visit(visitor)?;
        visitor.visit_pair("thread_id".into(), self.thread_id.into())?;
        visitor.visit_pair("task_id".into(), self.task_id.into())?;
        Ok(())
    }
}
