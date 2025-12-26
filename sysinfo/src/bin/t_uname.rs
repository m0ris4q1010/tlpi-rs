use nix::sys::utsname::uname;

use lib::{err_exit, exit_success};

fn main() {
    let uts_name = uname().unwrap_or_else(|e| {
        err_exit(e, "uname");
    });

    let nodename = uts_name.nodename().to_str().expect("nodename");
    let sysname = uts_name.sysname().to_str().expect("sysname");
    let release = uts_name.release().to_str().expect("release");
    let version = uts_name.version().to_str().expect("version");
    let machine = uts_name.machine().to_str().expect("machine");

    println!("Node name:   {}", nodename);
    println!("System name: {}", sysname);
    println!("Release:     {}", release);
    println!("Version:     {}", version);
    println!("Machine:     {}", machine);

    exit_success();
}
