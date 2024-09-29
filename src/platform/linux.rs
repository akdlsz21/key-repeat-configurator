use std::env;
use std::process::Command;

pub fn set_key_repeat_linux(delay: u64, rate: u64) {
    let xdg_output = Command::new("sh")
        .arg("-c")
        .arg("echo $XDG_SESSION_TYPE")
        .output()
        .expect("Failed to check XDG_SESSION_TYPE");

    let session_type = String::from_utf8_lossy(&xdg_output.stdout)
        .trim()
        .to_string();

    let is_wayland = session_type == "wayland";
    let is_x11 = session_type == "x11";

    if is_x11 {
        let output = Command::new("xset")
            .arg("r")
            .arg("rate")
            .arg(delay.to_string())
            .arg(rate.to_string())
            .output()
            .expect("Failed to execute xset command");

        if output.status.success() {
            println!("xset command executed successfully");
        } else {
            eprintln!(
                "xset command failed with error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    } else if is_wayland {
        // Ensure the environment variable is set for DBUS
        if let Ok(dbus_address) = env::var("DBUS_SESSION_BUS_ADDRESS") {
            let gsettings_interval = Command::new("gsettings")
                .arg("set")
                .arg("org.gnome.desktop.peripherals.keyboard")
                .arg("repeat-interval")
                .arg(rate.to_string())
                .env("DBUS_SESSION_BUS_ADDRESS", &dbus_address)
                .output()
                .expect("Failed to set repeat-interval");

            let gsettings_delay = Command::new("gsettings")
                .arg("set")
                .arg("org.gnome.desktop.peripherals.keyboard")
                .arg("delay")
                .arg(delay.to_string())
                .env("DBUS_SESSION_BUS_ADDRESS", &dbus_address)
                .output()
                .expect("Failed to set delay");
            println!("DBUS_SESSION_BUS_ADDRESS: {}", dbus_address);
            if gsettings_interval.status.success() && gsettings_delay.status.success() {
                println!("gsettings command executed successfully");
            } else {
                eprintln!(
                    "gsettings command failed with error: {}",
                    String::from_utf8_lossy(&gsettings_interval.stderr)
                );
            }
        } else {
            println!("DBUS_SESSION_BUS_ADDRESS is not set or accessible.");
        }
    } else {
        println!("Unknown display server: {}", session_type);
    }
}
