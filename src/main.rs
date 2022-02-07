use chrono::Local;
use std::{process::Command, thread, time::Duration};
use sysinfo::{NetworkExt, System, SystemExt};

fn main() {
    // The final string that'll be shown in the dwm bar.
    let mut system_info: String = String::new();
    loop {
        // Systeminfo - gets the infos
        let mut sys = System::new_all();
        system_info = String::new(); // refresh the string
                                     // TODO update system_info

        // show system_info
        system_name(&mut system_info, &sys);
        // separator(&mut system_info);
        // network(&mut system_info, &sys);
        separator(&mut system_info);
        memory(&mut system_info, &sys);
        separator(&mut system_info);
        volume(&mut system_info);
        separator(&mut system_info);
        date(&mut system_info);

        // sleep for 500 milliseconds
        thread::sleep(Duration::from_millis(1000));

        // show status in bar
        Command::new("xsetroot")
            .arg("-name")
            .arg(system_info)
            .output()
            .expect("Failed to show bar!");
    }
}

// Separator
fn separator(system_info: &mut String) {
    system_info.push_str(" ");
}

// Network
fn network(system_info: &mut String, sys: &System) {
    let mut to_push = String::new();
    for (_, data) in sys.networks() {
        to_push = format!(
            "[{} down {} up]",
            data.total_packets_received(),
            data.total_packets_transmitted(),
        );
    }

    system_info.push_str(&to_push);
}

// System Name
fn system_name(system_info: &mut String, sys: &System) {
    if let Some(name) = sys.name() {
        system_info.push_str(&format!("[{}]", name));
    }
}

// Memory
fn memory(system_info: &mut String, sys: &System) {
    system_info.push_str(&format!(
        "[{} / {} gb]",
        (sys.used_memory() / 10000) as f32 / 100.0,
        (sys.total_memory() / 10000) as f32 / 100.0
    ));
}

// Sound
fn volume(system_info: &mut String) {}

// Date
fn date(system_info: &mut String) {
    let dt = Local::now();
    let date = dt.format("%a %b %e").to_string();
    let time = dt.format("%H:%M:%S").to_string();

    system_info.push_str(&format!("[{}] [{}]", date, time));
}
