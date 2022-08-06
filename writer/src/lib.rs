use std::fmt::Display;

#[allow(deprecated)]
#[cfg(test)]
mod test;

pub(crate) const DEBUGLOG_PATH: &str = "../../debugLog";
pub(crate) const WAIT_TIMEOUT_MS: u64 = 5000;
pub(crate) const FLAG_PATH: &str = "../../debugLog/start.txt";
pub static mut COUNTER: usize = 0;

pub fn listen_flag_file() {
	loop {
		std::thread::sleep(std::time::Duration::from_millis(WAIT_TIMEOUT_MS));
		if let Ok(counter_str) = std::fs::read_to_string(FLAG_PATH) {
			unsafe {
				COUNTER = match counter_str.parse() {
					Ok(u) => u,
					Err(_) => 0,
				};
				if counter_str == "" { COUNTER = 9; }
				std::fs::remove_file(FLAG_PATH).ok();
			}
		}
	}
}

pub fn write_message<T: Display>(description: &str, message: &T) {
	unsafe {
		if COUNTER > 0 {
			std::fs::write(
				format!("../{}/message{}.txt", DEBUGLOG_PATH, COUNTER),
				format!("{}: {}", description, message)
			).ok();
			COUNTER -= 1;
		}
	}
}