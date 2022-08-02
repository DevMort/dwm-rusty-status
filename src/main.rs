use chrono::Local;
use std::io;
use std::{process::Command, thread, time::Duration};

fn main() {
    loop {
        // sleep for 500 milliseconds
        thread::sleep(Duration::from_millis(500));

        // show status in bar
        Command::new("xsetroot")
            .arg("-name")
            .arg(system_info)
            .output()
            .expect("Failed to show bar! Have you got xsetroot installed?");
    }
}

// username
fn username() -> Result<String, io::Error> {
    match Command::new("whoami").output() {
        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
        Err(e) => Err(e),
    }
}

// Date
fn date() -> String {
    let dt = Local::now();
    let date = dt.format("%a %b %e").to_string();
    let time = dt.format("%H:%M:%S").to_string();

    format!(" [{}] [{}] ", date, time)
}
