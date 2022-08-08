use std::io::Write;
use std::thread;
use std::fs;
use std::time::Duration;
use crate::*;

#[test]
fn test_listener_with_wrong_data() {
	thread::sleep(Duration::from_millis(WAIT_TIMEOUT_MS * 2 + 2000));
	fs::remove_file(flag_path()).ok();
	std::thread::spawn(move || {
		listen_flag_file();
	});
	let mut file = fs::File::create(flag_path()).unwrap();
	file.write_all(b"wrong data").unwrap();
	thread::sleep(Duration::from_millis(WAIT_TIMEOUT_MS + 1000));
	assert_eq!(unsafe { COUNTER }, 0);
	write_message("test listener with wrong data", &String::new());
	assert!(!std::path::Path::new(&flag_path()).exists(), "file wasn`t deleted");
}

#[test]
fn test_listener_with_empty_flag_file() {
	thread::sleep(Duration::from_millis(WAIT_TIMEOUT_MS + 1500));
	fs::remove_file(flag_path()).ok();
	std::thread::spawn(move || {
		listen_flag_file();
	});
	let mut file = fs::File::create(flag_path()).unwrap();
	file.write_all(b"").unwrap();
	thread::sleep(Duration::from_millis(WAIT_TIMEOUT_MS + 1000));
	write_message("test listener with empty flag file", &String::new());
	assert_eq!(unsafe { COUNTER }, 8);
}

#[test]
fn test_listener_with_value() {
	fs::remove_file(flag_path()).ok();
	std::thread::spawn(move || {
		listen_flag_file();
	});

	let mut file = fs::File::create(flag_path()).unwrap();
	file.write_all(b"5").unwrap();
	thread::sleep(Duration::from_millis(WAIT_TIMEOUT_MS + 1000));
	write_message("test listener with value", &String::new());
	assert_eq!(unsafe { COUNTER }, 4);
}