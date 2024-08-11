use chrono::{prelude::Local, Timelike};

// Convert XX:XX:XX to Vec<u8> (24h)
pub fn get_display_array() -> Vec<u8> {
    let now = Local::now();
    let v = vec![
        (now.hour() / 10) as u8,
        (now.hour() % 10) as u8,
        10,
        (now.minute() / 10) as u8,
        (now.minute() % 10) as u8,
        10,
        (now.second() / 10) as u8,
        (now.second() % 10) as u8,
    ];

    v
}
