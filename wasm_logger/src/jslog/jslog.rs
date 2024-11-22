use wasm_bindgen::prelude::*;
use web_sys::console;

/// Logs an error message to the browser console.
#[wasm_bindgen]
pub fn error(args: &[&dyn std::fmt::Debug]) {
    log("error", args);
}

/// Logs a warning message to the browser console.
#[wasm_bindgen]
pub fn warn(args: &[&dyn std::fmt::Debug]) {
    log("warn", args);
}

/// Logs a debug message to the browser console.
#[wasm_bindgen]
pub fn debug(args: &[&dyn std::fmt::Debug]) {
    log("debug", args);
}

/// Logs an info message to the browser console.
#[wasm_bindgen]
pub fn info(args: &[&dyn std::fmt::Debug]) {
    log("info", args);
}

/// Logs a general message to the browser console.
#[wasm_bindgen]
pub fn log(args: &[&dyn std::fmt::Debug]) {
    log("log", args);
}

/// Logs a formatted error message to the browser console.
#[wasm_bindgen]
pub fn errorf(format: &str, args: &[&dyn std::fmt::Display]) {
    logf("error", format, args);
}

/// Logs a formatted warning message to the browser console.
#[wasm_bindgen]
pub fn warnf(format: &str, args: &[&dyn std::fmt::Display]) {
    logf("warn", format, args);
}

/// Logs a formatted debug message to the browser console.
#[wasm_bindgen]
pub fn debugf(format: &str, args: &[&dyn std::fmt::Display]) {
    logf("debug", format, args);
}

/// Logs a formatted info message to the browser console.
#[wasm_bindgen]
pub fn infof(format: &str, args: &[&dyn std::fmt::Display]) {
    logf("info", format, args);
}

/// Logs a formatted general message to the browser console.
#[wasm_bindgen]
pub fn logf(format: &str, args: &[&dyn std::fmt::Display]) {
    logf_internal(format, args);
}

fn log(level: &str, args: &[&dyn std::fmt::Debug]) {
    let js_args: Vec<JsValue> = args.iter().map(|arg| JsValue::from_serde(arg).unwrap_or_else(|_| JsValue::NULL)).collect();
    match level {
        "error" => console::error_1(&js_args.into_iter().next().unwrap_or(JsValue::NULL)),
        "warn" => console::warn_1(&js_args.into_iter().next().unwrap_or(JsValue::NULL)),
        "debug" => console::debug_1(&js_args.into_iter().next().unwrap_or(JsValue::NULL)),
        "info" => console::info_1(&js_args.into_iter().next().unwrap_or(JsValue::NULL)),
        "log" => console::log_1(&js_args.into_iter().next().unwrap_or(JsValue::NULL)),
        _ => {}
    }
}

fn logf_internal(format: &str, args: &[&dyn std::fmt::Display]) {
    let message = format!(format, args);
    console::log_1(&JsValue::from_str(&message));
}

// // 示例用法
// #[wasm_bindgen]
// pub fn main() {
//     error(&["Error message"]);
//     warn(&["Warning message"]);
//     debug(&["Debug message"]);
//     info(&["Info message"]);
//     log(&["General message"]);

//     errorf("Error: {}", &["Something went wrong"]);
//     warnf("Warning: {}", &["Be careful"]);
//     debugf("Debug: {}", &["This is a debug message"]);
//     infof("Info: {}", &["This is an info message"]);
//     logf("Log: {}", &["This is a general message"]);
// }