use clap::Parser;
use nix::{
    errno::Errno,
    sys::signal::{Signal, kill},
    unistd::Pid,
};

use lib::{err_exit, exit_success};

#[derive(Parser)]
struct Cli {
    pid: i32,
    sig_num: i32,
}

fn main() {
    let cli = Cli::parse();

    let pid = Pid::from_raw(cli.pid);

    let signal = if cli.sig_num == 0 {
        None
    } else {
        let signal = Signal::try_from(cli.sig_num).expect("out of signal range");
        Some(signal)
    };

    if let Err(e) = kill(pid, signal) {
        match e {
            Errno::EPERM => {
                println!("Process exists, but we don't have permission to send it a signal")
            }
            Errno::ESRCH => println!("Process does not exist"),
            _ => err_exit(e, "kill"),
        }
    } else {
        if signal.is_none() {
            println!("Process exists and we can send it a signal");
        }
    }

    exit_success();
}
