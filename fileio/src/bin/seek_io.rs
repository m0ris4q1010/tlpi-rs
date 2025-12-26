use std::{
    fmt::{self, Display},
    os::fd::AsFd,
};

use clap::Parser;
use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{Whence, close, lseek, read, write},
};

use lib::{err_exit, exit_failure, exit_success};

#[derive(Parser)]
struct Cli {
    file: String,
    ops: Vec<String>,
}

enum Op {
    Seek { offset: i64 },
    Read { length: usize },
    ReadHex { length: usize },
    Write { str: String },
}

fn main() {
    let cli = Cli::parse();

    let ops: Vec<_> = cli.ops.iter().map(|s| Op::from(s.as_str())).collect();

    let open_flags = OFlag::O_RDWR | OFlag::O_CREAT;
    let file_perms = Mode::S_IRUSR
        | Mode::S_IWUSR
        | Mode::S_IRGRP
        | Mode::S_IWGRP
        | Mode::S_IROTH
        | Mode::S_IWOTH;
    let fd = open(cli.file.as_str(), open_flags, file_perms).unwrap_or_else(|e| {
        err_exit(e, "open");
    });

    for op in ops {
        match op {
            Op::Seek { offset } => {
                lseek(fd.as_fd(), offset, Whence::SeekSet).unwrap_or_else(|e| {
                    err_exit(e, "lseek");
                });
                println!("{}: seek succeeded", op);
            }
            Op::Read { length } => {
                let mut buf = vec![0; length];
                let num_read = read(fd.as_fd(), &mut buf).unwrap_or_else(|e| {
                    err_exit(e, "read");
                });
                if num_read == 0 {
                    println!("{}: end-of-file\n", op);
                } else {
                    print!("{}: ", op);
                    for j in 0..num_read {
                        print!(
                            "{}",
                            if buf[j].is_ascii_graphic() {
                                char::from(buf[j])
                            } else {
                                '?'
                            }
                        )
                    }
                    println!("");
                }
            }
            Op::ReadHex { length } => {
                let mut buf = vec![0; length];
                let num_read = read(fd.as_fd(), &mut buf).unwrap_or_else(|e| {
                    err_exit(e, "read");
                });
                if num_read == 0 {
                    println!("{}: end-of-file\n", op);
                } else {
                    print!("{}:", op);
                    for j in 0..num_read {
                        print!(" {:02x}", buf[j])
                    }
                    println!("");
                }
            }
            Op::Write { ref str } => {
                let num_written = write(fd.as_fd(), str.as_bytes()).unwrap_or_else(|e| {
                    err_exit(e, "write");
                });
                println!("{}: wrote {} bytes", op, num_written);
            }
        }
    }

    close(fd).unwrap_or_else(|e| err_exit(e, "close"));

    exit_success();
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Seek { offset } => write!(f, "s{}", offset),
            Op::Read { length } => write!(f, "r{}", length),
            Op::ReadHex { length } => write!(f, "R{}", length),
            Op::Write { str } => write!(f, "w{}", str),
        }
    }
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s.as_bytes().get(0) {
            Some(b's') => {
                let offset = s[1..].parse().unwrap_or_else(|e| {
                    eprintln!("Failed to parse offset: {}", e);
                    exit_failure();
                });
                Self::Seek { offset }
            }
            Some(b'r') => {
                let length = s[1..].parse().unwrap_or_else(|e| {
                    eprintln!("Failed to parse length: {}", e);
                    exit_failure();
                });
                Self::Read { length }
            }
            Some(b'R') => {
                let length = s[1..].parse().unwrap_or_else(|e| {
                    eprintln!("Failed to parse length: {}", e);
                    exit_failure();
                });
                Self::ReadHex { length }
            }
            Some(b'w') => {
                let str = s[1..].to_string();
                Self::Write { str }
            }
            Some(_) | None => {
                eprintln!("Argument must start with [rRws]: {}", s);
                exit_failure();
            }
        }
    }
}
