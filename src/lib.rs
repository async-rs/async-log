//! Async tracing capabilities for the log crate.
//!
//! ## Example
//!
//! ```rust
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

/// A new span created through `span!`.
#[must_use]
#[derive(Debug)]
pub struct Span {
    name: String,
}

impl Span {
    /// Create a new instance.
    ///
    /// You should generally prefer to call `span!` instead of constructing this manually.
    pub fn new(name: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        log::trace!("{}, span_mark=start", name);
        Self { name: name.to_owned() }
    }

    /// Create a new instance with arguments.
    ///
    /// You should generally prefer to call `span!` instead of constructing this manually.
    pub fn with_args(name: impl AsRef<str>, args: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        log::trace!("{}, span_mark=start, {}", name, args.as_ref());
        Self { name: name.to_owned() }
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        log::trace!("{}, span_mark=end", self.name);
    }
}

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

/// Create a span
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
