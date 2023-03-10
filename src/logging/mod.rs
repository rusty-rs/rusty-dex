pub extern crate chrono;
pub extern crate colored;

// TODO: add a stacktrace!() to print the stacktrace?

pub const DATETIME_FORMAT: &str = "%F %T";
pub static mut LOG_LEVEL: LogLevel = LogLevel::Error;

#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

#[macro_export]
macro_rules! die
{
    ($($arg:tt)*) => {{
        use $crate::error;
        error!($($arg)*);
        panic!();
    }}
}

#[macro_export]
macro_rules! error
{
    ($($arg:tt)*) => {{
        use $crate::logging::DATETIME_FORMAT;
        use $crate::logging::colored::Colorize;

        let time_now = $crate::logging::chrono::offset::Local::now();
        eprint!("[{}] {} [{}:{}] | ", time_now.format(DATETIME_FORMAT),
                                      "[ERROR]".red(),
                                      file!(),
                                      line!());
        eprintln!($($arg)*);
    }}
}

#[macro_export]
macro_rules! warning
{
    ($($arg:tt)*) => {{
        use $crate::logging::{
            LOG_LEVEL,
            DATETIME_FORMAT,
            LogLevel
        };
        use $crate::logging::colored::Colorize;

        let time_now = $crate::logging::chrono::offset::Local::now();
        match unsafe { &LOG_LEVEL } {
            LogLevel::Warning   |
                LogLevel::Info  |
                LogLevel::Debug => {
                    eprint!("[{}] {} [{}:{}] | ",
                            time_now.format(DATETIME_FORMAT),
                            "[WARNING]".yellow(),
                            file!(),
                            line!());
                    eprintln!($($arg)*);
                },
            _ => { },
        }
    }}
}

#[macro_export]
macro_rules! info
{
    ($($arg:tt)*) => {{
        use $crate::logging::{
            LOG_LEVEL,
            DATETIME_FORMAT,
            LogLevel
        };
        use $crate::logging::colored::Colorize;

        let time_now = $crate::logging::chrono::offset::Local::now();
        match unsafe { &LOG_LEVEL } {
            LogLevel::Info      |
                LogLevel::Debug => {
                    eprint!("[{}] {} [{}:{}] | ",
                            time_now.format(DATETIME_FORMAT),
                            "[INFO]".green(),
                            file!(),
                            line!());
                    eprintln!($($arg)*);
                },
            _ => { },
        }
    }}
}

#[macro_export]
macro_rules! debug
{
    ($($arg:tt)*) => {{
        use $crate::logging::{
            LOG_LEVEL,
            DATETIME_FORMAT,
            LogLevel
        };

        let time_now = $crate::logging::chrono::offset::Local::now();
        if let LogLevel::Debug = unsafe { &LOG_LEVEL } {
            eprint!("[{}] {} [{}:{}] | ", time_now.format(DATETIME_FORMAT),
                                          "[DEBUG]",
                                          file!(),
                                          line!());
            eprintln!($($arg)*);
        }
    }}
}

pub fn set_log_level(level: u8) {
    unsafe {
        LOG_LEVEL = match level {
            0     => LogLevel::Error,
            1     => LogLevel::Warning,
            2     => LogLevel::Info,
            3     => LogLevel::Debug,
            other => {
                error!("unknown log level {other}");
                describe_log_levels();
                std::process::exit(1);
            }
        };
    };
}

fn describe_log_levels() {
    println!("\nAvailable log levels:\n");
    println!("\tError   (0): only show unrecoverable errors (default value)");
    println!("\tWarning (1): show both recoverable and unrecoverable errors");
    println!("\tInfo    (2): show errors and info about the current execution");
    println!("\tDebug   (3): show everything, including verbose debug messages\n");
}
