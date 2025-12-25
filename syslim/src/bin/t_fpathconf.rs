use std::os::fd::AsFd;

use nix::unistd::{PathconfVar, fpathconf};

use lib::{err_exit, exit_failure, exit_success};

fn main() {
    let stdin = std::io::stdin();

    fpathconf_print("_PC_NAME_MAX", &stdin, PathconfVar::NAME_MAX);
    fpathconf_print("_PC_PATH_MAX", &stdin, PathconfVar::PATH_MAX);
    fpathconf_print("_PC_PIPE_BUF", &stdin, PathconfVar::PIPE_BUF);

    exit_success();
}

fn fpathconf_print<Fd: AsFd>(msg: &str, fd: Fd, name: PathconfVar) {
    let value = fpathconf(fd, name)
        .unwrap_or_else(|e| {
            err_exit(e, format!("fpathconf({:?})", name));
        })
        .unwrap_or_else(|| {
            eprintln!("not defined: {}", msg);
            exit_failure();
        });

    println!("{}: {}", msg, value);
}
