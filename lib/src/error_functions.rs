use std::{
    env,
    io::{self, Write},
};

use libc::EXIT_FAILURE;
use nix::errno::Errno;

pub fn err_msg(errno: Errno, user_msg: String) {
    let saved_errno = unsafe { *libc::__errno_location() };
    let _ = io::stdout().flush();
    let _ = writeln!(
        io::stderr(),
        "ERROR{} {}",
        format!("[{:?} {}]", errno, errno.desc()),
        user_msg,
    );
    unsafe {
        *libc::__errno_location() = saved_errno;
    }
}

pub fn err_exit(errno: Errno, user_msg: String) -> ! {
    let _ = io::stdout().flush();
    let _ = writeln!(
        io::stderr(),
        "ERROR{} {}",
        format!("[{:?} {}]", errno, errno.desc()),
        user_msg,
    );
    terminate(true)
}

pub fn err_exit2(errno: Errno, user_msg: String) -> ! {
    let _ = writeln!(
        io::stderr(),
        "ERROR{} {}",
        format!("[{:?} {}]", errno, errno.desc()),
        user_msg,
    );
    terminate(false);
}

pub fn fatal(user_msg: String) -> ! {
    let _ = io::stdout().flush();
    let _ = writeln!(io::stderr(), "ERROR: {}", user_msg,);
    terminate(true)
}

fn terminate(use_exit3: bool) -> ! {
    if let Ok(x) = env::var("EF_DUMPCORE") {
        if !x.is_empty() {
            unsafe { libc::abort() }
        }
    }

    if use_exit3 {
        unsafe { libc::exit(EXIT_FAILURE) }
    } else {
        unsafe { libc::_exit(EXIT_FAILURE) }
    }
}
