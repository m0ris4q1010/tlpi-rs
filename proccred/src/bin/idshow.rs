use nix::{
    libc::{setfsgid, setfsuid},
    unistd::{Gid, Uid, getgroups, getresgid, getresuid},
};

use lib::{err_exit, exit_success};
use users_groups::{group_name_from_id, user_name_from_id};

fn main() {
    let resuid = getresuid().unwrap_or_else(|e| {
        err_exit(e, "getresuid");
    });
    let resgid = getresgid().unwrap_or_else(|e| {
        err_exit(e, "getresgid");
    });
    let fsuid = unsafe { setfsuid(0) };
    let fsgid = unsafe { setfsgid(0) };

    println!(
        "UID: real={} ({}); eff={} ({}); saved={} ({}); fs={} ({})",
        user_name_from_id(resuid.real).unwrap_or("???".into()),
        resuid.real,
        user_name_from_id(resuid.effective).unwrap_or("???".into()),
        resuid.effective,
        user_name_from_id(resuid.saved).unwrap_or("???".into()),
        resuid.saved,
        user_name_from_id(Uid::from_raw(fsuid as u32)).unwrap_or("???".into()),
        fsuid,
    );

    println!(
        "GID: real={} ({}); eff={} ({}); saved={} ({}); fs={} ({})",
        group_name_from_id(resgid.real).unwrap_or("???".into()),
        resgid.real,
        group_name_from_id(resgid.effective).unwrap_or("???".into()),
        resgid.effective,
        group_name_from_id(resgid.saved).unwrap_or("???".into()),
        resgid.saved,
        group_name_from_id(Gid::from_raw(fsgid as u32)).unwrap_or("???".into()),
        fsgid,
    );

    let num_groups = getgroups().unwrap_or_else(|e| {
        err_exit(e, "getgroups");
    });
    print!("Supplementary groups {}:", num_groups.len());
    for gid in num_groups {
        print!(
            " {} ({})",
            group_name_from_id(gid).unwrap_or("???".into()),
            gid
        );
    }
    println!("");

    exit_success();
}
