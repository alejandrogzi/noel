#![allow(dead_code)]
use std::collections::HashMap;
use thiserror::Error;

const SEP: &str = ".";

#[derive(Debug, PartialEq)]
pub struct Attribute {
    gene_id: String,
}

impl Attribute {
    pub fn parse(line: &str) -> Result<Attribute, NoelError> {
        if !line.is_empty() {
            let mut attributes: HashMap<String, String> = HashMap::new();
            let bytes = line.as_bytes().iter().enumerate();

            let mut start = 0;
            for (i, byte) in bytes {
                if *byte == b';' {
                    let word = &line[start..i];
                    if !word.is_empty() && word.starts_with("gene_id") {
                        let (key, value) = get_pair(word).ok_or(NoelError::Parse)?;
                        attributes.insert(key, value);
                    }
                    start = i + 1;
                }
            }

            let gene_id = attributes.get("gene_id").ok_or(NoelError::Invalid);

            Ok(Attribute {
                gene_id: gene_id?.to_string(),
            })
        } else {
            Err(NoelError::Empty)
        }
    }

    pub fn gene_id(&self) -> &str {
        &self.gene_id
    }
}

fn get_pair(line: &str) -> Option<(String, String)> {
    let mut bytes = line.as_bytes().iter();
    let i = if let Some(pos) = bytes.position(|b| *b == b' ' || *b == b'=') {
        pos
    } else {
        line.len()
    };

    let key = &line[..i];
    let value = get_gene(*&line[i + 1..].trim_matches('"'), &SEP)?;

    Some((key.to_string(), value.to_string()))
}

fn get_gene(gene: &str, sep: &str) -> Option<String> {
    let bytes = gene.as_bytes().iter().enumerate();
    let start = 0;
    let mut word = String::new();

    for (i, byte) in bytes {
        if *byte == sep.as_bytes()[0] {
            word.push_str(&gene[start..i]);
        }
    }
    if !word.is_empty() {
        return Some(word);
    } else {
        return Some(gene.to_string());
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum NoelError {
    #[error("Empty line")]
    Empty,
    #[error("Invalid GTF line")]
    Invalid,
    #[error("Not an exon")]
    NoExon,
    #[error("Parsing error")]
    Parse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_gtf_attributes() {
        let input = "gene_id \"ABC\"; transcript_id \"XYZ\"; exon_number \"1\"; exon_id \"123\";"
            .to_string();
        let attr = Attribute::parse(&input).unwrap();

        assert_eq!(attr.gene_id(), "ABC");
    }

    #[test]
    fn valid_gff_attributes() {
        let input = "ID=exon:ENST00000456328.2.1;Parent=ENST00000456328.2;gene_id=ENSG00000290825.1;transcript_id=ENST00000456328.2,exon_number=1".to_string();
        let attr = Attribute::parse(&input).unwrap();

        assert_eq!(attr.gene_id(), "ENSG00000290825");
    }

    #[test]
    fn invalid_attributes() {
        let input = "transcript_id \"XYZ\"; exon_number \"1\";".to_string();
        let result = Attribute::parse(&input);

        assert_eq!(result.unwrap_err(), NoelError::Invalid);
    }

    #[test]
    fn valid_gene() {
        let gene = "ENSG00000290825.1".to_string();
        let result = get_gene(&gene, &SEP);
        assert_eq!(result.unwrap(), "ENSG00000290825");
    }

    #[test]
    fn invalid_gene() {
        let gene = "ENSG00000290825.1".to_string();
        let sep = ".".to_string();
        let result = get_gene(&gene, &sep);
        assert_ne!(result.unwrap(), "ENSG00000290825.1");
    }

    #[test]
    fn get_gff_pair() {
        let line = "gene_id=ENSG00000290825.1";
        let tup = ("gene_id".to_string(), "ENSG00000290825.1".to_string());

        let pair = get_pair(line).unwrap();
        assert_eq!(tup, pair);
    }
}
