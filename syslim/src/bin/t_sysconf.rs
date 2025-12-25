use nix::unistd::{SysconfVar, sysconf};

use lib::{err_exit, exit_failure, exit_success};

fn main() {
    sysconf_print("_SC_ARG_MAX       ", SysconfVar::ARG_MAX);
    sysconf_print("_SC_LOGIN_NAME_MAX", SysconfVar::LOGIN_NAME_MAX);
    sysconf_print("_SC_OPEN_MAX      ", SysconfVar::OPEN_MAX);
    sysconf_print("_SC_NGROUPS_MAX   ", SysconfVar::NGROUPS_MAX);
    sysconf_print("_SC_PAGESIZE      ", SysconfVar::PAGE_SIZE);
    sysconf_print("_SC_RTSIG_MAX     ", SysconfVar::RTSIG_MAX);

    exit_success();
}

fn sysconf_print(msg: &str, name: SysconfVar) {
    let value = sysconf(name)
        .unwrap_or_else(|e| {
            err_exit(e, format!("sysconf({:?})", name));
        })
        .unwrap_or_else(|| {
            eprintln!("not defined: {}", msg);
            exit_failure();
        });

    println!("{}: {}", msg, value);
}
