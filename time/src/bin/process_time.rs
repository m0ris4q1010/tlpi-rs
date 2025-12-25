use clap::Parser;
use nix::{
    sys::resource::{UsageWho, getrusage},
    time::{ClockId, clock_gettime},
    unistd::getppid,
};

use lib::{err_exit, exit_success};

#[derive(Parser)]
struct Cli {
    num_calls: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    display_process_times("At program start:");

    let num_calls = cli.num_calls.unwrap_or(10_000_000);

    for _ in 0..num_calls {
        getppid();
    }

    display_process_times("After getppid() loop:");

    exit_success();
}

fn display_process_times(msg: &str) {
    println!("{}", msg);

    let ts = clock_gettime(ClockId::CLOCK_PROCESS_CPUTIME_ID).unwrap_or_else(|e| {
        err_exit(e, "clock_gettime".into());
    });
    let cpu_time = ts.tv_sec() as f64 + ts.tv_nsec() as f64 * 1e-9;
    println!("    clock_gettime() returns: {:.2} secs", cpu_time);

    let ru = getrusage(UsageWho::RUSAGE_SELF).unwrap_or_else(|e| {
        err_exit(e, "getrusage".into());
    });
    let user = ru.user_time().tv_sec() as f64 + ru.user_time().tv_usec() as f64 * 1e-6;
    let system = ru.system_time().tv_sec() as f64 + ru.system_time().tv_usec() as f64 * 1e-6;
    println!(
        "    getrusage() yields: user CPU={:.2}; system CPU={:.2}",
        user, system
    );
}
