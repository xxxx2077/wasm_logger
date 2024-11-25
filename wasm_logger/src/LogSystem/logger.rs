// 日志级别和日志记录器
use wasm_bindgen::prelude::*;
use web_sys::console;
use std::sync::Mutex;

#[wasm_bindgen]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[wasm_bindgen]
pub struct Logger {
    level: LogLevel,
}

impl Logger {
    fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    fn should_log(&self, level: LogLevel) -> bool {
        self.level as u8 <= level as u8
    }
}

#[wasm_bindgen]
impl Logger {
    #[wasm_bindgen(constructor)]
    pub fn new_with_level(level: LogLevel) -> Self {
        Logger::new(level)
    }

    pub fn trace(&self, message: &str) {
        if self.should_log(LogLevel::Trace) {
            self.log("TRACE", message);
        }
    }

    pub fn debug(&self, message: &str) {
        if self.should_log(LogLevel::Debug) {
            self.log("DEBUG", message);
        }
    }

    pub fn info(&self, message: &str) {
        if self.should_log(LogLevel::Info) {
            self.log("INFO", message);
        }
    }

    pub fn warn(&self, message: &str) {
        if self.should_log(LogLevel::Warn) {
            self.log("WARN", message);
        }
    }

    pub fn error(&self, message: &str) {
        if self.should_log(LogLevel::Error) {
            self.log("ERROR", message);
        }
    }

    fn log(&self, level: &str, message: &str) {
        let timestamp = js_sys::Date::new_0().to_string();
        let formatted_message = format!("[{}] [{}] {}", timestamp, level, message);
        console::log_1(&JsValue::from_str(&formatted_message));
    }
}