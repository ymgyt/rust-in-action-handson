use std::{time, thread};

static mut SHUT_DOWN: bool = false;

fn main() {
    register_signal_handlers();

    let delay = time::Duration::from_secs(1);

    for i in 1_usize.. {
        println!("{}", i);
        unsafe {
            if SHUT_DOWN {
                println!("*");
                return;
            }
        }

        thread::sleep(delay);

        let signal = if i > 2 {
            libc::SIGTERM
        } else {
            libc::SIGUSR1
        };
        unsafe {
            libc::raise(signal);
        }
    }
    unreachable!();
}

fn register_signal_handlers() {
    unsafe {
        libc::signal(libc::SIGTERM, handle_sigterm as usize);
        libc::signal(libc::SIGUSR1, handle_sigusr1 as usize);
    }
}

fn handle_sigterm(sig: i32) {
   // register_signal_handlers();
    println!("SIGTERM {}", sig);

    unsafe {
        SHUT_DOWN = true;
    }
}

fn handle_sigusr1(sig: i32) {
    // register_signal_handlers();
    println!("SIGUSR1 {}", sig);
}