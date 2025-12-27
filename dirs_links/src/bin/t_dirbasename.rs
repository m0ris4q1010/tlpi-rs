use std::path::Path;

use clap::Parser;

use lib::exit_success;

#[derive(Parser)]
struct Cli {
    pathnames: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let pathnames = cli
        .pathnames
        .iter()
        .map(|p| Path::new(p))
        .collect::<Vec<_>>();

    for path in pathnames {
        println!(
            "{} ==> {:?} + {:?}",
            path.display(),
            path.parent().map(|p| p.display()),
            path.file_name().map(|p| p.to_str().expect("invalid utf-8"))
        );
    }

    exit_success();
}
