use std::fmt::Display;

pub fn check_file_and_write_message<T: Display>(flag_path: &str, description: &str, message: &T) {
	if let Ok(counter_str) = std::fs::read_to_string(format!("{}", flag_path)) {
		let mut counter: u8 = match counter_str.parse() {
			Ok(u) => u,
			Err(_) => 0,
		};
		if counter_str == "" { counter = 9; }
		if counter == 0 {
			std::fs::remove_file(flag_path).ok();
		} else {
			std::fs::write(
				format!("{}/message{}.txt", flag_path, 9 - counter),
				format!("{}: {}", description, message)
			).ok();
			std::fs::write(
				flag_path,
				format!("{}", counter - 1)
			).ok();
		}
	}
}