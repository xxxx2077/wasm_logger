// Copyright 2023 SeukWon Kang (kasworld@gmail.com)
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//    http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use js_sys::JsString;
use web_sys::{console, window};
use wasm_bindgen::prelude::*;
use hex;

#[wasm_bindgen]
pub fn set_cookie(name: &str, value: &str) {
    let encoded_value = hex::encode(value);
    let cookie_str = format!("{}={}; path=/", name, encoded_value);
    window()
        .unwrap()
        .document()
        .unwrap()
        .set_cookie(&cookie_str)
        .expect("Failed to set cookie");
}

#[wasm_bindgen]
pub fn get_cookie_map() -> JsValue {
    let document = window().unwrap().document().unwrap();
    let cookie_str = document.cookie().unwrap_or_default();
    let cookies: Vec<&str> = cookie_str.split(';').collect();

    let mut cookie_map = js_sys::Object::new();

    for cookie in cookies {
        let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
        if parts.len() != 2 {
            console::error_1(&JsString::from(format!("Invalid cookie: {}", cookie)));
            continue;
        }

        let name = parts[0].trim();
        let value = match hex::decode(parts[1].trim()) {
            Ok(decoded) => decoded,
            Err(e) => {
                console::error_2(&JsString::from("Failed to decode cookie value"), &JsString::from(e.to_string()));
                continue;
            }
        };

        js_sys::Reflect::set(&cookie_map, &JsString::from(name), &JsString::from(String::from_utf8_lossy(&value)))
            .expect("Failed to set property in cookie map");
    }

    cookie_map.into()
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // 初始化日志记录
    console_error_panic_hook::set_once();
    Ok(())
}