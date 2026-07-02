#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod hero;
mod rendar;

pub use app::App;
pub use rendar::Rendar;

pub const MAX_LEVEL: u32 = 100;

/// 頭文字だけ大文字にする簡単な関数っ
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
