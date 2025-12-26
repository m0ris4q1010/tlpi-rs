use std::{os::fd::AsFd, process::Command};

use clap::Parser;
use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{Whence, lseek, read, write},
};

use lib::{err_exit, exit_success, fatal};

const MAX_LINE: usize = 100;

#[derive(Parser)]
struct Cli {
    new_val: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut line = [0; MAX_LINE];

    let open_flags = if cli.new_val.is_some() {
        OFlag::O_RDWR
    } else {
        OFlag::O_RDONLY
    };
    let file_perms = Mode::empty();
    let fd = open("/proc/sys/kernel/pid_max", open_flags, file_perms).unwrap_or_else(|e| {
        err_exit(e, "open");
    });

    let n = read(fd.as_fd(), &mut line).unwrap_or_else(|e| {
        err_exit(e, "read");
    });
    let old_val = str::from_utf8(&line[..n]).expect("valid utf-8");

    if cli.new_val.is_some() {
        print!("Old value: ");
    }
    println!("{}", old_val.trim());

    if let Some(val) = cli.new_val {
        if let Err(e) = lseek(fd.as_fd(), 0, Whence::SeekSet) {
            err_exit(e, "lseek");
        }
        let n = write(fd.as_fd(), val.as_bytes()).unwrap_or_else(|e| {
            err_exit(e, "write");
        });
        if n != val.len() {
            fatal("write() failed");
        }

        let output = Command::new("cat")
            .arg("/proc/sys/kernel/pid_max")
            .output()
            .expect("failed to read pid_max");
        let pid_max = String::from_utf8(output.stdout).expect("");
        println!("/proc/sys/kernel/pid_max now contains {}", pid_max.trim());
    }

    exit_success();
}
