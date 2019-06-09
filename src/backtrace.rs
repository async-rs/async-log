/// The return type of `async_log_capture_caller`
#[derive(Debug)]
pub(crate) struct Symbol {
    name: Option<String>,
    lineno: Option<u32>,
    filename: Option<std::path::PathBuf>,
}

/// Find the method that called this.
///
/// `depth` is how many frames we should look past finding the method where this was called.
/// This might require a bit of wrangling to find.
#[no_mangle]
pub(crate) fn async_log_capture_caller(depth: u8) -> Option<Symbol> {
    let mut count = 0;
    let mut counting = false;
    let self_name = "async_log_capture_caller";
    let mut ret_symbol: Option<Symbol> = None;

    backtrace::trace(|frame| {
        if ret_symbol.is_some() {
            return false;
        }
        backtrace::resolve_frame(frame, |symbol| {
            if let Some(name) = symbol.name() {
                if format!("{}", name) == self_name {
                    counting = true;
                }

                if !counting {
                    return;
                }

                count += 1;
                if count == depth {
                    ret_symbol = Some(Symbol {
                        name: symbol.name().map(|s| format!("{}", s)),
                        lineno: symbol.lineno(),
                        filename: symbol.filename().map(|p| p.to_path_buf()),
                    })
                }
            }
        });
        true
    });
    ret_symbol
}
