use std::process::Command;

pub fn set_key_repeat_linux(delay: u64, rate: u64) {
    let output = Command::new("xset")
        .arg("r")
        .arg("rate")
        .arg(delay.to_string())
        .arg(rate.to_string())
        .output()
        .expect("delay and rate should be set");

    if output.status.success() {
        println!("xset command executed successfully");
    } else {
        eprintln!(
            "xset command failed with status: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
