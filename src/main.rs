//! # noel
//! An extremely fast GTF/GFF per gene Non-Overlapping Exon Length calculator (noel) written in Rust.
//!
//! ## Usage
//! ``` rust
//! Usage: noel -g/--gtf <GTF/GFF> -o/--out <OUTPUT>
//!
//! Arguments:
//! -g/--gtf <GTF/GFF>: GTF/GFF file
//! -o/--out <OUTPUT>: .txt file
//! Options:
//! --help: print help
//! --version: print version
//! ```
//!
//! ## Installation
//!
//! to install noel on your system follow this steps:
//! 1. download rust: `curl https://sh.rustup.rs -sSf | sh` on unix, or go [here](https://www.rust-lang.org/tools/install) for other options
//! 2. run `cargo install noel` (make sure `~/.cargo/bin` is in your `$PATH` before running it)
//! 3. use `noel` with the required arguments
//!
//! ## Build
//!
//! to build noel from this repo, do:
//! 1. get rust (as described above)
//! 2. run `git clone https://github.com/alejandrogzi/noel.git && cd noel`
//! 3. run `cargo run --release <GTF/GFF> <OUTPUT>` (arguments are positional, so you do not need to specify --i/--o)
//!
//! ## Library
//!
//! 1. include `noel = 0.2.1` or `noel = "*"` under `[dependencies]` in your `Cargo.toml` file or just run `cargo add noel` from the command line
//! 2. the library name is `noel`, to use it just write:
//! ``` rust
//! use noel::{noel, noel_reader};
//! ```
//! or
//! ``` rust
//! use noel::*;
//! ```
//! 3. invoke
//! ``` rust
//! let exons = noel_reader(input: &PathBuf)
//! let lengths: Vec<(String, u32)> = noel(exons: HashMap<String, Vec<(u32, u32)>>)
//! ```

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

use num_cpus;

use clap::{self, Parser};

pub use noel::*;

#[derive(Parser, Debug)]
#[command(
    author = "Alejandro Gonzales-Irribarren",
    version = "0.2.1",
    about = "An extremely fast GTF/GFF per gene Non-Overlapping Exon Length calculator written in Rust."
)]
struct Args {
    /// Input gtf/gff file
    #[clap(
        short = 'g',
        long = "gtf",
        help = "GTF/GFF file",
        value_name = "GTF/GFF",
        required = true
    )]
    gtf: PathBuf,

    /// Output file
    #[clap(
        short = 'o',
        long = "out",
        help = "Output .txt file",
        value_name = "OUTPUT",
        required = true
    )]
    out: PathBuf,

    #[clap(
        short = 't',
        long,
        help = "Number of threads",
        value_name = "THREADS",
        default_value_t = num_cpus::get()
    )]
    threads: usize,
}

fn main() {
    let start = Instant::now();
    let start_mem = max_mem_usage_mb();
    let args = Args::parse();
    let mut output = BufWriter::new(File::create(&args.out).unwrap());

    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();

    let exons = noel_reader(&args.gtf).expect("Error reading GTF/GFF file");
    let genes = get_lengths(exons).expect("Error calculating exon lengths");

    genes.iter().for_each(|(gene, bp)| {
        writeln!(output, "{}\t{}", gene, bp).unwrap();
    });

    let elapsed = start.elapsed().as_secs_f64();
    let mem = (max_mem_usage_mb() - start_mem).max(0.0);
    println!("Elapsed: {:.4}s | Memory: {:.4} MB", elapsed, mem);
}

fn max_mem_usage_mb() -> f64 {
    let rusage = unsafe {
        let mut rusage = std::mem::MaybeUninit::uninit();
        libc::getrusage(libc::RUSAGE_SELF, rusage.as_mut_ptr());
        rusage.assume_init()
    };
    let maxrss = rusage.ru_maxrss as f64;
    if cfg!(target_os = "macos") {
        maxrss / 1024.0 / 1024.0
    } else {
        maxrss / 1024.0
    }
}
