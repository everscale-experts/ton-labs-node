use std::fmt::Display;

const WAIT_TIMEOUT_MS: u64 = 5000;
const DEBUGLOG_PATH: &str = "../../debugLog";
pub static mut COUNTER: usize = 0;

pub unsafe fn listen_flag_file() {
	std::thread::spawn(move || {
		let flag_file_path = format!("../{}/start.txt", DEBUGLOG_PATH);
		loop {
			let flag_file_path = flag_file_path.as_str();
			std::thread::sleep(std::time::Duration::from_millis(WAIT_TIMEOUT_MS));
			if let Ok(counter_str) = std::fs::read_to_string(flag_file_path) {
				COUNTER = match counter_str.parse() {
					Ok(u) => u,
					Err(_) => 0,
				};
				if counter_str == "" { COUNTER = 9; }
				std::fs::remove_file(flag_file_path).ok();
			}
		}
	});
}

pub fn write_message<T: Display>(description: &str, message: &T) {
	unsafe {
		if COUNTER > 0 {
			std::fs::write(
				format!("../{}/message{}.txt", DEBUGLOG_PATH, COUNTER),
				format!("{}: {}", description, message)
			).ok();
		}
	}
}

#[deprecated]
pub fn check_file_and_write_message<T: Display>(
	flag_path: &str, // path without start.txt
	description: &str,
	message: &T
) {
	let flag_file_path = &format!("../{}/start.txt", flag_path);
	if let Ok(counter_str) = std::fs::read_to_string(flag_file_path) {
		let mut counter: usize = match counter_str.parse() {
			Ok(u) => u,
			Err(_) => 0,
		};
		if counter_str == "" { counter = 9; }
		if counter == 0 {
			std::fs::remove_file(flag_file_path).ok();
		} else {
			std::fs::write(
				// format!("{}/message{}.txt", flag_path, 9 - counter),
				format!("../{}/message{}.txt", flag_path, counter),
				format!("{}: {}", description, message)
			).ok();
			std::fs::write(
				flag_file_path,
				format!("{}", counter - 1)
			).ok();
		}
	}
}

#[allow(deprecated)]
#[cfg(test)]
mod tests {
	use std::io::Write;
	use std::fs;
	use crate::check_file_and_write_message;
	const FLAG_PATH: &str = "../../debugLog/start.txt";
	
	#[test]
	fn test_without_file() {
		fs::remove_file(FLAG_PATH).ok();
		let res_before = fs::read_to_string(FLAG_PATH);
		check_file_and_write_message("../debugLog", "", &String::new());
		let res_after = fs::read_to_string(FLAG_PATH);
		assert!(res_before.is_err() && res_after.is_err());
		assert!(match res_after {
			Ok(_) => false,
			Err(_) => true
		});
	}

	#[test]
	fn test_empty() {
		fn clear_file(create: bool) {
			let mut file = if create {
				fs::create_dir("../../debugLog").ok();
				fs::File::create(FLAG_PATH)
			} else {
				fs::File::open(FLAG_PATH)
			}.unwrap();
			file.write_all(b"").unwrap();
		}
		match fs::read_to_string(FLAG_PATH) {
			Ok(v) => if v != "" { clear_file(false) },
			Err(_) => clear_file(true)
		}
		assert!(std::path::Path::new(FLAG_PATH).exists());
		check_file_and_write_message("../debugLog", "", &String::new());
		let counter = std::fs::read_to_string(FLAG_PATH).ok().unwrap_or("".to_string());
		assert!(counter.parse::<usize>().is_ok() || counter == "", "file content is not a number: '{}'", counter);
	}

	#[test]
	fn test_with_value() {
		fn write_value(v: usize) -> usize {
			fs::create_dir("../../debugLog").ok();
			let mut file = fs::File::create(FLAG_PATH).unwrap();
			file.write(format!("{}", v).as_bytes()).unwrap();
			v
		}

		let counter = match fs::read_to_string(FLAG_PATH) {
			Ok(v) => v.parse().ok().unwrap_or(write_value(15)),
			Err(_) => write_value(15),
		};

		for _ in 0..counter + 1 {
			check_file_and_write_message("../debugLog", "", &String::new());
		}

		assert!(!std::path::Path::new(FLAG_PATH).exists(), "file wasn`t deleted");
	}
}