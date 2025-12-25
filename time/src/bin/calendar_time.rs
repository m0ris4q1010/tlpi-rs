use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};

use lib::exit_success;

const SECONDS_IN_TROPICAL_YEAR: f64 = 365.24219 * (24 * 60 * 60) as f64;

fn main() {
    let t = time();
    print!("Seconds since the Epoch (1 Jan 1970): {}", t);
    println!(
        " (about {:6.3} years)",
        (t as f64) / SECONDS_IN_TROPICAL_YEAR
    );

    let gm = DateTime::from_timestamp(t as i64, 0).expect("now as utc");
    println!("Broken down by gmtime():");
    println!(
        "  year={} mon={} mday={} hour={} min={} sec={} wday={} yday={}",
        gm.year(),
        gm.month(),
        gm.day(),
        gm.hour(),
        gm.minute(),
        gm.second(),
        gm.weekday(),
        gm.ordinal(),
    );

    let loc = gm.with_timezone(&Local);
    println!("Broken down by localtime():");
    println!(
        "  year={} mon={} mday={} hour={} min={} sec={} wday={} yday={}",
        loc.year(),
        loc.month(),
        loc.day(),
        loc.hour(),
        loc.minute(),
        loc.second(),
        loc.weekday(),
        loc.ordinal(),
    );

    println!(
        "asctime() formats the gmtime() value as: {}",
        gm.format("%a %b %e %T %Y")
    );
    println!(
        "mktime() of gmtime() value: {} secs",
        Utc.with_ymd_and_hms(
            gm.year(),
            gm.month(),
            gm.day(),
            gm.hour(),
            gm.minute(),
            gm.second()
        )
        .unwrap()
        .timestamp()
    );

    exit_success();
}

fn time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("`now` must be after `unix epoch`")
        .as_secs()
}
