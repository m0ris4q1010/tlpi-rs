use std::{
    io::{self, Write},
    os::fd::AsFd,
    path::Path,
    process::Command,
};

use clap::Parser;
use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{close, unlink, write},
};

use lib::{err_exit, exit_success, fatal};

const BUF_SIZE: usize = 1024;

#[derive(Parser)]
struct Cli {
    temp_file: String,
    #[arg(default_value_t = 100_000)]
    num_1kb_blocks: usize,
}

fn main() {
    let cli = Cli::parse();

    let open_flags = OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_EXCL;
    let file_perms = Mode::S_IRUSR | Mode::S_IWUSR;
    let fd = open(cli.temp_file.as_str(), open_flags, file_perms).unwrap_or_else(|e| {
        err_exit(e, "open");
    });
    if let Err(e) = unlink(cli.temp_file.as_str()) {
        err_exit(e, "unlink");
    }

    let buf = [0; BUF_SIZE];

    for _ in 0..cli.num_1kb_blocks {
        let n = write(fd.as_fd(), &buf).unwrap_or_else(|e| {
            err_exit(e, "write");
        });
        if n < BUF_SIZE {
            fatal("partial write");
        }
    }

    system(cli.temp_file.as_str());

    if let Err(e) = close(fd) {
        err_exit(e, "close");
    }
    println!("********** Closed file descriptor");

    system(cli.temp_file.as_str());

    exit_success();
}

fn system(file: &str) {
    let dir = Path::new(file).parent().expect("no parent directory");

    let output = Command::new("df").arg("-k").arg(dir).output().unwrap();

    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
