use std::{path::PathBuf, io::Write, fs::OpenOptions};
use chrono::Timelike;
use log::Level;

pub const FG_UNSET: &'static str = "\x1b[0m";
pub const FG_BLACK: &'static str = "\x1b[30m";
pub const FG_WHITE: &'static str = "\x1b[97m";
pub const FG_RED: &'static str = "\x1b[31m";
pub const FG_GREEN: &'static str = "\x1b[32m";
pub const FG_YELLOW: &'static str = "\x1b[33m";
pub const FG_BLUE: &'static str = "\x1b[34m";
pub const FG_MAGENTA: &'static str = "\x1b[35m";
pub const FG_CYAN: &'static str = "\x1b[36m";
pub const FG_DARK_GRAY: &'static str = "\x1b[90m";
pub const FG_LIGHT_GRAY: &'static str = "\x1b[37m";
pub const FG_LIGHT_RED: &'static str = "\x1b[91m";
pub const FG_LIGHT_GREEN: &'static str = "\x1b[92m";
pub const FG_LIGHT_YELLOW: &'static str = "\x1b[93m";
pub const FG_LIGHT_BLUE: &'static str = "\x1b[94m";
pub const FG_LIGHT_MAGENTA: &'static str = "\x1b[95m";
pub const FG_LIGHT_CYAN: &'static str = "\x1b[96m";

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
                Level::Trace => FG_DARK_GRAY,
                Level::Warn => FG_YELLOW,
                Level::Error => FG_RED,
                Level::Debug => FG_GREEN,
                Level::Info => FG_BLUE
            },
            level_str,
            FG_DARK_GRAY
        ].join("");
        std::io::stdout().write_all(format!("{FG_DARK_GRAY}[{second:02}:{minute:02} {level_colored} {module}:{line}]{FG_UNSET} {msg}\n").as_bytes()).ok();
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
        std::io::stdout().write_all(format!("{FG_DARK_GRAY}[{second:02}:{minute:02}]{FG_UNSET} {e}").as_bytes()).ok();
        OpenOptions::new().create(true).read(true).write(true).append(true).open(log_file())
            .and_then(|mut file|file.write_all(format!("[{second:02}:{minute:02}] {e}\n\n{backtrace:#?}").as_bytes())).ok();
        native_dialog::MessageDialog::new()
            .set_title("Nexodia")
            .set_text(&e.to_string())
            .set_type(native_dialog::MessageType::Error)
            .show_alert().ok();
    }))
}