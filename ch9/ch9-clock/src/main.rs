use chrono::Local;
use chrono::{DateTime, TimeZone};
use clap::{App, Arg};
use std::mem::zeroed;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
    }
}

#[cfg(not(windows))]
fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
    use libc::settimeofday;
    use libc::{suseconds_t, time_t, timeval, timezone};

    let t = t.with_timezone(&Local);
    let mut u: timeval = unsafe { zeroed() };

    u.tv_sec = t.timestamp() as time_t;
    u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

    unsafe {
        let mock_tz: *const timezone = std::ptr::null();
        settimeofday(&u as *const timeval, mock_tz);
    }
}

fn main() {
    let app = App::new("clock")
        .version("0.1")
        .about("Gets and sets (aspirationally) the time.")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("use-standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(
            Arg::with_name("datetime")
                .help("When <action> is 'set', apply <datetime>. Otherwise, ignore."),
        );

    let args = app.get_matches();

    let action = args.value_of("std").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        unimplemented!()
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
