use clap::Parser;
use nix::libc::{c_char, c_void, free, malloc, sbrk};

use lib::{exit_failure, exit_success};

const MAX_ALLOCS: usize = 1_000_000;

#[derive(Parser)]
struct Cli {
    num_allocs: usize,
    block_size: usize,
    free_step: Option<usize>,
    free_min: Option<usize>,
    free_max: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let Cli {
        num_allocs,
        block_size,
        ..
    } = cli;
    let free_step = cli.free_step.unwrap_or(1);
    let free_min = cli.free_min.unwrap_or(1);
    let free_max = cli.free_max.unwrap_or(num_allocs);

    assert!(num_allocs <= MAX_ALLOCS);
    assert!(free_max <= num_allocs);

    let mut ptr = [std::ptr::null_mut(); MAX_ALLOCS];

    println!("Initial program break:         {:p}", unsafe { sbrk(0) });

    println!("Allocating {} * {} bytes", num_allocs, block_size);

    for j in 0..num_allocs {
        unsafe {
            ptr[j] = malloc(block_size) as *mut c_char;
            if ptr[j].is_null() {
                eprintln!("failed to malloc()");
                exit_failure();
            }
        }
    }

    println!("Prgram break is now:           {:p}", unsafe { sbrk(0) });

    println!(
        "Freeing blocks from {} to {} in steps of {}",
        free_min, free_max, free_step
    );

    for j in ((free_min - 1)..free_max).step_by(free_step) {
        unsafe {
            free(ptr[j] as *mut c_void);
        }
    }

    println!("After free(), program break is {:p}", unsafe { sbrk(0) });

    exit_success();
}
