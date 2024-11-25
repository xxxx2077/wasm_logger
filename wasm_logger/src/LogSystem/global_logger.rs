// 全局日志记录器
lazy_static! {
    pub static ref GLOBAL_LOGGER: Mutex<Logger> = Mutex::new(Logger::new(LogLevel::Info));
}

#[wasm_bindgen]
pub fn set_global_log_level(level: LogLevel) {
    let mut logger = GLOBAL_LOGGER.lock().unwrap();
    logger.level = level;
}

#[wasm_bindgen]
pub fn trace(message: &str) {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    logger.trace(message);
}

#[wasm_bindgen]
pub fn debug(message: &str) {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    logger.debug(message);
}

#[wasm_bindgen]
pub fn info(message: &str) {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    logger.info(message);
}

#[wasm_bindgen]
pub fn warn(message: &str) {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    logger.warn(message);
}

#[wasm_bindgen]
pub fn error(message: &str) {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    logger.error(message);
}