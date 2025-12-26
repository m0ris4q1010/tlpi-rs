use clap::Parser;
use nix::unistd::chown;

use lib::{err_msg, exit_failure, exit_success, fatal};
use users_groups::{group_id_from_name, user_id_from_name};

#[derive(Parser)]
struct Cli {
    owner: String,
    group: String,
    files: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let uid = if &cli.owner == "-" {
        None
    } else {
        let uid = user_id_from_name(&cli.owner).unwrap_or_else(|| {
            fatal(format!("No such user {}", &cli.owner));
        });
        Some(uid)
    };

    let gid = if &cli.group == "-" {
        None
    } else {
        let gid = group_id_from_name(&cli.group).unwrap_or_else(|| {
            fatal(format!("No group user {}", &cli.group));
        });
        Some(gid)
    };

    let mut err_found = false;
    for file in &cli.files {
        if let Err(e) = chown(file.as_str(), uid, gid) {
            err_msg(e, format!("chown: {}", &file));
            err_found = true;
        }
    }

    if err_found {
        exit_failure();
    } else {
        exit_success();
    }
}
