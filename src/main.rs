use chrono::Local;
use std::{process::Command, thread, time::Duration};

fn main() {
    thread::sleep(Duration::from_millis(1000));

    loop {
        let system_info = format!("{}{}{}{}", username(), date(), battery(), volume());

        // sleep for 500 milliseconds
        thread::sleep(Duration::from_millis(500));

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
            format!(" {} ", String::from_utf8(output.stdout).unwrap().trim())
        }
        Err(_) => String::new(), // should it give an error, let's just not show the username
    }
}

// Date
fn date() -> String {
    let dt = Local::now();
    let date = dt.format(" %a %b %e").to_string();
    let time = dt.format(" %H:%M").to_string();

    format!("{} {} ", date, time)
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
            let volume = s
                .rsplit(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let x: &[_] = &['[', ']'];

            if let Some(percent) = volume.get(1) {
                format!(" {}", percent.trim_matches(x))
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

// Battery
fn battery() -> String {
    match Command::new("acpi").arg("-b").output() {
        Ok(output) => {
            let s = String::from_utf8(output.stdout).unwrap();
            if !s.contains("No support") {
                let percent = s
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                match percent.get(3) {
                    Some(p) => {
                        let charge = p.chars().filter(|c| c.is_numeric()).collect::<String>();
                        format!(" {}% ", charge)
                    }
                    None => String::new(),
                }
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}
