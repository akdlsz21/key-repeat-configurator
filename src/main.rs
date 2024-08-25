#[allow(dead_code, unused_imports)]
use clap::Parser;

mod platform;

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

    println!("{:?}", args);
    println!("Delay: {} ms", delay);
    println!("Rate: {} ms", rate);

    platform::windows::set_key_repeat_windows(delay, rate);
    // if cfg!(target_os = "linux") {
    //     platform::linux::set_key_repeat_linux(delay, rate);
    // } else if cfg!(target_os = "windows") {
    //     platform::windows::set_key_repeat_windows(delay, rate);
    // } else {
    //     eprintln!("Unsupported operating system");
    // }
}
