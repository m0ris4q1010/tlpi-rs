pub mod error_functions;

pub use error_functions::{err_exit, err_exit2, err_msg, fatal};

pub fn exit_success() -> ! {
    std::process::exit(nix::libc::EXIT_SUCCESS);
}

pub fn exit_failure() -> ! {
    std::process::exit(nix::libc::EXIT_FAILURE);
}
