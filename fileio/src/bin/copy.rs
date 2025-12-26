use std::os::fd::AsFd;

use clap::Parser;
use nix::{
    errno::Errno,
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{close, read, write},
};

use lib::{err_exit, exit_success};

const BUF_SIZE: usize = 1024;

#[derive(Parser)]
struct Cli {
    old_file: String,
    new_file: String,
}

fn main() {
    let cli = Cli::parse();

    let open_flags = OFlag::O_RDONLY;
    let file_perms = Mode::empty();
    let input_fd = open(cli.old_file.as_str(), open_flags, file_perms).unwrap_or_else(|e| {
        err_exit(e, format!("opening file {}", cli.old_file));
    });

    let open_flags = OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_TRUNC;
    let file_perms = Mode::S_IRUSR
        | Mode::S_IWUSR
        | Mode::S_IRGRP
        | Mode::S_IWGRP
        | Mode::S_IROTH
        | Mode::S_IWOTH;
    let output_fd = open(cli.new_file.as_str(), open_flags, file_perms).unwrap_or_else(|e| {
        err_exit(e, format!("opening file {}", cli.new_file));
    });

    let mut buf = [0; BUF_SIZE];
    loop {
        let num_read = read(input_fd.as_fd(), &mut buf).unwrap_or_else(|e| {
            err_exit(e, "read");
        });
        if num_read == 0 {
            break;
        }
        let num_written = write(output_fd.as_fd(), &buf[..num_read]).unwrap_or_else(|e| {
            err_exit(e, "write() returned error");
        });
        if num_written < num_read {
            err_exit(Errno::UnknownErrno, "partial write occurred");
        }
    }

    close(input_fd).unwrap_or_else(|e| err_exit(e, "close input"));
    close(output_fd).unwrap_or_else(|e| err_exit(e, "close output"));

    exit_success();
}
