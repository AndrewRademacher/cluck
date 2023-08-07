use std::{process::Stdio, thread::sleep, time::Duration};

use chrono::Utc;

use subprocess::Exec;

fn main() {
    loop {
        println!("The time is {}", Utc::now());
        sleep(Duration::from_secs(1));
    }
}
