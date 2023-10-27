use std::collections::{BTreeMap, HashMap};

pub mod gff;
pub use gff::*;

pub mod gtf;
pub use gtf::*;

pub fn coco(exons: BTreeMap<String, Vec<(u32, u32)>>) -> HashMap<String, u32> {
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
    genes
}