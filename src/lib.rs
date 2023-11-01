//! A small library for computing the number of base pairs in a gene
//!
//! to include noel as a library and use it within your project follow these steps:
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
//!

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub mod record;
pub use record::*;

pub fn noel(exons: HashMap<String, Vec<(u32, u32)>>) -> Option<HashMap<String, u32>> {
    let mut genes = HashMap::new();
    for (gene, exons) in exons.iter() {
        let (min_start, max_end) = exons.iter().fold((u32::MAX, 0), |acc, &(start, end)| {
            (acc.0.min(start), acc.1.max(end))
        });

        let mut bp = vec![0; (max_end - min_start + 1) as usize];

        for &(start, end) in exons.iter() {
            for i in (start - min_start)..(end - min_start + 1) {
                bp[i as usize] = 1;
            }
        }

        let total_bp: u32 = bp.iter().sum();
        genes.insert(gene.clone(), total_bp);
    }
    Some(genes)
}

pub fn noel_reader(input: &PathBuf) -> Option<HashMap<String, Vec<(u32, u32)>>> {
    let rdr = read_gtf(input)?;
    let mut exons: HashMap<String, Vec<(u32, u32)>> = HashMap::new();

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
    Some(exons)
}

pub fn read_gtf(input: &Path) -> Option<BufReader<File>> {
    match input.extension() {
        Some(ext) if ext == "gtf" || ext == "gff" || ext == "gff3" => {
            let records = BufReader::new(File::open(input).unwrap());
            Some(records)
        }
        _ => {
            let err = "No gtf/gff file provided. Check the extension of your file.";
            eprintln!("{} {}", "Error:", err);
            None
        }
    }
}
