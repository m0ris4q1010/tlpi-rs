use std::fs;

use clap::Parser;
use nix::{
    fcntl::readlink,
    sys::stat::{SFlag, lstat},
};

use lib::{err_exit, exit_failure, exit_success, fatal};

#[derive(Parser)]
struct Cli {
    pathname: String,
}

fn main() {
    let cli = Cli::parse();

    let statbuf = lstat(cli.pathname.as_str()).unwrap_or_else(|e| {
        err_exit(e, "lstat");
    });

    let kind = SFlag::from_bits_truncate(statbuf.st_mode);
    if !kind.contains(SFlag::S_IFLNK) {
        fatal(format!("{} is not a symbolic link", cli.pathname));
    }

    let buf = readlink(cli.pathname.as_str())
        .unwrap_or_else(|e| {
            err_exit(e, "readlink");
        })
        .into_string()
        .expect("invalid utf-8");

    println!("readlink: {} --> {}", cli.pathname, buf);

    let buf = fs::canonicalize(&buf).unwrap_or_else(|e| {
        eprintln!("failed to canonicalize path: {}", e);
        exit_failure();
    });

    println!("realpath: {} --> {}", cli.pathname, buf.display());

    exit_success();
}
