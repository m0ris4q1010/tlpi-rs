use std::env;

use chrono::Utc;
use chrono_tz::Tz;

use lib::exit_success;

fn main() {
    let default_tz = Tz::Asia__Tokyo;
    let tz = match env::var("TZ") {
        Ok(tz) => tz.parse().unwrap_or(default_tz),
        Err(_) => default_tz,
    };
    let dt = Utc::now().with_timezone(&tz);

    println!("asctime() of local time is: {}", dt.format("%c"));
    println!(
        "strftime() of local time is: {}",
        dt.format("%A, %d %B %Y, %H:%M:%S %Z")
    );

    exit_success();
}
