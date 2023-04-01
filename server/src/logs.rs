use log::{Log, Record, Level, Metadata, LevelFilter};
use std::{fs, path::PathBuf};
use chrono::Utc;
use std::io::Write;

struct Logger {
    info_file: fs::File,
    error_file: fs::File,
}

impl Logger {
    fn new(path: &str) -> Logger {
        let log_dir_path =  PathBuf::from(path);
        create_folder_path(&log_dir_path);

        let mut info_log = log_dir_path.clone();
        let mut error_log = log_dir_path.clone();

        let timestamp = Utc::now().format("%Y-%m-%d").to_string();

        info_log.push(format!("info-{}.log", timestamp));
        error_log.push(format!("error-{}.log", timestamp));

        Self {
            info_file: create_new_file(&info_log),
            error_file: create_new_file(&error_log),
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn flush(&self) {}

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
            let message = format!("{} - {} - {}", timestamp, record.level(), record.args());
            let mut file = match record.level() {
                Level::Error => &self.error_file,
                _ => &self.info_file,
            };

            if let Err(err) = writeln!(file, "{}", message) {
                eprintln!("error writing to log file: {}", err);
            }
        }
    }
}

pub fn set_log(path: &str, level: LevelFilter) {
    let logger = Logger::new(path);
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(level);
}

fn create_new_file(name: &PathBuf) -> fs::File {
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(name)
        .ok()
        .unwrap()
}

fn create_folder_path(path: &PathBuf) {
    if !path.exists() {
        if let Err(e) = fs::create_dir_all(path) {
            eprintln!("failed to create directory {}: {}", path.display(), e);
        }
    }
}