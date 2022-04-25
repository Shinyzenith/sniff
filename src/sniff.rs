use libc::{wait, WNOHANG};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    env, fs,
    path::Path,
    process::{exit, Command},
    sync::mpsc::channel,
    time::Duration,
};
fn main() {
    env::set_var("RUST_LOG", "sniff=trace");
    env_logger::init();
    log::trace!("Logger initialized.");

    let config_file = fetch_sniff_config_file();
    let json: serde_json::Value =
        serde_json::from_str(config_file.as_str()).expect("JSON was not well-formatted");

    if let Ok(current_dir) = env::current_dir() {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(0)).unwrap();

        if let Err(e) = watcher.watch(current_dir.into_os_string(), RecursiveMode::Recursive) {
            log::error!("Failed to watch current directory: {:#?}", e);
        }

        loop {
            if let Ok(event) = rx.recv() {
                match event {
                    DebouncedEvent::Write(path) => {
                        check_and_run(path.to_str().unwrap(), json.clone());
                    }
                    DebouncedEvent::NoticeWrite(path) => {
                        check_and_run(path.to_str().unwrap(), json.clone());
                    }
                    _ => {}
                }
            } else {
                log::error!("Failed to recieve event from recieve channel");
            }
        }
    } else {
        log::error!("Failed to get current working directory.");
    }
}

fn fetch_sniff_config_file() -> String {
    let config_file_path = Path::new("./sniff.json");
    if config_file_path.exists() {
        return fs::read_to_string(config_file_path).unwrap();
    }

    let config_file_path = match env::var("XDG_CONFIG_HOME") {
        Ok(val) => {
            log::debug!("XDG_CONFIG_HOME exists: {:#?}", val);
            Path::new(&val).join("sniff/sniff.json")
        }
        Err(_) => {
            log::error!("XDG_CONFIG_HOME has not been set.");
            Path::new("~/.config/sniff/sniff.json").to_path_buf()
        }
    };
    if !config_file_path.exists() {
        log::error!("Config file doesn't exist.");
        exit(1);
    }
    fs::read_to_string(config_file_path).unwrap()
}

fn run_system_command(command: &str) {
    println!("{:#?}", command);
    unsafe {
        let mut status = WNOHANG;
        wait(&mut status);
    }
    if let Err(e) = Command::new("sh").arg("-c").arg(command).spawn() {
        log::error!("Failed to execute {}", command);
        log::error!("Error, {}", e);
    } else {
        log::debug!("Ran: {:#?}", command);
    }
}

fn check_and_run(file_name: &str, json: serde_json::Value) {
    match json {
        serde_json::Value::Object(map) => {
            for (patterns, commands) in map.iter() {
                if regex::Regex::new(patterns)
                    .unwrap()
                    .captures(file_name)
                    .is_some()
                {
                    match commands {
                        serde_json::Value::Array(arr) => {
                            for command in arr {
                                match command {
                                    serde_json::Value::String(command) => {
                                        run_system_command(command);
                                    }
                                    _ => {
                                        log::error!("Command wasn't a string.");
                                        exit(1);
                                    }
                                }
                            }
                        }
                        _ => {
                            log::error!("Did not recieve an array of commands.");
                            exit(1);
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    };
}
