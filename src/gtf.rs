mod attributes;
pub use attributes::*;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Record {
    feat: String,
    start: u32,
    end: u32,
    gene_id: String,
    transcript_id: String,
}

impl Record {
    pub fn new(line: String) -> Result<Self, CocoError> {
        if line.is_empty() {
            return Err(CocoError::Empty);
        }

        let fields = splitb(line)?;

        if fields[2] != "exon" {
            return Err(CocoError::NoExon);
        }

        let attributes = Attribute::parse(&fields[8])?;

        Ok(Record {
            feat: fields[2].to_string(),
            start: fields[3].parse().unwrap(),
            end: fields[4].parse().unwrap(),
            gene_id: attributes.gene_id().to_string(),
            transcript_id: attributes.transcript_id().to_string(),
        })
    }

    pub fn info(&self) -> (u32, u32, String, String) {
        (
            self.start,
            self.end,
            self.gene_id.clone(),
            self.transcript_id.clone(),
        )
    }

    pub fn feature(&self) -> &str {
        &self.feat
    }
}

fn splitb(line: String) -> Result<Vec<String>, CocoError> {
    let bytes = line.as_bytes().iter().enumerate();
    let mut start = 0;
    let mut entries = Vec::new();

    for (i, byte) in bytes {
        if *byte == b'\t' {
            let word = line[start..i].to_string();
            if !word.is_empty() {
                entries.push(word);
            }
            start = i + 1;
        }
    }
    entries.push(line[start..].to_string());
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_record() {
        let line = "1\thavana\texon\t2408530\t2408619\t.\t-\t0\tgene_id \"ENSG00000157911\"; gene_version \"11\"; transcript_id \"ENST00000508384\"; transcript_version \"5\"; exon_number \"3\"; gene_name \"PEX10\"; gene_source \"ensembl_havana\"; gene_biotype \"protein_coding\"; transcript_name \"PEX10-205\"; transcript_source \"havana\"; transcript_biotype \"protein_coding\"; protein_id \"ENSP00000464289\"; protein_version \"1\"; tag \"cds_end_NF\"; tag \"mRNA_end_NF\"; transcript_support_level \"3\";".to_string();
        let result = Record::new(line.clone());

        assert!(result.is_ok());

        let record = result.unwrap();
        assert_eq!(record.feat, "exon");
        assert_eq!(record.start, 2408530);
        assert_eq!(record.end, 2408619);
        assert_eq!(record.gene_id, "ENSG00000157911");
        assert_eq!(record.transcript_id, "ENST00000508384");
    }

    #[test]
    fn empty_record() {
        let line = "".to_string();
        let result = Record::new(line);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CocoError::Empty);
    }

    #[test]
    fn test_info() {
        let line = "1\thavana\texon\t2408530\t2408619\t.\t-\t0\tgene_id \"ENSG00000157911\"; gene_version \"11\"; transcript_id \"ENST00000508384\"; transcript_version \"5\"; exon_number \"3\"; gene_name \"PEX10\"; gene_source \"ensembl_havana\"; gene_biotype \"protein_coding\"; transcript_name \"PEX10-205\"; transcript_source \"havana\"; transcript_biotype \"protein_coding\"; protein_id \"ENSP00000464289\"; protein_version \"1\"; tag \"cds_end_NF\"; tag \"mRNA_end_NF\"; transcript_support_level \"3\";".to_string();
        let record = Record::new(line).unwrap();
        let (start, end, gene_id, transcript_id) = record.info();

        assert_eq!(start, 2408530);
        assert_eq!(end, 2408619);
        assert_eq!(gene_id, "ENSG00000157911");
        assert_eq!(transcript_id, "ENST00000508384");
    }
}
