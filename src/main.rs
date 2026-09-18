use crate::util::{log, utilizer_dir};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

mod api;
mod util;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    order: Vec<String>,
    skip_if_fail: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            order: vec![],
            skip_if_fail: false,
        }
    }
}

fn setup() -> io::Result<()> {
    let root = utilizer_dir();
    fs::create_dir_all(root.join("scripts"))?;

    let settings = root.join("settings.json");
    if !settings.exists() {
        let text = serde_json::to_string_pretty(&Settings::default())
            .map_err(io::Error::other)?;
        fs::write(settings, format!("{text}\n"))?;
    }

    let log_file = root.join("output.log");
    if !log_file.exists() {
        fs::File::create(log_file)?;
    }

    Ok(())
}

fn load_settings() -> Result<Settings, String> {
    let path = utilizer_dir().join("settings.json");
    let text = fs::read_to_string(&path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|e| format!("failed to parse {}: {e}", path.display()))
}

fn script_path(name: &str) -> std::path::PathBuf {
    let name = if Path::new(name).extension().is_some() {
        name.to_string()
    } else {
        format!("{name}.lua")
    };
    utilizer_dir().join("scripts").join(name)
}

fn run() -> Result<(), String> {
    setup().map_err(|e| format!("failed to initialize .utilizer: {e}"))?;
    log("booting...", 0);

    let settings = load_settings()?;
    let lua = api::new_lua();

    for name in &settings.order {
        let path = script_path(name);
        log(&format!("running {}...", path.display()), 1);

        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(e) => {
                let msg = format!("failed to read {}: {e}", path.display());
                log(&msg, 3);
                if settings.skip_if_fail { continue; }
                return Err(msg);
            }
        };

        let chunk_name = path.to_string_lossy();
        if let Err(e) = lua.load(&source).set_name(chunk_name.as_ref()).exec() {
            let msg = format!("script {} failed: {e}", path.display());
            log(&msg, 3);
            if settings.skip_if_fail { continue; }
            return Err(msg);
        }
    }

    log("finished.", 1);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("[ERROR]: {e}");
        std::process::exit(1);
    }
}
