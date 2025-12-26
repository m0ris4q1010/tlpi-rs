use nix::{
    libc::{gid_t, uid_t},
    unistd::{Gid, Group, Uid, User},
};

use lib::err_exit;

pub fn user_name_from_id(uid: Uid) -> Option<String> {
    User::from_uid(uid)
        .unwrap_or_else(|e| {
            err_exit(e, "getpwuid_r");
        })
        .map(|user| user.name)
}

pub fn user_id_from_name(name: &str) -> Option<Uid> {
    if let Ok(uid) = uid_t::from_str_radix(name, 10) {
        return Some(Uid::from_raw(uid));
    }
    User::from_name(name)
        .unwrap_or_else(|e| {
            err_exit(e, "getpwnam_r");
        })
        .map(|user| user.uid)
}

pub fn group_name_from_id(gid: Gid) -> Option<String> {
    Group::from_gid(gid)
        .unwrap_or_else(|e| {
            err_exit(e, "getgrgid_r");
        })
        .map(|group| group.name)
}

pub fn group_id_from_name(name: &str) -> Option<Gid> {
    if let Ok(gid) = gid_t::from_str_radix(name, 10) {
        return Some(Gid::from_raw(gid));
    }
    Group::from_name(name)
        .unwrap_or_else(|e| {
            err_exit(e, "getgrnam_r");
        })
        .map(|group| group.gid)
}
