use std::{env, io, path::Path};

use clap::Parser;
use nix::{
    libc::{c_int, sigpending, sigset_t},
    sys::signal::{SigHandler, SigSet, SigmaskHow, Signal, signal, sigprocmask},
    unistd::{getpid, sleep},
};

use lib::{err_exit, exit_failure, exit_success};

use signals::{NSIG, print_sigset};

#[derive(Parser)]
struct Cli {
    num_secs: Option<u32>,
}

static mut SIG_CNT: [c_int; NSIG] = [0; NSIG];
static mut GOT_SIG_INT: c_int = 0;

fn main() {
    let cli = Cli::parse();

    let arg_0 = env::args().next().unwrap();
    let arg_0 = Path::new(&arg_0)
        .file_name()
        .unwrap()
        .to_str()
        .expect("invalid utf-8");
    println!("{}: PID is {}", arg_0, getpid());

    for n in 1..NSIG {
        let sig = Signal::try_from(n as c_int).unwrap();
        let handler = SigHandler::Handler(handler);
        let _ = unsafe { signal(sig, handler) };
    }

    if let Some(num_secs) = cli.num_secs {
        let blocking_mask = SigSet::all();
        if let Err(e) = sigprocmask(SigmaskHow::SIG_SETMASK, Some(&blocking_mask), None) {
            err_exit(e, "sigprocmask");
        }
        println!("{}: sleeping for {} seconds", arg_0, num_secs);
        sleep(num_secs);

        let mut pending_mask = SigSet::empty();
        let pending_mask = &mut pending_mask as *mut SigSet as *mut sigset_t;
        if unsafe { sigpending(pending_mask) } == -1 {
            eprintln!("sigpending");
            exit_failure();
        }
        let pending_mask = unsafe { SigSet::from_sigset_t_unchecked(*pending_mask) };

        println!("{}: pending signals are: ", arg_0);
        print_sigset(&mut io::stdout(), "\t\t", pending_mask);

        let empty_mask = SigSet::empty();
        if let Err(e) = sigprocmask(SigmaskHow::SIG_SETMASK, Some(&empty_mask), None) {
            err_exit(e, "sigprocmask");
        }
    }

    loop {
        if unsafe { GOT_SIG_INT } > 0 {
            break;
        }
    }

    for n in 1..NSIG {
        let count = unsafe { SIG_CNT[n] };
        if count != 0 {
            println!(
                "{}: signal {} caught {} time{}",
                arg_0,
                n,
                count,
                if count == 1 { "" } else { "s" }
            );
        }
    }

    exit_success();
}

extern "C" fn handler(sig: c_int) {
    if sig == Signal::SIGINT as c_int {
        unsafe {
            GOT_SIG_INT = 1;
        }
    } else {
        unsafe {
            SIG_CNT[sig as usize] += 1;
        }
    }
}
