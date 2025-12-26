use chrono::{DateTime, Local};
use clap::Parser;
use nix::sys::stat::{FileStat, Mode, SFlag, lstat, major, minor, stat};

use lib::{err_exit, exit_success};

use files::file_perm_str;

#[derive(Parser)]
struct Cli {
    file: String,
    #[arg(short, long)]
    lstat: bool,
}

fn main() {
    let cli = Cli::parse();

    let st = if cli.lstat {
        lstat(cli.file.as_str()).unwrap_or_else(|e| {
            err_exit(e, "lstat");
        })
    } else {
        stat(cli.file.as_str()).unwrap_or_else(|e| {
            err_exit(e, "stat");
        })
    };

    display_stat_info(st);

    exit_success();
}

fn display_stat_info(stat: FileStat) {
    let kind = SFlag::from_bits_truncate(stat.st_mode);
    let perm = Mode::from_bits_truncate(stat.st_mode);

    #[rustfmt::skip]
    println!(
        "File type:                {}",
        if kind.contains(SFlag::S_IFREG) { "regular file" }
        else if kind.contains(SFlag::S_IFDIR) { "directory" }
        else if kind.contains(SFlag::S_IFCHR) { "character device" }
        else if kind.contains(SFlag::S_IFBLK) { "block device" }
        else if kind.contains(SFlag::S_IFLNK) { "symbolic (soft) link" }
        else if kind.contains(SFlag::S_IFIFO) { "FIFO or pipe" }
        else if kind.contains(SFlag::S_IFSOCK) { "socket" }
        else { "unknown file type?" }
    );

    println!(
        "Device containing i-node: major={}, minor={}",
        major(stat.st_dev),
        minor(stat.st_dev)
    );
    println!("I-node number:            {}", stat.st_ino);

    println!(
        "Mode:                     {:o} ({})",
        stat.st_mode,
        file_perm_str(perm, 0)
    );

    if perm.contains(Mode::S_ISUID) || perm.contains(Mode::S_ISGID) || perm.contains(Mode::S_ISVTX)
    {
        #[rustfmt::skip]
        println!(
            "    special bits:        {}{}{}",
            if perm.contains(Mode::S_ISUID) { " set-UID" } else { "" },
            if perm.contains(Mode::S_ISGID) { " set-GID" } else { "" },
            if perm.contains(Mode::S_ISVTX) { " sticky" } else { "" },
        );
    }

    println!("Number of (hard) links:   {}", stat.st_nlink);

    println!(
        "Ownership:                UID={} GID={}",
        stat.st_uid, stat.st_gid
    );

    if kind.contains(SFlag::S_IFCHR) || kind.contains(SFlag::S_IFBLK) {
        println!(
            "Device number (st_rdev):   major={}; minor={}",
            major(stat.st_rdev),
            minor(stat.st_rdev),
        );
    }

    println!("File size:                {} bytes", stat.st_size);
    println!("Optimal I/O block size:   {} bytes", stat.st_blksize);
    println!("512B blocks allocated:    {}", stat.st_blocks);

    println!("Last file access:         {}", ctime(stat.st_atime));
    println!("Last file modification:   {}", ctime(stat.st_mtime));
    println!("Last status change:       {}", ctime(stat.st_ctime));
}

fn ctime(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap()
        .with_timezone(&Local)
        .format("%c")
        .to_string()
}
