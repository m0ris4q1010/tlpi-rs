use std::{env, ffi::CString};

use nix::{env::clearenv, errno::Errno};

use lib::{err_exit, exit_failure, exit_success};

fn main() {
    unsafe {
        if let Err(e) = clearenv() {
            eprintln!("failed to clearenv(): {}", e);
            exit_failure();
        }
    }

    for arg in env::args() {
        let p = CString::new(arg)
            .expect("Failed to convert into CString")
            .into_raw();
        unsafe {
            if nix::libc::putenv(p) != 0 {
                let errno = Errno::from_raw(*nix::libc::__errno_location());
                err_exit(errno, "putenv".into());
            }
        }
    }

    unsafe {
        env::set_var("GREET", "Hello world");
        env::remove_var("BYE");
    }

    for (key, value) in env::vars() {
        println!("{}={}", key, value);
    }

    exit_success();
}
