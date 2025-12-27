use clap::Parser;
use nix::{dir::Dir, fcntl::OFlag, sys::stat::Mode};

use lib::{err_exit, err_msg, exit_success};

#[derive(Parser)]
struct Cli {
    directories: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.directories.is_empty() {
        list_files(".");
    } else {
        for dir in cli.directories {
            list_files(&dir);
        }
    }

    exit_success();
}

fn list_files(dirname: &str) {
    let open_flags = OFlag::O_RDONLY | OFlag::O_CLOEXEC;
    let file_perms = Mode::empty();
    let dir = match Dir::open(dirname, open_flags, file_perms) {
        Ok(dir) => dir,
        Err(e) => {
            err_msg(e, format!("opendir: {}", dirname));
            return;
        }
    };

    for entry in dir {
        match entry {
            Ok(entry) => {
                let filename = entry.file_name().to_str().expect("invalid utf-8");
                if filename == "." || filename == ".." {
                    continue;
                }
                println!("{}/{}", dirname, filename);
            }
            Err(e) => {
                err_exit(e, "readdir");
            }
        }
    }
}
