use std::{io::IoSliceMut, mem, os::fd::AsFd, slice};

use clap::Parser;
use nix::{
    fcntl::{OFlag, open},
    libc::stat,
    sys::{stat::Mode, uio::readv},
};

use lib::{err_exit, exit_success};

#[derive(Parser)]
struct Cli {
    file: String,
}

fn main() {
    let cli = Cli::parse();

    let mut my_struct: stat = unsafe { mem::zeroed() };
    let mut x: i32 = 0;
    let mut buf = [0u8; 100];

    let mut tot_required = 0;
    tot_required += mem::size_of::<stat>();
    tot_required += mem::size_of::<i32>();
    tot_required += buf.len();

    let mut iov = [
        IoSliceMut::new(unsafe {
            slice::from_raw_parts_mut(
                &mut my_struct as *mut stat as *mut u8,
                mem::size_of::<stat>(),
            )
        }),
        IoSliceMut::new(unsafe {
            slice::from_raw_parts_mut(&mut x as *mut i32 as *mut u8, mem::size_of::<i32>())
        }),
        IoSliceMut::new(&mut buf),
    ];

    let open_flags = OFlag::O_RDONLY;
    let file_perms = Mode::empty();
    let fd = open(cli.file.as_str(), open_flags, file_perms).unwrap_or_else(|e| {
        err_exit(e, "open".into());
    });

    let num_read = readv(fd.as_fd(), &mut iov).unwrap_or_else(|e| {
        err_exit(e, "readv".into());
    });

    if num_read < tot_required {
        println!("Read fewer bytes than requested");
    }
    println!(
        "total bytes requested: {}; bytes read: {}",
        tot_required, num_read
    );

    exit_success();
}
