use std::{thread::sleep, time::Duration};

use chrono::Utc;

fn main() {
    loop {
        println!("The time is {}", Utc::now());
        sleep(Duration::from_secs(1));
    }
}
