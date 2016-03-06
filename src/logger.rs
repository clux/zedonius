use log::{self, Log, LogLevel, LogMetadata, LogRecord, SetLoggerError};
use std::io::{self, Write};
use ansi_term::{Colour, Style};

struct StdioLogger {
    log_level: LogLevel,
}

fn level_style(l: LogLevel) -> Style {
    match l {
        LogLevel::Error => Colour::Red.bold(),
        LogLevel::Warn => Colour::Purple.bold(),
        LogLevel::Info => Colour::White.bold(),
        LogLevel::Debug => Colour::White.dimmed(),
        LogLevel::Trace => Colour::White.dimmed(),
    }
}

impl Log for StdioLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let msg = format!("{}: {}",
                              level_style(record.level()).paint(record.location().module_path()),
                              record.args());
            if record.level() <= LogLevel::Warn {
                writeln!(&mut io::stderr(), "{}", msg);
            } else {
                println!("{}", msg);
            }
        }
    }
}

pub fn init_with_level(log_level: LogLevel) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log_level.to_log_level_filter());
        Box::new(StdioLogger { log_level: log_level })
    })
}

pub fn init_with_verbosity(verbosity: usize) -> Result<(), SetLoggerError> {
    let level = match verbosity {
        0 => LogLevel::Warn,  // default
        1 => LogLevel::Info,  // -v
        2 => LogLevel::Debug, // -vv
        _ => LogLevel::Trace, // -vvv and above
    };
    init_with_level(level)
}
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(LogLevel::Info)
}
