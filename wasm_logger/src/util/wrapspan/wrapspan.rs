use std::fmt;

// 定义一个 Trait，用于将颜色转换为 HTML 颜色字符串
trait ToHTMLColorString {
    fn to_html_color_string(&self) -> String;
}

// 实现 TitleTHCSText 函数
fn title_thcs_text<T: ToHTMLColorString>(title: &str, bi: T, txt: &str) -> String {
    title_color_text(title, bi.to_html_color_string(), txt)
}

// 实现 TitleTHCSTextf 函数
fn title_thcs_textf<T: ToHTMLColorString>(title: &str, bi: T, format: &str, args: &[&dyn fmt::Display]) -> String {
    let txt = format_args!(format, args).to_string();
    title_color_text(title, bi.to_html_color_string(), &txt)
}

// 实现 TitleColorTextf 函数
fn title_color_textf(title: &str, co: &str, format: &str, args: &[&dyn fmt::Display]) -> String {
    let txt = format_args!(format, args).to_string();
    title_color_text(title, co, &txt)
}

// 实现 TitleColorText 函数
fn title_color_text(title: &str, co: &str, txt: &str) -> String {
    format!(r#"<span style="color:{}" title="{}">{}</span>"#, co, title, txt)
}

// 实现 THCSText 函数
fn thcs_text<T: ToHTMLColorString>(bi: T, txt: &str) -> String {
    color_text(bi.to_html_color_string(), txt)
}

// 实现 THCSTextf 函数
fn thcs_textf<T: ToHTMLColorString>(bi: T, format: &str, args: &[&dyn fmt::Display]) -> String {
    let txt = format_args!(format, args).to_string();
    color_text(bi.to_html_color_string(), &txt)
}

// 实现 ColorTextf 函数
fn color_textf(co: &str, format: &str, args: &[&dyn fmt::Display]) -> String {
    let txt = format_args!(format, args).to_string();
    color_text(co, &txt)
}

// 实现 ColorText 函数
fn color_text(co: &str, txt: &str) -> String {
    format!(r#"<span style="color:{}">{}</span>"#, co, txt)
}

// 示例结构体和 ToHTMLColorString 的实现
#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl ToHTMLColorString for Color {
    fn to_html_color_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

// fn main() {
//     // 示例使用
//     let color = Color { r: 255, g: 0, b: 0 };
//     let text = "Hello, World!";
//     let formatted_text = title_thcs_textf("Tooltip", &color, "Hello, {}!", &[&"World"]);

//     println!("{}", formatted_text);
// }