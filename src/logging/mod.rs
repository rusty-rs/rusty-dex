use chrono::offset::Local;
use colored::Colorize;

const DATETIME_FORMAT: &str = "%F %T";

#[derive(Debug)]
enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

pub struct Logger {
    log_level: LogLevel,
}

impl Logger {
    pub fn new(log_level: u8) -> Self {
        let log_level = match log_level {
            0     => LogLevel::Error,
            1     => LogLevel::Warning,
            2     => LogLevel::Info,
            3     => LogLevel::Debug,
            other => panic!("Error: unknown log level {other}")
        };

        Logger { log_level }
    }

    pub fn error(&self, msg: String) {
        /* Error are always printed, regardless of the log level */
        let now = Local::now();
        println!("[{}] {} {}", now.format(DATETIME_FORMAT),
                               "[ERROR]".red(),
                               msg);
    }

    pub fn warning(&self, msg: String) {
        let now = Local::now();
        match self.log_level {
            LogLevel::Warning   |
                LogLevel::Info  |
                LogLevel::Debug => {
                println!("[{}] {} {}", now.format(DATETIME_FORMAT),
                                       "[WARNING]".yellow(),
                                       msg);
                },
            _               => { }
        }
    }

    pub fn info(&self, msg: String) {
        let now = Local::now();
        match self.log_level {
            LogLevel::Info      |
                LogLevel::Debug => {
                println!("[{}] {} {}", now.format(DATETIME_FORMAT),
                                       "[INFO]".green(),
                                       msg);
                },
            _                     => { }
        }
    }

    pub fn debug(&self, msg: String) {
        let now = Local::now();
        if let LogLevel::Debug = self.log_level {
            println!("[{}] [DEBUG] {}",
                     now.format(DATETIME_FORMAT),
                     msg);
        }
    }
}
