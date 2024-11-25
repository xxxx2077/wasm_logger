use std::env;
use std::thread;
use std::time::{Duration, Instant};
use wasm_bindgen::prelude::*;

mod logger; // 假设 logger.rs 在同一目录下，并且正确地模块化了

use logger::{Logger, LogLevel};

fn main() {
    // 初始化日志记录器
    let mut logger = Logger::new_with_level(LogLevel::Error);

    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    let log_number: usize = match args.get(1) {
        Some(arg) => arg.parse().expect("lognumber must be an integer"),
        None => 100,
    };
    let length: String = match args.get(2) {
        Some(arg) => arg.clone(),
        None => "log20".to_string(),
    };

    // 预定义的日志内容
    let print_log = vec![
        ("log20", "This is log hotspot!"),
        ("log50", "This is hotspots.arXiEDHcPlfrcKwJxWjrjAYLvWkgzD"),
        ("log100", "This is hotspots.bJzsxyrMpzYGcfgKquEUPTHQtiFzeEHJxTEmdAtQQHhpfjtrKClFLsfjacHuTDkgzWNGmSOaZnzLxoUp"),
        // 其他日志长度省略...
        ("log1000", "This is hotspots.htTGLdjysnJBmraMbjsSytuKfxduwsdQJyzsIneGUNyiIYLwXBHgWupqOXLfUXOnpYbkJriscvFxGMRXbQsuipuBTsiMjpXKpbNcEoFWDDiMGeORcLWacLLOdcMaEVMcPDTpwNeEvUCgfcAARWsVBogiRstmyNCFwLfyvRdfdESZeSdjVzXDVGIKQztMylXzZfgCSzKIjiVtYwjHNTIwFDvhpyTgjKLfAFOHtJaVKEmzxDxrQJPmENyfBpQTDgFtwJYSwGPJkXBxoWKyhlVgavTnAOIgxSMsupviMzCttszFjubiNpFuierLCDVhsLudMgFbyaRFuapcYfGrvMVrIKWvoDEBHAdgLcGWVxMduuMkhHtAUtnCKHcYFkvXTCbixbGKpZBCuOkpcTCVTsGFZahRgzCwPHmKqaYougbHiOiDDSHatwIAKQDqhKNJEuydQfwcBoIeMotVHIpNzhcIndmUaNiHfYltuJuJNraDdUysIKwBNclJFLUbajVSgVheJbFdPjlTlnafBFAqeKQPvQvgjTLISfYKVmZcfReYOnRRlneMtBuVxCMapyYPZShEWkMsdTIYxLgjEawGXwXvSvUkAvclpOPKSTxrOIfHSMLcmRlfzXlbIPHpmncnfhGWIRfcVsZazehLGuYfVEfjlzuluPxlFKxeYJjmtEzYTVGOAjuMIpUYEwNeFNOeqoyHkJhrzNtsyiYBNHRmlMCZubXmwBRXingWgbYMRyabuABkGUpWQBQLdEtYHyRnCocXgirNamwpkBmdfjuzPzbvxtzdwHwXZrffSSeKPvkRkwLvofZmXBwABRhimRkAiMxFZRKflFdVKDcBjpCBvMZaJVnqyuqxCvfVRWzLijTQAjPIbBBRUTFKeRabpkeUddhfONWuRAhxbghoIKHIYhyvjBxyywIQsvUveuJvZzprbTaBKQJpRBiezwfTAtRDpmitJqPYxWhBhAuvvQjbbyjf"),
    ].into_iter().collect::<std::collections::HashMap<_, _>>();

    // 记录开始时间
    let start = Instant::now();

    // 定义每个循环周期的日志条目数和总的循环次数
    let each_interval_log = log_number;
    let cycles = 10000;

    // 循环生成日志
    for _ in 0..cycles {
        logger.error("This is not log hotspot");
        let begin = Instant::now();
        for _ in 0..each_interval_log {
            if let Some(message) = print_log.get(&length) {
                logger.error(message);
            }
        }
        let elapsed = begin.elapsed();
        if elapsed < Duration::from_millis(100) {
            thread::sleep(Duration::from_millis(100) - elapsed);
        } else {
            logger.error("The benchmark cannot handle so much logs at the interval, please reduce lognumber");
        }
    }

    // 打印总耗时
    println!("Total time taken: {:?}", start.elapsed());
    println!("Rust bench finish");
}