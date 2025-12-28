use std::{env, path::Path};

use clap::Parser;
use nix::{
    sys::signal::{Signal, kill},
    unistd::Pid,
};

use lib::{err_exit, exit_success};

#[derive(Parser)]
struct Cli {
    pid: i32,
    num_sigs: usize,
    sig_num: i32,
    sig_num_2: Option<i32>,
}

fn main() {
    let cli = Cli::parse();
    let arg_0 = env::args().next().unwrap();
    let arg_0 = Path::new(&arg_0)
        .file_name()
        .unwrap()
        .to_str()
        .expect("invalid utf-8");

    let pid = Pid::from_raw(cli.pid);
    assert!(cli.num_sigs > 0);
    let sig = Signal::try_from(cli.sig_num).expect("sig_num: out of range");

    println!(
        "{}: sending signal {} to process {} {} times",
        arg_0, cli.sig_num, cli.pid, cli.num_sigs,
    );

    for _ in 0..cli.num_sigs {
        if let Err(e) = kill(pid, sig) {
            err_exit(e, "kill");
        }
    }

    if let Some(sig_num) = cli.sig_num_2 {
        let sig = Signal::try_from(sig_num).expect("sig_num_2: out of range");
        if let Err(e) = kill(pid, sig) {
            err_exit(e, "kill");
        }
    }

    println!("{}: exiting", arg_0);
    exit_success();
}
