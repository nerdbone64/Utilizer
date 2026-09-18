use colored::{Color, Colorize};
use std::{fs::OpenOptions, io::Write, path::PathBuf};

pub fn root_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

pub fn utilizer_dir() -> PathBuf {
    root_dir().join(".utilizer")
}

pub fn log(msg: &str, level: i32) {
    let (color, lvl) = match level {
        0 => (Color::Blue, "DEBUG"),
        1 => (Color::Green, "INFO"),
        2 => (Color::Yellow, "WARNING"),
        3 => (Color::Red, "ERROR"),
        _ => (Color::Magenta, "UNKNOWN"),
    };
    println!("[{}]: {msg}", lvl.color(color));
    let path = utilizer_dir().join("output.log");
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "[{lvl}]: {msg}");
    }
}
