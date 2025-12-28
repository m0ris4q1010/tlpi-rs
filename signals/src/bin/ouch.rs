use nix::{
    libc::c_int,
    sys::signal::{SigHandler, Signal, signal},
    unistd::sleep,
};

use lib::err_exit;

fn main() {
    let result = unsafe { signal(Signal::SIGINT, SigHandler::Handler(sig_handler)) };
    if let Err(e) = result {
        err_exit(e, "signal");
    }

    for j in 0.. {
        println!("{}", j);
        sleep(3);
    }
}

extern "C" fn sig_handler(_signal: c_int) {
    println!("Ouch!");
}
