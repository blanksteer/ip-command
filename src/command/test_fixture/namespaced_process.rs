use std::env;
use std::thread;
use std::time::Duration;
use std::process;

fn main() {
    println!("{}", process::id());

    let args: Vec<String> = env::args().collect();
    let sleep_duration: u64 = args[1].parse().unwrap();
    thread::sleep(Duration::from_secs(sleep_duration));
}