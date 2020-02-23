use std::time;
use std::process;
use std::thread;

fn main() {
    let delay = time::Duration::from_secs(1);

    let pid = process::id();
    println!("{}", pid);

    for i in 1..=60 {
        thread::sleep(delay);
        println!("{}", i);
    }
}
