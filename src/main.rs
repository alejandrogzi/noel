use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::{self, Parser};

pub use coco::*;

#[derive(Parser, Debug)]
#[command(
    author = "Alejandro Gonzales-Irribarren",
    version = "0.1.0",
    about = ""
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

    let genes = coco(exons).unwrap();

    genes.iter().for_each(|(gene, bp)| {
        writeln!(output, "{}\t{}", gene, bp).unwrap();
    });

    let elapsed = start.elapsed().as_secs_f64();
    eprintln!("Elapsed: {elapsed}s");
}

fn read_gtf(input: &Path) -> Option<BufReader<File>> {
    match input.extension() {
        Some(ext) if ext == "gtf" || ext == "gff" || ext == "gff3" => {
            let records = BufReader::new(File::open(input).unwrap());
            Some(records)
        }
        _ => {
            let err = "No gtf/gff file provided";
            eprintln!("{} {}", "Error:", err);
            std::process::exit(1)
        }
    }
}
