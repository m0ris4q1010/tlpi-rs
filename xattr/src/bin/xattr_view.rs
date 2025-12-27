use clap::Parser;
use xattr::{get, list};

use lib::{exit_failure, exit_success};

#[derive(Parser)]
struct Cli {
    files: Vec<String>,
    #[arg(short('x'), long)]
    hex: bool,
}

fn main() {
    let cli = Cli::parse();

    for file in &cli.files {
        let xattrs = list(file).unwrap_or_else(|e| {
            eprintln!("listxattr: {}", e);
            exit_failure();
        });
        println!("{}:", file);
        for name in xattrs {
            let value = get(file, &name).unwrap_or_else(|e| {
                eprintln!("getxattr: {}", e);
                exit_failure();
            });
            let name = name.to_str().expect("invalid utf-8");
            if let Some(value) = value {
                let value = if cli.hex {
                    value.iter().map(|b| format!("{:02x}", b)).collect()
                } else {
                    String::from_utf8(value).expect("invalid utf-8")
                };
                println!("    name={}; value={}", name, value);
            } else {
                println!("    name={}; value=", name);
            }
        }
    }

    exit_success();
}
