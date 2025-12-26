use std::os::fd::AsFd;

use clap::Parser;
use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{Whence, lseek64, write},
};

use lib::{err_exit, exit_success};

#[derive(Parser)]
struct Cli {
    pathname: String,
    offset: i64,
}

fn main() {
    let cli = Cli::parse();

    let open_flags = OFlag::O_RDWR | OFlag::O_CREAT;
    let file_perms = Mode::S_IRUSR | Mode::S_IWUSR;
    let fd = open(cli.pathname.as_str(), open_flags, file_perms).unwrap_or_else(|e| {
        err_exit(e, "open");
    });

    lseek64(fd.as_fd(), cli.offset, Whence::SeekSet).unwrap_or_else(|e| {
        err_exit(e, "lseek64");
    });

    write(fd.as_fd(), "test".as_bytes()).unwrap_or_else(|e| {
        err_exit(e, "write");
    });

    exit_success();
}
