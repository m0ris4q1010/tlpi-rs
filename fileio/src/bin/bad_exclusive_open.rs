use clap::Parser;
use nix::{
    errno::Errno,
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{close, getpid, sleep},
};

use lib::{err_exit, exit_success};

#[derive(Parser)]
struct Cli {
    file: String,
    #[arg(short, long)]
    sleep: bool,
}

fn main() {
    let cli = Cli::parse();

    let open_flags = OFlag::O_WRONLY;
    let file_perms = Mode::empty();
    match open(cli.file.as_str(), open_flags, file_perms) {
        Ok(fd) => {
            println!("[PID {}] File \"{}\" already exists", getpid(), cli.file);
            close(fd).unwrap_or_else(|e| err_exit(e, "close"));
        }
        Err(e) => {
            if e != Errno::ENOENT {
                err_exit(e, "open");
            } else {
                println!("[PID {}] File \"{}\" doesn't exist yet", getpid(), cli.file);
                if cli.sleep {
                    sleep(5);
                    println!("[PID {}], Done sleeping", getpid());
                }
            }
            let open_flags = OFlag::O_WRONLY | OFlag::O_CREAT;
            let file_perms = Mode::S_IRUSR | Mode::S_IWUSR;
            let fd = open(cli.file.as_str(), open_flags, file_perms).unwrap_or_else(|e| {
                err_exit(e, "open");
            });
            println!(
                "[PID {}] Creted file \"{}\" exclusively",
                getpid(),
                cli.file
            );
            close(fd).unwrap_or_else(|e| err_exit(e, "close"));
        }
    }

    exit_success();
}
