use clap::{CommandFactory, Parser};
use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify, InotifyEvent};

use lib::{err_exit, exit_failure};

#[derive(Parser)]
struct Cli {
    pathnames: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    if cli.pathnames.is_empty() {
        Cli::command().print_help().unwrap();
        exit_failure();
    }

    let instance = Inotify::init(InitFlags::empty()).unwrap_or_else(|e| {
        err_exit(e, "inotify_init");
    });

    for pathname in &cli.pathnames {
        let wd = instance
            .add_watch(pathname.as_str(), AddWatchFlags::IN_ALL_EVENTS)
            .unwrap_or_else(|e| {
                err_exit(e, "inotify_add_watch");
            });
        println!("Watching {} using wd {:?}", pathname, wd);
    }

    loop {
        let events = instance
            .read_events()
            .unwrap_or_else(|e| err_exit(e, "inotify_read_events"));
        println!("Read {} events from inotify instance", events.len());
        for event in events {
            display_inotify_event(event);
        }
    }
}

fn display_inotify_event(event: InotifyEvent) {
    println!("    wd = {:?}", event.wd);
    if event.cookie > 0 {
        println!("    cookie = {}", event.cookie);
    }

    print!("    mask = ");
    for flag in ADD_WATCH_FLAGS {
        if event.mask.contains(flag) {
            print!("{:?} ", flag);
        }
    }
    println!("");

    if let Some(name) = event.name {
        let name = name.into_string().expect("invalid utf-8");
        println!("    name = {}", name);
    }
}

const ADD_WATCH_FLAGS: [AddWatchFlags; 16] = [
    AddWatchFlags::IN_ACCESS,
    AddWatchFlags::IN_ATTRIB,
    AddWatchFlags::IN_CLOSE_NOWRITE,
    AddWatchFlags::IN_CLOSE_WRITE,
    AddWatchFlags::IN_CREATE,
    AddWatchFlags::IN_DELETE,
    AddWatchFlags::IN_DELETE_SELF,
    AddWatchFlags::IN_IGNORED,
    AddWatchFlags::IN_ISDIR,
    AddWatchFlags::IN_MODIFY,
    AddWatchFlags::IN_MOVE_SELF,
    AddWatchFlags::IN_MOVED_FROM,
    AddWatchFlags::IN_MOVED_TO,
    AddWatchFlags::IN_OPEN,
    AddWatchFlags::IN_Q_OVERFLOW,
    AddWatchFlags::IN_UNMOUNT,
];
