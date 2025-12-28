use std::{ffi::CStr, io::Write};

use nix::{
    libc::{c_int, sigpending, sigset_t, strsignal},
    sys::signal::{SigSet, SigmaskHow, Signal, sigprocmask},
};

use lib::err_msg;

pub const NSIG: usize = 31 + 1;

pub fn print_sigset<F: Write>(of: &mut F, prefix: &str, sigset: SigSet) {
    let mut cnt = 0;
    for sig in 1..(NSIG as c_int) {
        if sigset.contains(Signal::try_from(sig).unwrap()) {
            cnt += 1;
            let p = unsafe { strsignal(sig) };
            let s = if p.is_null() {
                "unknown signal"
            } else {
                unsafe { CStr::from_ptr(p).to_str().expect("invalid utf-8") }
            };
            of.write_fmt(format_args!("{}{} ({})\n", prefix, sig, s))
                .expect("write_fmt");
        }
    }
    if cnt == 0 {
        of.write_fmt(format_args!("{}<empty signal set>\n", prefix))
            .expect("write_fmt");
    }
}

pub fn print_sig_mask<F: Write>(of: &mut F, msg: Option<&str>) -> i32 {
    if let Some(msg) = msg {
        of.write_fmt(format_args!("{}", msg)).expect("write_fmt");
    }
    let mut curr_mask = SigSet::empty();
    if let Err(e) = sigprocmask(SigmaskHow::SIG_BLOCK, None, Some(&mut curr_mask)) {
        err_msg(e, "sigprocmask");
        return -1;
    }
    print_sigset(of, "\t\t", curr_mask);
    return 0;
}

pub fn print_pending_sigs<F: Write>(of: &mut F, msg: Option<&str>) -> i32 {
    if let Some(msg) = msg {
        of.write_fmt(format_args!("{}", msg)).expect("write_fmt");
    }
    let mut pending_sigs = SigSet::empty();
    let pending_sigs = &mut pending_sigs as *mut SigSet as *mut sigset_t;
    if unsafe { sigpending(pending_sigs) } == -1 {
        return -1;
    }
    let pending_sigs = unsafe { SigSet::from_sigset_t_unchecked(*pending_sigs) };
    print_sigset(of, "\t\t", pending_sigs);
    return 0;
}
