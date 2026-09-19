use colored::{Color, Colorize};
use std::{fs::OpenOptions, io::{self, Write}, path::PathBuf, process::{Command, Stdio}};

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

#[cfg(windows)]
fn configure_detached(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    const DETACHED_PROCESS: u32 = 0x00000008;
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
    command.creation_flags(
        DETACHED_PROCESS |
        CREATE_NEW_PROCESS_GROUP
    );
}

#[cfg(unix)]
fn configure_detached(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {return Err(io::Error::last_os_error());}
            Ok(())
        });
    }
}

pub fn launch(
    program: &str,
    args: &[String],
) -> io::Result<()> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    configure_detached(&mut command);
    command.spawn()?;
    Ok(())
}