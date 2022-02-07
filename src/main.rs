use chrono::Local;
use std::{process::Command, thread, time::Duration};
use sysinfo::{NetworkExt, RefreshKind, System, SystemExt};

fn main() {
    // The final string that'll be shown in the dwm bar.
    loop {
        // Systeminfo - gets the infos
        // let sys = System::new_with_specifics(RefreshKind::new().with_memory().with_network());
        let sys = System::new_with_specifics(RefreshKind::new().with_memory());
        let mut system_info: String = String::new();

        // show system_info
        system_name(&mut system_info, &sys);
        // separator(&mut system_info);
        // network(&mut system_info, &sys);
        memory(&mut system_info, &sys);
        volume(&mut system_info);
        date(&mut system_info);

        // sleep for 1000 milliseconds
        thread::sleep(Duration::from_millis(1000));

        // show status in bar
        Command::new("xsetroot")
            .arg("-name")
            .arg(system_info)
            .output()
            .expect("Failed to show bar! Have you got xsetroot installed?");
    }
}

// Network
// fn network(system_info: &mut String, sys: &System) {
//     let mut to_push = String::new();

//     for (_, data) in sys.networks() {
//         to_push = format!(
//             "[{} down {} up]",
//             data.total_packets_received(),
//             data.total_packets_transmitted(),
//         );
//     }

//     system_info.push_str(&to_push);
// }

// System Name
fn system_name(system_info: &mut String, sys: &System) {
    if let Some(name) = sys.name() {
        system_info.push_str(&format!(" [{}] ", name));
    }
}

// Memory
fn memory(system_info: &mut String, sys: &System) {
    system_info.push_str(&format!(
        " [{} / {} gb] ",
        (sys.used_memory() / 10000) as f32 / 100.0,
        (sys.total_memory() / 10000) as f32 / 100.0
    ));
}

// Sound
fn volume(system_info: &mut String) {
    let mut args = std::env::args();
    if args.len() == 1 || args.len() > 2 {
        eprintln!("[error] pass in a path to the volume script getting the volume");
        std::process::exit(1);
    }

    if let Ok(volume) = std::str::from_utf8(
        &Command::new("bash")
            .arg(args.nth(1).unwrap())
            .output()
            .expect("Failed to get volume.")
            .stdout,
    ) {
        system_info.push_str(&format!(" [{}%] ", volume.trim()));
    }
}

// Date
fn date(system_info: &mut String) {
    let dt = Local::now();
    let date = dt.format("%a %b %e").to_string();
    let time = dt.format("%H:%M:%S").to_string();

    system_info.push_str(&format!(" [{}] [{}] ", date, time));
}
