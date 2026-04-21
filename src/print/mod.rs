mod default;
mod minimal;
mod modern;

use crate::{
    cache::Cache,
    config::OutputStyle,
    print::{default::print_default, minimal::print_minimal, modern::print_modern},
};

const PB_COLUMNS: &[&str] = &["mode", "wpm", "raw", "acc", "cons"];
const SUB: (u8, u8, u8) = (100, 102, 105); // #646669
const ACCENT: (u8, u8, u8) = (226, 183, 20); // #e2b714
const BANNER_SEPARATOR: &str = "·";

pub fn print_user_data(cache: Cache, style: OutputStyle) {
    match style {
        OutputStyle::Default => print_default(cache),
        OutputStyle::Modern => print_modern(cache),
        OutputStyle::Minimal => print_minimal(cache),
    }
}
