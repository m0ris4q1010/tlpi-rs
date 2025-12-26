use std::ffi::CString;

use nix::libc::{STDOUT_FILENO, c_char, c_int, write};

use lib::exit_success;

unsafe extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
}

fn main() {
    unsafe {
        let fmt = CString::new("To man the world is twofold, ").unwrap();
        printf(fmt.as_ptr());
        if std::env::args().count() > 1 {
            let fmt = CString::new("\n").unwrap();
            printf(fmt.as_ptr());
        }
        let msg = "in accordance with his twofold attitude.\n";
        write(STDOUT_FILENO, msg.as_ptr() as *const _, msg.len());
    }

    exit_success();
}
