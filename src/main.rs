#[allow(dead_code, unused_imports)]
use clap::Parser;
use std::process::Command;

const NAME: &str = "Key Repeat Configurator";
const VER: &str = "0.1.0";
const AUTHOR: &str = "Jihoon Choi";
const ABOUT: &str =
    "Adjusts the key repeat initial and repeat rates across Linux, Mac, and Windows systems.";
const D_V: &str = "MILLISECONDS";
const R_V: &str = "Repitions per second";

// my current config is 160 30;

#[derive(Parser, Debug)]
#[command(name = NAME, version = VER, author = AUTHOR, about = ABOUT)]
struct Args {
    /// The initial delay before the key starts repeating.
    #[arg(short = 'd', long = "delay", value_name = D_V, default_value = "160")]
    delay: Option<u64>,

    #[arg(short = 'r', long = "rate", value_name =R_V, default_value = "30" )]
    /// the rate at which the key repeats (repition per second).
    rate: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let delay = args.delay.unwrap();
    let rate = args.rate.unwrap();
    // {delay, rate} get vals();
    println!("{:?}", args);
    println!("Delay: {} ms", delay);
    println!("Rate: {} ms", rate);

    if cfg!(target_os = "linux") {
        set_key_repeat_linux(delay, rate);
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        set_key_repeat_windows(delay, rate);
    } else {
        eprintln!("Unsupported operating system");
    }
}

fn set_key_repeat_linux(delay: u64, rate: u64) {
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

#[cfg(target_os = "windows")]
fn set_key_repeat_windows(delay: u64, rate: u64) {
    use std::ptr;
    use winapi::um::winuser::SystemParametersInfoW;
    use winapi::um::winuser::{SPI_SETKEYBOARDDELAY, SPI_SETKEYBOARDSPEED};

    let delay_value = match delay {
        250 => 0,
        500 => 1,
        750 => 2,
        1000 => 3,
        _ => ((delay - 250) / 250).min(3).max(0) as u32,
    };

    // Convert rate from CPS to the Windows rate value (0-31)
    let rate_cps = rate as f64;
    let rate_value = (31.0 - ((rate_cps - 2.5) / 27.5 * 31.0)).round() as u32;

    unsafe {
        let result_delay =
            SystemParametersInfoW(SPI_SETKEYBOARDDELAY, delay_value, ptr::null_mut(), 0);

        let result_rate =
            SystemParametersInfoW(SPI_SETKEYBOARDSPEED, rate_value, ptr::null_mut(), 0);

        if result_delay != 0 && result_rate != 0 {
            println!("Keyboard repeat settings updated successfully");
        } else {
            eprintln!("Failed to update keyboard repeat settings");
        }
    }
}
