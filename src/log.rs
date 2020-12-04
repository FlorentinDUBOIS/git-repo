//! # Log module
//!
//! This module provide all stuffs to provide log facilities
use std::cmp::min;

use slog::{o, Drain, Level, LevelFilter, Logger};
use slog_async::Async;
use slog_scope::{set_global_logger, GlobalLoggerGuard as Guard};
use slog_term::{FullFormat, TermDecorator};

pub fn initialize(verbosity: usize) -> Guard {
    let level = min(
        Level::Trace.as_usize(),
        Level::Critical.as_usize() + verbosity,
    );
    let level = Level::from_usize(level).unwrap_or_else(|| Level::Trace);

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();
    let drain = LevelFilter::new(drain, level).fuse();

    set_global_logger(Logger::root(drain, o!()))
}
