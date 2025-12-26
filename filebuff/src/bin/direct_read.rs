use std::{
    alloc::{self, Layout},
    os::fd::AsFd,
    slice,
};

use clap::Parser;
use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{Whence, lseek, read},
};

use lib::{err_exit, exit_success};

#[derive(Parser)]
struct Cli {
    file: String,
    length: usize,
    #[arg(default_value_t = 0)]
    offset: i64,
    #[arg(default_value_t = 4096)]
    alignment: usize,
}

fn main() {
    let Cli {
        file,
        length,
        offset,
        alignment,
    } = Cli::parse();

    let open_flags = OFlag::O_RDONLY | OFlag::O_DIRECT;
    let fd = open(file.as_str(), open_flags, Mode::empty()).unwrap_or_else(|e| {
        err_exit(e, "open");
    });

    let layout =
        Layout::from_size_align(length + alignment, alignment * 2).expect("invalid layout");
    let p = unsafe { alloc::alloc(layout) };

    let buf = unsafe { slice::from_raw_parts_mut(p.add(alignment), length) };

    if let Err(e) = lseek(fd.as_fd(), offset, Whence::SeekSet) {
        err_exit(e, "lseek");
    }

    let num_read = read(fd.as_fd(), buf).unwrap_or_else(|e| {
        err_exit(e, "read");
    });
    println!("Read {} bytes", num_read);

    unsafe { alloc::dealloc(p, layout) }

    exit_success();
}
