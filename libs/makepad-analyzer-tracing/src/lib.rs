#![allow(dead_code)]

mod log_with_styles;

use std::{env, io};
use tracing::{level_filters::LevelFilter, Level, Metadata};

pub use tracing_subscriber::{
  self,
  filter::EnvFilter,
  fmt::{format::FmtSpan, MakeWriter},
};
const LOG_FILTER: &str = "RUST_LOG";

// This allows us to write ERROR and WARN level logs to stderr and everything else to stdout.
// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/trait.MakeWriter.html
pub struct StdioTracingWriter {
  pub writer_mode: TracingWriterMode,
}

impl<'a> MakeWriter<'a> for StdioTracingWriter {
  type Writer = Box<dyn io::Write>;

  fn make_writer(&'a self) -> Self::Writer {
      if self.writer_mode == TracingWriterMode::Stderr {
          Box::new(io::stderr())
      } else {
          // We must have an implementation of `make_writer` that makes
          // a "default" writer without any configuring metadata. Let's
          // just return stdout in that case.
          Box::new(io::stdout())
      }
  }

  fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer {
      // Here's where we can implement our special behavior. We'll
      // check if the metadata's verbosity level is WARN or ERROR,
      // and return stderr in that case.
      if self.writer_mode == TracingWriterMode::Stderr
          || (self.writer_mode == TracingWriterMode::Stdio && meta.level() <= &Level::WARN)
      {
          return Box::new(io::stderr());
      }

      // Otherwise, we'll return stdout.
      Box::new(io::stdout())
  }
}

#[derive(PartialEq, Eq)]
pub enum TracingWriterMode {
  /// Write ERROR and WARN to stderr and everything else to stdout.
  Stdio,
  /// Write everything to stdout.
  Stdout,
  /// Write everything to stderr.
  Stderr,
}

#[derive(Default)]
pub struct TracingSubscriberOptions {
  pub verbosity: Option<u8>,
  pub silent: Option<bool>,
  pub log_level: Option<LevelFilter>,
  pub writer_mode: Option<TracingWriterMode>,
}

pub fn init_tracing_subscriber(options: TracingSubscriberOptions) {
  let env_filter = match env::var_os(LOG_FILTER) {
    Some(_) => EnvFilter::try_from_default_env().expect("Invalid `RUST_LOG` value"),
    None => EnvFilter::new("info"),
  };

  let level_filter = options
    .log_level
    .or_else(|| {
        options.verbosity.and_then(|verbosity| {
            match verbosity {
                1 => Some(LevelFilter::DEBUG), // matches --verbose or -v
                2 => Some(LevelFilter::TRACE), // matches -vv
                _ => None,
            }
        })
    })
    .or_else(|| {
        options
            .silent
            .and_then(|silent| if silent { Some(LevelFilter::OFF) } else { None })
    });

  let builder = tracing_subscriber::fmt::Subscriber::builder()
    .with_env_filter(env_filter)
    .with_ansi(true)
    .with_level(false)
    .without_time()
    .with_file(false)
    .with_writer(StdioTracingWriter {
      writer_mode: options.writer_mode.unwrap_or(TracingWriterMode::Stdio),
    });

    if let Some(level_filter) = level_filter {
      builder.with_max_level(level_filter).init();
    } else {
      builder.init();
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tracing_test::traced_test;

  #[traced_test]
  #[test]
  fn test_log_with_styles() {
    log_with_styles::log_warning("This is a warning");
    log_with_styles::log_error("This is an error");
    log_with_styles::log_green("This is green");
    log_with_styles::log_yellow("This is yellow");
    log_with_styles::log_green_bold("This is green bold");
    log_with_styles::log_yellow_bold("This is yellow bold");
    log_with_styles::log_yellow_err("This is yellow error");
    log_with_styles::log_red_err("This is red error");
  }
}
