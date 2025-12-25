use chrono::{Local, NaiveDateTime};
use clap::Parser;

use lib::{exit_failure, exit_success};

#[derive(Parser)]
struct Cli {
    input_date_time: String,
    in_format: String,
    out_format: Option<String>,
}

fn main() {
    let Cli {
        input_date_time,
        in_format,
        out_format,
    } = Cli::parse();

    let tm = NaiveDateTime::parse_from_str(&input_date_time, &in_format).unwrap_or_else(|e| {
        eprintln!("NaiveDateTime::parse_from_str(): {}", e);
        exit_failure();
    });
    let tm = tm.and_local_timezone(Local).unwrap();

    println!("calendar time (seconds since Epoch): {}", tm.timestamp());

    let out_format = out_format.unwrap_or("%H:%M:%S %A, %d %B %Y %Z".to_string());
    println!("DateTime's format() yields: {}", tm.format(&out_format));

    exit_success();
}
