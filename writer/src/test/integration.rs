use std::io::Write;
use std::thread;
use std::fs;
use std::time::Duration;
use crate::*;

// Achtung! Run tests with `cargo test -- --test-threads 1` to avoid files conflicts.

/// Creates start.txt and writes a string ("wrong data") there,
/// then it simulates a message writing,
/// compares COUNTER value with expected value (0)
/// and ensures that start.txt has been deleted.
/// 
/// It is for ensure that the listener is
/// deletes flag file after discovering unexpected value there.
#[test]
fn test_listener_with_wrong_data() {
    fs::remove_file(flag_path()).ok();
    set_default_debuglog_path();
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

/// Creates empty start.txt,
/// then it simulates a message writing
/// and compares COUNTER value with expected value (8).
/// 
/// It is for ensure that the default value is
/// correctly writes to the COUNTER.
#[test]
fn test_listener_with_empty_flag_file() {
    fs::remove_file(flag_path()).ok();
    set_default_debuglog_path();
    std::thread::spawn(move || {
        listen_flag_file();
    });
    let mut file = fs::File::create(flag_path()).unwrap();
    file.write_all(b"").unwrap();
    thread::sleep(Duration::from_millis(WAIT_TIMEOUT_MS + 100));
    assert_eq!(unsafe { COUNTER }, 9);
    write_message("test listener with empty flag file", &String::new());
    assert_eq!(unsafe { COUNTER }, 8);
    unsafe {
        assert!(std::path::Path::new(&format!("{}/message9.txt", DEBUGLOG_PATH)).exists());
        assert_eq!(
            fs::read_to_string(format!("{}/message9.txt", DEBUGLOG_PATH)).unwrap(),
            String::from("test listener with empty flag file: ")
        );
    }
}

/// Creates start.txt and writes 5 there,
/// then it simulates a message writing
/// and compares COUNTER value with expected value (4).
///
/// It is for ensure that the value is correctly
/// reads from start.txt and writes to the COUNTER.
#[test]
fn test_listener_with_value() {
    fs::remove_file(flag_path()).ok();
    set_default_debuglog_path();
    std::thread::spawn(move || {
        listen_flag_file();
    });
    unsafe {
        fs::create_dir_all(&DEBUGLOG_PATH).ok();
    }
    let mut file = fs::File::create(flag_path()).unwrap();
    file.write_all(b"5").unwrap();
    thread::sleep(Duration::from_millis(WAIT_TIMEOUT_MS + 1000));
    write_message("test listener with value", &String::new());
    assert_eq!(unsafe { COUNTER }, 4);
}