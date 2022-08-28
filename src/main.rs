use chrono::Local;
use std::{process::Command, thread, time::Duration};

fn main() {
    loop {
        let system_info = format!("{}{}{}", username(), date(), volume());

        // sleep for 500 milliseconds
        thread::sleep(Duration::from_millis(250));

        // show status in bar
        Command::new("xsetroot")
            .arg("-name")
            .arg(system_info.trim())
            .output()
            .expect("Failed to show bar! Have you got xsetroot installed?");
    }
}

// username
fn username() -> String {
    match Command::new("whoami").output() {
        Ok(output) => {
            format!("[{}] ", String::from_utf8(output.stdout).unwrap().trim())
        }
        Err(_) => String::new(), // should it give an error, let's just not show the username
    }
}

// Date
fn date() -> String {
    let dt = Local::now();
    let date = dt.format("%a %b %e").to_string();
    let time = dt.format("%H:%M:%S").to_string();

    format!("[{}] [{}] ", date, time)
}

// Volume
fn volume() -> String {
    match Command::new("amixer").arg("sget").arg("Master").output() {
        Ok(output) => {
            let s = String::from_utf8(output.stdout)
                .unwrap()
                .trim()
                .lines()
                .filter(|l| l.contains("Front Left"))
                .collect::<String>();
            format!("{}", s.rsplit(' ').collect::<Vec<&str>>().get(1).unwrap())
        }
        Err(_) => String::new(),
    }
}
