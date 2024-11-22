use std::fmt;

/// Represents a text with a count.
#[derive(Debug, Clone)]
struct TextNCount {
    text: String,
    count: i32,
}

impl TextNCount {
    /// Creates a new `TextNCount` instance.
    fn new(text: String, count: i32) -> Self {
        TextNCount { text, count }
    }
}

impl fmt::Display for TextNCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.count != 1 {
            write!(f, "{} ({})", self.text, self.count)
        } else {
            write!(f, "{}", self.text)
        }
    }
}

/// A list of `TextNCount` items.
#[derive(Debug, Default)]
struct TextNCountList {
    items: Vec<TextNCount>,
}

impl TextNCountList {
    /// Appends a formatted string to the list.
    fn appendf(&mut self, format: &str, args: &[&dyn fmt::Display]) {
        let s = format_args!(format, args);
        self.append(s.to_string());
    }

    /// Appends a string to the list.
    fn append(&mut self, txt: String) {
        let ti = TextNCount::new(txt, 1);
        if let Some(last) = self.items.last_mut() {
            if last.text == ti.text {
                last.count += 1;
                return;
            }
        }
        self.items.push(ti);
    }

    /// Clears the list.
    fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns the last item in the list.
    fn last(&self) -> Option<&TextNCount> {
        self.items.last()
    }

    /// Returns the last N items in the list.
    fn get_last_n(&self, n: usize) -> Vec<&TextNCount> {
        let l = self.items.len();
        if l <= n {
            self.items.iter().collect()
        } else {
            self.items[(l - n)..].iter().collect()
        }
    }

    /// Converts the list to an HTML string.
    fn to_html_string(&self) -> String {
        let mut buf = String::new();
        for item in &self.items {
            if item.count != 1 {
                writeln!(buf, "{} ({})<br/>", item.text, item.count).unwrap();
            } else {
                writeln!(buf, "{}<br/>", item.text).unwrap();
            }
        }
        buf
    }

    /// Converts the list to an HTML string in reverse order.
    fn to_html_string_rev(&self) -> String {
        let mut buf = String::new();
        for item in self.items.iter().rev() {
            if item.count != 1 {
                writeln!(buf, "{} ({})<br/>", item.text, item.count).unwrap();
            } else {
                writeln!(buf, "{}<br/>", item.text).unwrap();
            }
        }
        buf
    }
}

// 示例用法
fn main() {
    let mut list = TextNCountList::default();
    list.append("Hello".to_string());
    list.append("World".to_string());
    list.append("Hello".to_string());

    println!("{}", list.to_html_string());
    println!("{}", list.to_html_string_rev());

    list.clear();
    list.appendf("{} {}!", &["Hello", "World"]);
    println!("{}", list.to_html_string());
}