use std::{
    fmt::Display,
    path::PathBuf,
    fs,
};

#[cfg(test)]
mod test;

pub(crate) const WAIT_TIMEOUT_MS: u64 = 5000;
pub static mut DEBUGLOG_PATH: String = String::new();
pub static mut COUNTER: usize = 0;

pub fn listen_flag_file() {
    unsafe {
        if DEBUGLOG_PATH.is_empty() {
            set_default_debuglog_path();
        }
    }
    println!("Listener started.");
    loop {
        std::thread::sleep(std::time::Duration::from_millis(WAIT_TIMEOUT_MS));
        if let Ok(counter_str) = fs::read_to_string(flag_path()) {
            unsafe {
                println!("Flag catched! Path: {:?}", fs::canonicalize(PathBuf::from(&DEBUGLOG_PATH)));
                COUNTER = match counter_str.parse() {
                    Ok(u) => u,
                    Err(_) => 0,
                };
                if counter_str == "" { COUNTER = 9; }
                fs::remove_file(flag_path()).ok();
                println!("Counter: {}", COUNTER);
            }
        }
    }
}

pub fn write_message<T: Display>(description: &str, message: &T) {
    unsafe {
        println!("Writing message ({}). {} messages remaining", description, COUNTER);
        if COUNTER > 0 {
            if !std::path::Path::new(&DEBUGLOG_PATH).exists() {
                fs::create_dir_all(&DEBUGLOG_PATH).ok();
            }
            fs::write(
                format!("{}/message{}.txt", DEBUGLOG_PATH, COUNTER),
                format!("{}: {}", description, message)
            ).ok();
            COUNTER -= 1;
        }
    }
}

pub(crate) fn flag_path() -> String {
    unsafe {
        format!("{}/start.txt", DEBUGLOG_PATH)
    }
}

pub fn set_path(debuglog_path: Option<String>) {
    if let Some(path) = debuglog_path {
        unsafe {
            if !std::path::Path::new(&path).exists() {
                fs::create_dir_all(&path).ok();
            }
            DEBUGLOG_PATH = path.clone();
        }
    }
}

pub fn set_default_debuglog_path() {
    unsafe {
        DEBUGLOG_PATH = String::from("./debugLog");
    }
}