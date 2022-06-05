use libc::{wait, WNOHANG};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::{
    env,
    env::args,
    fs,
    path::Path,
    process::{exit, Command},
    sync::mpsc::channel,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Ignore {
    sniff_ignore_dir: Vec<String>,
    sniff_ignore_file: Vec<String>,
    sniff_cooldown: u128,
}

fn main() {
    let mut last_run: Instant = Instant::now();

    env::set_var("RUST_LOG", "sniff=warn");
    let mut args = args();
    if let Some(arg) = args.nth(1) {
        match arg.as_str() {
            "-d" => env::set_var("RUST_LOG", "sniff=trace"),
            _ => {
                println!("Usage:\nsniff [FLAGS]\n\nFlags:\n-d -- debug",);
                exit(1);
            }
        }
    }
    env_logger::init();
    log::trace!("Logger initialized.");

    let config_file = fetch_sniff_config_file();
    let json: serde_json::Value =
        serde_json::from_str(config_file.as_str()).expect("JSON was not well-formatted");

    let ignore_list: Ignore =
        serde_json::from_str(config_file.as_str()).expect("JSON was not well-formatted");

    if let Ok(current_dir) = env::current_dir() {
        let (tx, rx) = channel();
        log::debug!("Created tx and rx channels as mpsc.");
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(0)).unwrap();
        log::debug!("Created watcher.");

        if let Err(e) = watcher.watch(current_dir.into_os_string(), RecursiveMode::Recursive) {
            log::error!("Failed to watch current directory: {:#?}", e);
        }

        loop {
            if let Ok(event) = rx.recv() {
                match event {
                    DebouncedEvent::Write(path) | DebouncedEvent::NoticeWrite(path) => {
                        log::debug!("Received Write event: {:?}", path);
                        check_and_run(
                            path.to_str().unwrap(),
                            json.clone(),
                            ignore_list.clone(),
                            &mut last_run,
                        );
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

fn check_and_run(
    file_name: &str,
    json: serde_json::Value,
    ignore_list: Ignore,
    last_run: &mut Instant,
) {
    // First the file check.
    for ignore_file in ignore_list.sniff_ignore_file {
        if ignore_file[..] == file_name[file_name.rfind('/').unwrap() + 1..] {
            log::debug!("Ignoring {} as it's in the ingored file list.", file_name);
            return;
        }
    }

    // Now the dir check.
    for ignore_dir in ignore_list.sniff_ignore_dir {
        if file_name[0..file_name.rfind('/').unwrap()].contains(ignore_dir.as_str()) {
            log::debug!(
                "Ignoring {} as it's in the ingored directory list.",
                file_name
            );
            return;
        }
    }

    // Cooldown check.
    if Instant::now().duration_since(*last_run).as_millis() < ignore_list.sniff_cooldown {
        log::debug!("In cooldown.");
        return;
    }
    *last_run = Instant::now();

    match json {
        serde_json::Value::Object(map) => {
            for (pattern, commands) in map.iter() {
                if regex::Regex::new(pattern)
                    .unwrap()
                    .captures(file_name)
                    .is_some()
                {
                    log::debug!("Found a pattern match!");
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
