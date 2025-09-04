use std::io::{Write, stdout};

use serde::Serialize;

pub fn print_string(printable: String) {
    let _ = stdout().write(printable.as_bytes());
    let _ = stdout().flush();
}

pub fn print<T: Serialize>(printable: &T) {
    let string = serde_yaml::to_string(printable).unwrap();
    let _ = stdout().write(string.as_bytes());
    let _ = stdout().flush();
}
