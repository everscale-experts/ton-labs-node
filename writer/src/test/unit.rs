use crate::*;

// Achtung! Run tests with `cargo test -- --test-threads 1` to avoid files conflicts.

// Test write_message() function
#[test]
fn test_message_writing() {
    unsafe {
        COUNTER = 2;
        DEBUGLOG_PATH = String::from("./debugLog");
        assert_eq!(COUNTER, 2);
        write_message("testing write_message", &String::from("message #2"));
        assert_eq!(COUNTER, 1);
        write_message("testing write_message", &String::from("message #1"));
        assert_eq!(COUNTER, 0);
        write_message("testing write_message", &String::from("message #0"));
        assert_eq!(COUNTER, 0);
        for i in 1..=2 {
            assert!(
                std::path::Path::new(&format!("{}/message{}.txt", DEBUGLOG_PATH, i)).exists(),
                "Message #{} was not written", i
            );
            assert_eq!(
                std::fs::read_to_string(format!("{}/message{}.txt", DEBUGLOG_PATH, i)).unwrap(),
                format!("testing write_message: message #{}", i)
            )
        }
        assert!(
            !std::path::Path::new(&format!("{}/message0.txt", DEBUGLOG_PATH)).exists(),
            "Message #0 was written"
        );
    }
}

// Test functions set_path(), flag_path() and set_default_debuglog_path()
#[test]
fn test_flag_path() {
    unsafe {
        DEBUGLOG_PATH = String::new();
        set_path(Some(String::from("./debugLogTest")));
        assert_eq!(DEBUGLOG_PATH, String::from("./debugLogTest"));
        assert_eq!(flag_path(), String::from("./debugLogTest/start.txt"));
        set_default_debuglog_path();
        assert_eq!(DEBUGLOG_PATH, String::from("./debugLog"));
        assert_eq!(flag_path(), String::from("./debugLog/start.txt"));
    }
}