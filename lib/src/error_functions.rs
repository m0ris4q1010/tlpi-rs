use std::{
    env,
    fmt::Display,
    io::{self, Write},
};

use nix::{
    errno::Errno,
    libc::{__errno_location, _exit, EXIT_FAILURE, abort, exit},
};

pub fn err_msg(errno: Errno, user_msg: impl Display) {
    let saved_errno = unsafe { *__errno_location() };
    let _ = io::stdout().flush();
    let _ = writeln!(
        io::stderr(),
        "ERROR{} {}",
        format!("[{:?} {}]", errno, errno.desc()),
        user_msg,
    );
    unsafe {
        *__errno_location() = saved_errno;
    }
}

pub fn err_exit(errno: Errno, user_msg: impl Display) -> ! {
    let _ = io::stdout().flush();
    let _ = writeln!(
        io::stderr(),
        "ERROR{} {}",
        format!("[{:?} {}]", errno, errno.desc()),
        user_msg,
    );
    terminate(true)
}

pub fn err_exit2(errno: Errno, user_msg: impl Display) -> ! {
    let _ = writeln!(
        io::stderr(),
        "ERROR{} {}",
        format!("[{:?} {}]", errno, errno.desc()),
        user_msg,
    );
    terminate(false);
}

pub fn fatal(user_msg: impl Display) -> ! {
    let _ = io::stdout().flush();
    let _ = writeln!(io::stderr(), "ERROR: {}", user_msg,);
    terminate(true)
}

fn terminate(use_exit3: bool) -> ! {
    if let Ok(x) = env::var("EF_DUMPCORE") {
        if !x.is_empty() {
            unsafe { abort() }
        }
    }

    if use_exit3 {
        unsafe { exit(EXIT_FAILURE) }
    } else {
        unsafe { _exit(EXIT_FAILURE) }
    }
}
