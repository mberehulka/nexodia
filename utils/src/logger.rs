use std::{path::PathBuf, io::Write, fs::OpenOptions};
use chrono::Timelike;
use log::Level;

use crate::foreground::*;

pub fn log_file() -> PathBuf {
    std::env::current_dir().unwrap_or(PathBuf::from("./")).join("nexodia.log")
}

pub struct Logger {}
impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool { true }
    fn log(&self, record: &log::Record) {
        let now = chrono::Local::now();
        let second = now.second();
        let minute = now.minute();
        let module = record.module_path().unwrap_or_default();
        let line = record.line().unwrap_or_default();
        let msg = record.args();
        let level_str = &record.level().as_str().to_uppercase();
        let level_colored = [
            match record.level() {
                Level::Trace => DARK_GRAY,
                Level::Warn => YELLOW,
                Level::Error => RED,
                Level::Debug => GREEN,
                Level::Info => BLUE
            },
            level_str,
            DARK_GRAY
        ].join("");
        std::io::stdout().write_all(format!("{DARK_GRAY}[{second:02}:{minute:02} {level_colored} {module}:{line}]{UNSET} {msg}\n").as_bytes()).ok();
        OpenOptions::new().create(true).read(true).write(true).append(true).open(log_file())
            .and_then(|mut file|file.write_all(format!("[{second:02}:{minute:02} {level_str} {module}:{line}] {msg}\n").as_bytes())).ok();
        if let Level::Error = record.level() {
            native_dialog::MessageDialog::new()
                .set_title("Nexodia")
                .set_text(&format!("{msg}"))
                .set_type(native_dialog::MessageType::Error)
                .show_alert().ok();
        }
    }
    fn flush(&self) {}
}

pub fn init_logs() {
    std::fs::remove_file(log_file()).ok();
    log::set_logger(Box::leak(Box::new(Logger{}))).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    std::panic::set_hook(Box::new(|e| {
        let backtrace = backtrace::Backtrace::new();
        let now = chrono::Local::now();
        let second = now.second();
        let minute = now.minute();
        std::io::stdout().write_all(format!("{DARK_GRAY}[{second:02}:{minute:02}]{UNSET} {e}").as_bytes()).ok();
        OpenOptions::new().create(true).read(true).write(true).append(true).open(log_file())
            .and_then(|mut file|file.write_all(format!("[{second:02}:{minute:02}] {e}\n\n{backtrace:#?}").as_bytes())).ok();
        native_dialog::MessageDialog::new()
            .set_title("Nexodia")
            .set_text(&e.to_string())
            .set_type(native_dialog::MessageType::Error)
            .show_alert().ok();
    }))
}