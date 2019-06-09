#[doc(hidden)]
#[macro_export]
macro_rules! span_inner {
    ($name:expr, $block:expr) => {{
        let span = async_log::Span::new($name);
        let res = $block;
        drop(span);
        res
    }};
    ($name:expr, $block:expr, $args:expr) => {{
        let span = async_log::Span::with_args($name, $args);
        let res = $block;
        drop(span);
        res
    }};
}

/// Create a tracing span.
///
/// Spans are pairs of `trace!` logs. Every `span` wraps a block, and logs a message at the start,
/// of the block and a message after the block has finished. This works in asynchronous contexts
/// too.
///
/// Each span takes a `name`, a block, and optionally a list of key-value pairs in between those.
/// Once structured logging becomes part of `log` (currently feature gated as `kv_unstable`), we'll
/// move to support arbitrary key-value pairs.
///
/// Because of the way this macro is constructed, we currently support up to 9 key-value pairs.
/// Which makes a total of 12 arguments.
///
/// ## Examples
/// ```
/// use async_log::span;
/// use log::info;
///
/// span!("main", {
///     let x = "foo";
///     info!("this {}", x);
///
///     span!("inner", "x={}", x, {
///         info!("we must go deeper {}", x);
///     });
/// })
/// ```
#[macro_export]
macro_rules! span {
    ($name:expr, $block:expr) => {{
        async_log::span_inner!($name, $block)
    }};
    ($name:expr, $fmt:expr, $block:expr) => {{
        async_log::span_inner!($name, $block, $fmt)
    }};
    ($name:expr, $fmt:expr, $a:expr, $block:expr) => {{
        let args = format!($fmt, $a);
        async_log::span_inner!($name, $block, args)
    }};
    ($name:expr, $fmt:expr, $a:expr, $b:expr, $block:expr) => {{
        let args = format!($fmt, $a, $b);
        async_log::span_inner!($name, $block, args)
    }};
    ($name:expr, $fmt:expr, $a:expr, $b:expr, $c:expr, $block:expr) => {{
        let args = format!($fmt, $a, $b, $c);
        async_log::span_inner!($name, $block, args)
    }};
    ($name:expr, $fmt:expr, $a:expr, $b:expr, $c:expr, $d:expr, $block:expr) => {{
        let args = format!($fmt, $a, $b, $c, $d);
        async_log::span_inner!($name, $block, args)
    }};
    ($name:expr, $fmt:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $block:expr) => {{
        let args = format!($fmt, $a, $b, $c, $d, $e);
        async_log::span_inner!($name, $block, args)
    }};
    ($name:expr, $fmt:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $block:expr) => {{
        let args = format!($fmt, $a, $b, $c, $d, $e, $f);
        async_log::span_inner!($name, $block, args)
    }};
    ($name:expr, $fmt:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $block:expr) => {{
        let args = format!($fmt, $a, $b, $c, $d, $e, $f, $g);
        async_log::span_inner!($name, $block, args)
    }};
}
