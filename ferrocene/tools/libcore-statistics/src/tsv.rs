use anyhow::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Tab-separated values
#[allow(clippy::upper_case_acronyms)]
pub struct TSV<const N: usize> {
    writer: BufWriter<File>,
}

impl<const N: usize> TSV<N> {
    pub fn new(path: &Path, header: [&str; N]) -> Result<Self, Error> {
        let mut tsv = Self {
            writer: BufWriter::new(File::create(path)?),
        };
        tsv.add(header)?;

        Ok(tsv)
    }

    pub fn add(&mut self, line: [&str; N]) -> Result<(), Error> {
        for (i, heading) in line.iter().enumerate() {
            if i != 0 {
                self.writer.write_all(b"\t")?;
            }
            self.writer.write_all(heading.as_bytes())?;
        }
        self.writer.write_all(b"\n")?;
        Ok(())
    }
}
