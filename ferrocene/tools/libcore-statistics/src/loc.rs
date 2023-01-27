use anyhow::Error;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokei::{Languages, Config, LanguageType};

const REMOVE_SYMBOLS: &[char] = &[',', ';', '}', ']', ')'];

pub(crate) struct LOC {
    base: PathBuf,
    files: HashMap<PathBuf, CachedFile>,
}

impl LOC {
    pub(crate) fn new(base: &Path) -> Self {
        Self {
            base: base.into(),
            files: HashMap::new(),
        }
    }

    pub(crate) fn stats_for(&mut self, path: &Path, begin: usize, end: usize) -> Result<usize, Error> {
        if !self.files.contains_key(path) {
            self.load_file(path)?;
        }

        let file = &self.files[path];
        let contents = &file.contents[(file.line_ranges[begin - 1].0..file.line_ranges[end - 1].1)];

        // tokei unfortunately doesn't have an API accepting a string, only works by providing a
        // real file to it. We thus need to create a temporary file just for this :/
        let temp = tempfile::Builder::new().suffix(".rs").tempfile()?;
        std::fs::write(temp.path(), contents)?;

        // Note that the statistics by tokei here will be different than the statistics of tokei in
        // the CLI, since we do some pre-processing to remove the symbols in REMOVE_SYMBOLS before
        // passing the contents to tokei.
        let mut config = Config::default();
        config.treat_doc_strings_as_comments = Some(true);
        let mut languages = Languages::new();
        languages.get_statistics(&[temp.path()], &[], &config);

        Ok(languages.get(&LanguageType::Rust).map(|lang| lang.code).unwrap_or(0))
    }

    fn load_file(&mut self, path: &Path) -> Result<(), Error> {
        let mut contents = std::fs::read_to_string(self.base.join(path))?;

        // We first remove all the symbols that are likely to be alone in a line, so that lines
        // with just those symbols will be counted as blank by tokei.
        for symbol in REMOVE_SYMBOLS {
            contents = contents.replace(*symbol, "");
        }

        let contents = contents.into_bytes();

        let mut start = 0;
        let mut line_ranges = contents
            .iter()
            .enumerate()
            .filter(|(_, byte)| **byte == b'\n')
            .map(|(i, _)| {
                let range = (start, i);
                start = i + 1;
                range
            })
            .collect::<Vec<_>>();
        line_ranges.push((start, contents.len()));

        self.files.insert(
            path.into(),
            CachedFile {
                contents,
                line_ranges,
            },
        );

        Ok(())
    }
}

struct CachedFile {
    contents: Vec<u8>,
    line_ranges: Vec<(usize, usize)>,
}
