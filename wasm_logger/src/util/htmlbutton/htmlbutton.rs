use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct HTMLButton {
    key_code: String,
    id_base: String,
    button_text: Vec<String>,
    tool_tip: String,
    click_fn: Option<js_sys::Function>,
    state: usize,
}

#[wasm_bindgen]
impl HTMLButton {
    #[wasm_bindgen(constructor)]
    pub fn new(key_code: String, id_base: String, button_text: Vec<String>, tool_tip: String) -> Self {
        HTMLButton {
            key_code,
            id_base,
            button_text,
            tool_tip,
            click_fn: None,
            state: 0,
        }
    }

    fn js_id(&self) -> String {
        format!("jsid_{}", self.id_base)
    }

    fn js_fn_name(&self) -> String {
        format!("jsfn_{}", self.id_base)
    }

    #[wasm_bindgen]
    pub fn make_js_fn(&mut self, obj: &JsValue) -> js_sys::Function {
        let this = self.clone();
        Closure::wrap(Box::new(move |_: JsValue| {
            this.state = (this.state + 1) % this.button_text.len();
            let btn_str = format!("{}({})", this.button_text[this.state], this.key_code);
            let document = web_sys::window().unwrap().document().unwrap();
            if let Some(element) = document.get_element_by_id(&this.js_id()) {
                element.set_inner_html(&btn_str);
                if let Some(ref click_fn) = this.click_fn {
                    let _ = click_fn.call1(&JsValue::null(), &obj);
                }
            }
        }) as Box<dyn FnMut(JsValue)>).into_js_value()
    }

    #[wasm_bindgen]
    pub fn make_html(&self) -> String {
        let btn_str = format!("{}({})", self.button_text[self.state], self.key_code);
        format!(r#"<button id="{}" title="{}" onclick="{}();">{}</button>"#, self.js_id(), self.tool_tip, self.js_fn_name(), btn_str)
    }

    #[wasm_bindgen]
    pub fn enable(&self) {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(element) = document.get_element_by_id(&self.js_id()) {
            element.remove_attribute("disabled").unwrap();
        }
    }

    #[wasm_bindgen]
    pub fn disable(&self) {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(element) = document.get_element_by_id(&self.js_id()) {
            element.set_attribute("disabled", "true").unwrap();
        }
    }

    #[wasm_bindgen]
    pub fn show(&self) {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(element) = document.get_element_by_id(&self.js_id()) {
            element.style().set_property("display", "initial").unwrap();
        }
    }

    #[wasm_bindgen]
    pub fn hide(&self) {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(element) = document.get_element_by_id(&self.js_id()) {
            element.style().set_property("display", "none").unwrap();
        }
    }

    #[wasm_bindgen]
    pub fn register(&mut self, obj: &JsValue) {
        let closure = self.make_js_fn(obj);
        let window = web_sys::window().unwrap();
        window.set_item(&self.js_fn_name(), &closure.as_ref().unchecked_into());
    }
}

#[wasm_bindgen]
pub struct HTMLButtonGroup {
    name: String,
    button_list: Vec<HTMLButton>,
    id2button: HashMap<String, HTMLButton>,
    key_code2button: HashMap<String, HTMLButton>,
}

#[wasm_bindgen]
impl HTMLButtonGroup {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, buttons: Vec<HTMLButton>) -> Self {
        let mut id2button = HashMap::new();
        let mut key_code2button = HashMap::new();

        for button in &buttons {
            id2button.insert(button.id_base.clone(), button.clone());
            key_code2button.insert(button.key_code.clone(), button.clone());
        }

        HTMLButtonGroup {
            name,
            button_list: buttons,
            id2button,
            key_code2button,
        }
    }

    #[wasm_bindgen]
    pub fn get_by_id_base(&self, idb: String) -> Option<HTMLButton> {
        self.id2button.get(&idb).cloned()
    }

    #[wasm_bindgen]
    pub fn get_by_key_code(&self, kcode: String) -> Option<HTMLButton> {
        self.key_code2button.get(&kcode).cloned()
    }

    #[wasm_bindgen]
    pub fn register(&mut self, obj: &JsValue) {
        for button in &mut self.button_list {
            button.register(obj);
        }
    }

    #[wasm_bindgen]
    pub fn make_html(&self, obj: &JsValue) -> String {
        let mut buf = format!("{}", self.name);
        for button in &self.button_list {
            buf.push_str(&button.make_html());
        }
        buf
    }
}