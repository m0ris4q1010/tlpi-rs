use nix::{
    fcntl::{AT_FDCWD, OFlag, open},
    sys::stat::{Mode, stat, umask},
    unistd::{UnlinkatFlags, mkdir, unlink, unlinkat},
};

use lib::{err_exit, err_msg, exit_success};

use files::file_perm_str;

const MYFILE: &str = "myfile";
const MYDIR: &str = "mydir";

fn main() {
    let file_perms = Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IWGRP;
    let dir_perms = Mode::S_IRWXU | Mode::S_IRWXG | Mode::S_IRWXO;
    let umask_setting = Mode::S_IWGRP | Mode::S_IXGRP | Mode::S_IWOTH | Mode::S_IXOTH;

    umask(umask_setting);

    let open_flags = OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_EXCL;
    let _fd = open(MYFILE, open_flags, file_perms).unwrap_or_else(|e| {
        err_exit(e, format!("open-{}", MYFILE));
    });
    if let Err(e) = mkdir(MYDIR, dir_perms) {
        err_exit(e, format!("mkdir-{}", MYDIR));
    }

    let u = umask(Mode::empty());

    let sb = stat(MYFILE).unwrap_or_else(|e| {
        err_exit(e, format!("stat-{}", MYFILE));
    });
    println!("Requested file perms: {}", file_perm_str(file_perms, 0));
    println!("Process umask:        {}", file_perm_str(u, 0));
    println!(
        "Actual file perms:    {}\n",
        file_perm_str(Mode::from_bits_truncate(sb.st_mode), 0),
    );

    let sb = stat(MYDIR).unwrap_or_else(|e| {
        err_exit(e, format!("stat-{}", MYDIR));
    });
    println!("Requested dir. perms: {}", file_perm_str(dir_perms, 0));
    println!("Process umask:        {}", file_perm_str(u, 0));
    println!(
        "Actual dir. perms:    {}",
        file_perm_str(Mode::from_bits_truncate(sb.st_mode), 0)
    );

    if let Err(e) = unlink(MYFILE) {
        err_msg(e, format!("unlink-{}", MYFILE));
    }
    if let Err(e) = unlinkat(AT_FDCWD, MYDIR, UnlinkatFlags::RemoveDir) {
        err_msg(e, format!("unlink-{}", MYDIR));
    }

    exit_success();
}
