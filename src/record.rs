mod attr;
pub use attr::*;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Record {
    pub feat: String,
    pub start: u32,
    pub end: u32,
    pub gene_id: String,
}

impl Record {
    pub fn new(line: &str) -> Result<Self, NoelError> {
        if line.is_empty() || line.starts_with('#') {
            return Err(NoelError::Empty);
        }

        let fields = line.split('\t').collect::<Vec<&str>>();

        if fields[2] != "exon" {
            return Err(NoelError::NoExon);
        }

        let gene_id = Attribute::parse(fields[8])?;

        Ok(Record {
            feat: fields[2].to_string(),
            start: fields[3].parse().unwrap(),
            end: fields[4].parse().unwrap(),
            gene_id: gene_id.gene_id().to_string(),
        })
    }

    pub fn info(&self) -> (u32, u32, String) {
        (self.start, self.end, self.gene_id.clone())
    }

    pub fn feature(&self) -> &str {
        &self.feat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_record() {
        let line = "1\thavana\texon\t2408530\t2408619\t.\t-\t0\tgene_id \"ENSG00000157911\"; gene_version \"11\"; transcript_id \"ENST00000508384\"; transcript_version \"5\"; exon_number \"3\"; gene_name \"PEX10\"; gene_source \"ensembl_havana\"; gene_biotype \"protein_coding\"; transcript_name \"PEX10-205\"; transcript_source \"havana\"; transcript_biotype \"protein_coding\"; protein_id \"ENSP00000464289\"; protein_version \"1\"; tag \"cds_end_NF\"; tag \"mRNA_end_NF\"; transcript_support_level \"3\";".to_string();
        let result = Record::new(&line);

        assert!(result.is_ok());

        let record = result.unwrap();
        assert_eq!(record.feat, "exon");
        assert_eq!(record.start, 2408530);
        assert_eq!(record.end, 2408619);
        assert_eq!(record.gene_id, "ENSG00000157911");
    }

    #[test]
    fn empty_record() {
        let line = "".to_string();
        let result = Record::new(&line);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NoelError::Empty);
    }

    #[test]
    fn test_info() {
        let line = "1\thavana\texon\t2408530\t2408619\t.\t-\t0\tgene_id \"ENSG00000157911\"; gene_version \"11\"; transcript_id \"ENST00000508384\"; transcript_version \"5\"; exon_number \"3\"; gene_name \"PEX10\"; gene_source \"ensembl_havana\"; gene_biotype \"protein_coding\"; transcript_name \"PEX10-205\"; transcript_source \"havana\"; transcript_biotype \"protein_coding\"; protein_id \"ENSP00000464289\"; protein_version \"1\"; tag \"cds_end_NF\"; tag \"mRNA_end_NF\"; transcript_support_level \"3\";".to_string();
        let record = Record::new(&line).unwrap();
        let (start, end, gene_id) = record.info();

        assert_eq!(start, 2408530);
        assert_eq!(end, 2408619);
        assert_eq!(gene_id, "ENSG00000157911");
    }
}
