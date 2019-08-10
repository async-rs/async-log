# async-log
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Async tracing capabilities for the standard [`log`] crate.

[`log`]: https://docs.rs/log

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

This crate provides extension types and hooks to `log` to enable asynchronous logging.

## What is Async Logging?
When building a _synchronous_ application, log messages can be relied on to always happen
in sequence. But unfortunately synchronous applications are rarely capable of utilizating
system resources to their full potential.

In contrast, concurrent applications make a lot better use of system resources. But it also
means we can no longer rely on log messages to strictly happen in sequence. In order to make
sense of logs in asynchronous applications, we need to be able to correlate sequences of events
with each other:

```txt
a1 -> b1 -> b2 -> a2 -> b3     # raw log stream

a1 -------------> a2           # parsed log stream a
      b1 -> b2 -------> b3     # parsed log stream b
```
_The raw log stream contains items for both "a" and "b". With async logging you want to be able
to distinguish between the items for "a", and the items from "b"._

## How do we correlate messages?
The goal of async logging is to determine which events happened in sequence inside your code. In
practice this means being able to correlate events with each other past _yield points_ (e.g.
`.await`), and _thread bounds_.

The way we do this is by adding the current task ID, and thread ID from where the log is
occurring. An whenever a _new_ task is spawned we log the following values:

- The ID of the task from which the new task is spawned (`task_parent_id`)
- The ID of the new task that's spawned (`task_id`)
- The current thread ID (`thread_id`)
- The line from where the task was spawned (`spawn_line`, when `RUST_BACKTRACE=1` enabled)

With all this information we have all the information to correlate tasks with each other. We
know what the parent task was, what the new task is, and log that information together. On the
receiving side we can then reconstruct that to create correlations.

## What is a span?
A span is a pair of messages. One is emitted at the start of an operation, and the other is
emitted at the end of the operation. If we add timestamps to when each message was sent, we're
able to determine how long operations take. Or determine which operations never finished.

In `async-log` each span is annotated with a `span_mark` message:
- `span_mark=start` marks the start of a span
- `span_mark=end` marks the end of a span

__example__
```txt
runtime::fs::read_to_string, span_mark=start, path=/tmp/foob, task_id=7, thread_id=8
runtime::fs::read_to_string, span_mark=end, path=/tmp/foob, task_id=7, thread_id=8
```

## Why build on the log crate?
[`log`](https://docs.rs/log/) is Rust's standard log crate. It's incredibly flexible, and was
built with extensibility in mind. Because it's so widely used, being able to extend it allows
us to add tracing data to crates without needing to make any changes to their `log!` calls.

## Formatting
Structured logging (key-value logging) is [currently in the
process](https://github.com/rust-lang-nursery/log/issues/328) of being added to `log`.

At the time of writing there are no published versions available with even the experimental
features available. So until then we have to add key-value pairs using strings. Once key-value
logging is added to `log` we'll publish a breaking change, and move over.

The syntax we've chosen to use is `foo=bar` pairs. Multiple pairs should be delimited using
commas (`,`). Every pair should come _after_ the first message. An example log looks like this:

```txt
a new cat has logged on, name=nori, snacks=always
```

## Examples
```rust
use async_log::span;
use log::info;

fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || 12)
        .start(log::LevelFilter::Trace)
        .unwrap();
}

fn main() {
    setup_logger();

    span!("level I", {
        let x = "beep";
        info!("look at this value, x={}", x);

        span!("level II", {
            let y = "boop";
            info!("another nice value, y={}", y);
        })
    })
}
```

## Installation
```sh
$ cargo add async-log
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
- [log](http://docs.rs/log)

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/async-log.svg?style=flat-square
[2]: https://crates.io/crates/async-log
[3]: https://img.shields.io/travis/rustasync/async-log/master.svg?style=flat-square
[4]: https://travis-ci.org/rustasync/async-log
[5]: https://img.shields.io/crates/d/async-log.svg?style=flat-square
[6]: https://crates.io/crates/async-log
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/async-log

[releases]: https://github.com/rustasync/async-log/releases
[contributing]: https://github.com/rustasync/async-log/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/rustasync/async-log/labels/good%20first%20issue
[help-wanted]: https://github.com/rustasync/async-log/labels/help%20wanted
