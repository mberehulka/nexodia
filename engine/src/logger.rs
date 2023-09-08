use std::{path::PathBuf, io::Write, fs::OpenOptions};
use chrono::Timelike;
use log::Level;

const FG_UNSET: &'static str = "\x1b[0m";
const _FG_BLACK: &'static str = "\x1b[30m";
const _FG_WHITE: &'static str = "\x1b[97m";
const FG_RED: &'static str = "\x1b[31m";
const FG_GREEN: &'static str = "\x1b[32m";
const FG_YELLOW: &'static str = "\x1b[33m";
const FG_BLUE: &'static str = "\x1b[34m";
const _FG_MAGENTA: &'static str = "\x1b[35m";
const _FG_CYAN: &'static str = "\x1b[36m";
const FG_DARK_GRAY: &'static str = "\x1b[90m";
const _FG_LIGHT_GRAY: &'static str = "\x1b[37m";
const _FG_LIGHT_RED: &'static str = "\x1b[91m";
const _FG_LIGHT_GREEN: &'static str = "\x1b[92m";
const _FG_LIGHT_YELLOW: &'static str = "\x1b[93m";
const _FG_LIGHT_BLUE: &'static str = "\x1b[94m";
const _FG_LIGHT_MAGENTA: &'static str = "\x1b[95m";
const _FG_LIGHT_CYAN: &'static str = "\x1b[96m";

pub struct Logger {
    path: PathBuf
}
impl Logger {
    pub fn new() -> &'static Self {
        let s: &'static Self = Box::leak(Box::new(Self {
            path: std::env::current_dir().unwrap_or(PathBuf::from("./")).join("nexodia.log")
        }));
        std::fs::remove_file(&s.path).ok();
        log::set_logger(s).unwrap();
        log::set_max_level(log::LevelFilter::Info);
        std::panic::set_hook(Box::new(|e| {
            let now = chrono::Local::now();
            let second = now.second();
            let minute = now.minute();
            let msg = [
                e.to_string(),
                pretty_backtrace()
            ].join("\n");
            std::io::stdout().write_all(
                format!("{FG_DARK_GRAY}[{second:02}:{minute:02}]{FG_UNSET} {msg}").as_bytes()
            ).ok();
            OpenOptions::new().create(true).read(true).write(true).append(true).open(&s.path)
                .and_then(|mut file| {
                    file.write_all(format!("[{second:02}:{minute:02}] {msg}").as_bytes())
                }).ok();
            native_dialog::MessageDialog::new()
                .set_title("Nexodia")
                .set_text(&msg)
                .set_type(native_dialog::MessageType::Error)
                .show_alert().ok();
        }));
        s
    }
}
impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool { true }
    fn log(&self, record: &log::Record) {
        let module = record.module_path().unwrap_or_default();

        match record.level() {
            Level::Info => if module.starts_with("wgpu") { return }
            Level::Warn => if module.starts_with("wgpu_hal::auxil::dxgi::exception") { return }
            _ => {}
        }
        
        let msg = if let Level::Warn | Level::Error = record.level() {
            [record.args().to_string(), pretty_backtrace()].join("\n")
        } else {
            record.args().to_string()
        };

        let now = chrono::Local::now();
        let second = now.second();
        let minute = now.minute();
        let line = record.line().unwrap_or_default();
        let level_str = record.level().as_str().to_uppercase();
        let level_colored = [
            match record.level() {
                Level::Trace => FG_DARK_GRAY,
                Level::Warn => FG_YELLOW,
                Level::Error => FG_RED,
                Level::Debug => FG_GREEN,
                Level::Info => FG_BLUE
            },
            &level_str,
            FG_DARK_GRAY
        ].join("");
        std::io::stdout().write_all(
            format!("{FG_DARK_GRAY}[{second:02}:{minute:02} {level_colored} {module}:{line}]{FG_UNSET} {msg}\n").as_bytes()
        ).ok();
        OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(&self.path)
            .and_then(|mut file|
                file.write_all(
                    format!("[{second:02}:{minute:02} {level_str} {module}:{line}] {msg}\n").as_bytes()
                )
            ).ok();
        if let Level::Error = record.level() {
            native_dialog::MessageDialog::new()
                .set_title("Nexodia")
                .set_text(&msg)
                .set_type(native_dialog::MessageType::Error)
                .show_alert().ok();
        }
    }
    fn flush(&self) {}
}

fn pretty_backtrace() -> String {
    backtrace::Backtrace::new()
        .frames()
        .into_iter()
        .map(|frame| frame.symbols())
        .flatten()
        .filter_map(|symbol| {
            match (symbol.name(), symbol.lineno()) {
                (Some(name), Some(line)) => {
                    let name = name.to_string();
                    if
                        name.starts_with("std::") ||
                        name.starts_with("log::") ||
                        name.starts_with("backtrace::") ||
                        name.starts_with("engine::logger::pretty_backtrace") ||
                        name.contains("$") ||
                        name.contains("<") ||
                        !name.contains("::")
                    {
                        None
                    } else {
                        Some(format!("\t{name}:{line}"))
                    }
                },
                _ => None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}