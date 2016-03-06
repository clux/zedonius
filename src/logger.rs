use log::{self, Log, LogLevel, LogMetadata, LogRecord, SetLoggerError};
use std::io::{self, Write};
use ansi_term::Colour;

struct StdioLogger {
    log_level: LogLevel,
}


fn level_style(l: LogLevel) -> Colour {
    match l {
        LogLevel::Error => Colour::Fixed(9), // bright red
        LogLevel::Warn => Colour::Fixed(11), // bright yellow
        LogLevel::Info => Colour::Fixed(10), // bright green
        LogLevel::Debug => Colour::Fixed(7), // light grey
        LogLevel::Trace => Colour::Fixed(8), // grey
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

#[cfg(test)]
mod tests {
    use super::init_with_verbosity;

    #[test]
    fn init_and_macros() {
        let l = init_with_verbosity(3);
        assert_eq!(l.is_ok(), true);
        error!("error log");
        warn!("warn log");
        info!("info log");
        debug!("debug log");
        trace!("trace log");
    }
}
