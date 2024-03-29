use libc::{wait, WNOHANG};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ClearTerm {
    sniff_clear_term: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Cooldown {
    sniff_cooldown: u128,
}

fn main() {
    let mut last_run: Instant = Instant::now();

    env::set_var("RUST_LOG", "sniff=info");
    let mut args = args();
    if let Some(arg) = args.nth(1) {
        match arg.as_str() {
            "-d" => env::set_var("RUST_LOG", "sniff=trace"),
            _ => {
                println!(
                    "Usage:\nsniff [FLAGS]\nVersion: {:#?}\n\nFlags:\n-d -- debug\n\nAuthors: {:#?}",
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_AUTHORS")
                );
                exit(0);
            }
        }
    }
    env_logger::init();
    log::trace!("Logger initialized.");

    let config_file = match fetch_sniff_config() {
        Some(cfg) => cfg,
        None => {
            log::error!("Failed to read sniff config file. Either it does not exist or the required permissions for reading the file are not available!");
            exit(1);
        }
    };

    let json: serde_json::Value = match serde_json::from_str(config_file.as_str()) {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("Failed to marshal input string to JSON: {:#?}", e);
            exit(1);
        }
    };

    let ignore_list: Option<Ignore> = match serde_json::from_str(config_file.as_str()) {
        Ok(ign) => Some(ign),
        Err(..) => {
            log::debug!("No ignore directives found.");
            None
        }
    };

    let clear_term: Option<ClearTerm> = match serde_json::from_str(config_file.as_str()) {
        Ok(clear) => Some(clear),
        Err(..) => {
            log::debug!("No clear directive found.");
            None
        }
    };

    let cooldown: u128 = match serde_json::from_str(config_file.as_str()) {
        Ok(cooldown) => cooldown,
        Err(..) => 650,
    };

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
                            clear_term.clone(),
                            cooldown,
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

fn fetch_sniff_config() -> Option<String> {
    let config_file_path = Path::new("./sniff.json");
    if config_file_path.exists() {
        return Some(fs::read_to_string(config_file_path).unwrap());
    }

    let config_file_path = match env::var("XDG_CONFIG_HOME") {
        Ok(val) => {
            log::debug!("XDG_CONFIG_HOME exists: {:#?}", val);
            Path::new(&val).join("sniff/sniff.json")
        }
        Err(..) => {
            log::error!(
                "XDG_CONFIG_HOME has not been set. Falling back to ~/.config/sniff/sniff.json"
            );
            Path::new("~/.config/sniff/sniff.json").to_path_buf()
        }
    };

    if !config_file_path.exists() {
        return None;
    }

    if Path::new(&config_file_path).exists() {
        Some(fs::read_to_string(config_file_path).unwrap())
    } else {
        None
    }
}

fn run_system_command(
    command_dir_pairs: Vec<(Vec<String>, Option<PathBuf>)>,
    clear_term: Option<ClearTerm>,
) {
    unsafe {
        let mut status = WNOHANG;
        wait(&mut status);
    }

    if let Some(clear_term) = clear_term {
        if clear_term.sniff_clear_term {
            if let Err(e) = Command::new("clear").spawn() {
                log::error!("Failed to clear terminal: {:#?}", e);
            }
        }
    }

    log::info!("************* Initializing Command Runners! *************");
    for (commands, relative_dir) in command_dir_pairs {
        for command in commands {
            // We need to split the arg apart because it returns a temporary pointer that is free'd after
            // the execution of the declaration line below.
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(command.clone());

            // Dir setting
            if let Some(relative_dir) = relative_dir.clone() {
                cmd.current_dir(relative_dir);
            }

            if let Err(e) = cmd.spawn() {
                log::error!("Failed to execute {}", command);
                log::error!("Error, {}", e);
            }
        }
    }
}

fn check_and_run(
    file_path: &str,
    json: serde_json::Value,
    ignore_list: Option<Ignore>,
    clear_term: Option<ClearTerm>,
    cooldown: u128,
    last_run: &mut Instant,
) {
    // Cooldown check.
    if Instant::now().duration_since(*last_run).as_millis() < cooldown {
        log::debug!("In cooldown.");
        return;
    }

    if let Some(ignore_list) = ignore_list {
        // First the file check.
        for ignore_file in ignore_list.sniff_ignore_file {
            if ignore_file[..] == file_path[file_path.rfind('/').unwrap() + 1..] {
                log::debug!("Ignoring {} as it's in the ingored file list.", file_path);
                return;
            }
        }

        // Now the dir check.
        for ignore_dir in ignore_list.sniff_ignore_dir {
            if file_path[0..file_path.rfind('/').unwrap()].contains(ignore_dir.as_str()) {
                log::debug!(
                    "Ignoring {} as it's in the ingored directory list.",
                    file_path
                );
                return;
            }
        }
    }

    *last_run = Instant::now();

    let mut command_dir_pairs: Vec<(Vec<String>, Option<PathBuf>)> = Vec::new();
    let file_name = match Path::new(file_path).extension() {
        Some(x) => match x.to_str() {
            Some(y) => y,
            None => {
                log::error!("OSstr to str conversion failed!");
                exit(1);
            }
        },
        None => {
            log::error!(
                "{:#?} had no extension. Skipping instruction search.",
                file_path
            );
            return;
        }
    };
    match json {
        serde_json::Value::Object(map) => {
            for (pattern, instructions) in map.iter() {
                if pattern.eq(file_name) {
                    match instructions {
                        serde_json::Value::Object(obj) => {
                            let mut relative_dir: Option<PathBuf> = None;
                            let mut command_strs: Vec<String> = Vec::new();
                            for (key, pair) in obj.iter() {
                                match key.as_str() {
                                    "relative_dir" => {
                                        if let serde_json::Value::String(dir) = pair {
                                            relative_dir = Some(PathBuf::from(dir));
                                        } else {
                                            log::error!(
                                                "Key \"relative_dir\" only takes string values!"
                                            );
                                            exit(1);
                                        }
                                    }
                                    "commands" => {
                                        if let serde_json::Value::Array(commands) = pair {
                                            for command in commands {
                                                match command {
                                                    serde_json::Value::String(command) => {
                                                        command_strs.push(command.replace(
                                                            "%sniff_file_name%",
                                                            file_path,
                                                        ));
                                                    }
                                                    _ => {
                                                        log::error!("Command wasn't a string.");
                                                        exit(1);
                                                    }
                                                }
                                            }
                                        } else {
                                            log::error!("Key \"commands\" only takes arrays!");
                                            exit(1);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            command_dir_pairs.push((command_strs, relative_dir.clone()));

                            log::info!("Running build scripts for {:#?}", file_path);
                            run_system_command(command_dir_pairs.clone(), clear_term.clone());
                        }
                        serde_json::Value::Array(arr) => {
                            let mut commands: Vec<String> = Vec::new();
                            for command in arr {
                                match command {
                                    serde_json::Value::String(command) => {
                                        commands
                                            .push(command.replace("%sniff_file_name%", file_path));
                                    }
                                    _ => {
                                        log::error!(
                                            "Command arrays must be filled with strings only!"
                                        );
                                        exit(1);
                                    }
                                }
                            }

                            run_system_command(vec![(commands, None)], clear_term.clone());
                        }
                        _ => {
                            log::error!("Received incorrect pattern object for {:#?}", pattern);
                            exit(1);
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    };
}
