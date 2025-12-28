use nix::{
    libc::c_int,
    sys::signal::{SigHandler, Signal, signal},
    unistd::pause,
};

use lib::{err_exit, exit_success};

fn main() {
    if let Err(e) = unsafe { signal(Signal::SIGINT, SigHandler::Handler(sig_handler)) } {
        err_exit(e, "signal");
    }
    if let Err(e) = unsafe { signal(Signal::SIGQUIT, SigHandler::Handler(sig_handler)) } {
        err_exit(e, "signal");
    }

    loop {
        pause();
    }
}

extern "C" fn sig_handler(signal: c_int) {
    // UNSAFE: This handler use non-async-signal-safe functions
    // (printf(), exit(); see Section 21.1.2)

    static mut COUNT: i32 = 0;

    if signal == Signal::SIGINT as c_int {
        unsafe {
            COUNT += 1;
        }
        println!("Caught SIGINT {}", unsafe { COUNT });
        return;
    }

    println!("Caught SIGQUIT - that's all folks!");

    exit_success();
}
