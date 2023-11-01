//! # noel
//! An extremely fast GTF/GFF per gene Non-Overlapping Exon Length calculator (noel) written in Rust.
//!
//! ## Usage
//! ``` rust
//! Usage: noel[EXE] --i <GTF/GFF> --o <OUTPUT>
//!
//! Arguments:
//! --i <GTF/GFF>: GTF/GFF file
//! --o <OUTPUT>: .txt file
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
//! 1. include `noel = 0.1.0` or `noel = "*"` under `[dependencies]` in your `Cargo.toml` file or just run `cargo add noel` from the command line
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
//! let lengths: HashMap<String, u32> = noel(exons: HashMap<String, Vec<(u32, u32)>>)
//! ```

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::time::Instant;

use clap::{self, Parser};

pub use noel::*;

#[derive(Parser, Debug)]
#[command(
    author = "Alejandro Gonzales-Irribarren",
    version = "0.1.0",
    about = "An extremely fast GTF/GFF per gene Non-Overlapping Exon Length calculator written in Rust."
)]
struct Args {
    /// Input gtf/gff file
    #[clap(short, long)]
    input: PathBuf,
    /// Output file
    #[clap(short, long)]
    output: PathBuf,
}

fn main() {
    let start = Instant::now();
    let start_mem = max_mem_usage_mb();
    let args = Args::parse();

    let rdr = read_gtf(&args.input).unwrap();
    let mut exons: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
    let mut output = File::create(&args.output).unwrap();

    for line in rdr.lines() {
        let line = line.unwrap();

        if line.starts_with("#") {
            continue;
        }

        match Record::new(line) {
            Ok(record) => {
                let (start, end, gene_id) = record.info();
                exons
                    .entry(gene_id)
                    .or_insert(Vec::new())
                    .push((start, end));
            }
            Err(_) => continue,
        }
    }

    let genes = noel(exons).unwrap();

    genes.iter().for_each(|(gene, bp)| {
        writeln!(output, "{}\t{}", gene, bp).unwrap();
    });

    let elapsed = start.elapsed().as_secs_f64();
    let mem = (max_mem_usage_mb() - start_mem).max(0.0);
    eprintln!("Elapsed: {elapsed}s | Memory: {mem} MB");
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
