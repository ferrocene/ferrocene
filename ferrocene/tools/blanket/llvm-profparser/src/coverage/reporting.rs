use crate::coverage::*;
use std::collections::BTreeMap;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Default)]
pub struct CoverageReport {
    pub files: BTreeMap<PathBuf, CoverageResult>,
}

#[derive(Clone, Debug, Default)]
pub struct CoverageResult {
    pub hits: BTreeMap<SourceLocation, usize>,
}

impl CoverageReport {
    pub fn apply_remapping(&mut self, remapping: &PathRemapping) {
        let inputs = self.files.keys().cloned().collect::<Vec<_>>();
        for path in &inputs {
            if path.starts_with(&remapping.source) {
                let end = path.strip_prefix(&remapping.source).unwrap();
                let new_path = remapping.dest.join(end);
                if let Some(values) = self.files.remove(path) {
                    self.files.insert(new_path, values);
                } else {
                    unreachable!();
                }
            }
        }
    }
}

impl CoverageResult {
    pub fn max_hits(&self) -> usize {
        self.hits.values().max().copied().unwrap_or_default()
    }

    pub fn insert(&mut self, loc: SourceLocation, count: usize) {
        self.hits
            .entry(loc)
            .and_modify(|x| *x = x.saturating_add(count))
            .or_insert(count);
    }

    /// For line coverage just finds first region that mentions this line
    pub fn hits_for_line(&self, line: usize) -> Option<usize> {
        self.hits
            .iter()
            .find(|(k, _)| k.line_start <= line && k.line_end >= line)
            .map(|(_, v)| *v)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum RemappingParseError {
    #[error("Path remapping is empty")]
    EmptyRemapping,
    #[error("Missing source path (remapping should be 'source,dest'")]
    MissingSourcePath,
    #[error("Missing destination path (remapping should be 'source,dest'")]
    MissingDestinationPath,
    #[error("Too many paths, two expected")]
    TooManyPaths,
}

/// Map the paths in the coverage data to local source file paths. This allows
/// you to generate the coverage data on one machine, and then use llvm-cov on a
/// different machine where you have the same files on a different path.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PathRemapping {
    /// Path used on source machine
    source: PathBuf,
    /// Path used in destination machine
    dest: PathBuf,
}

impl FromStr for PathRemapping {
    type Err = RemappingParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths: Vec<&str> = s.split(',').collect();

        if paths.is_empty() {
            Err(Self::Err::EmptyRemapping)
        } else if paths.len() > 2 {
            Err(Self::Err::TooManyPaths)
        } else if paths[0].is_empty() {
            Err(Self::Err::MissingSourcePath)
        } else if paths[1].is_empty() {
            Err(Self::Err::MissingDestinationPath)
        } else {
            let source = PathBuf::from(paths[0]);
            let dest = PathBuf::from(paths[1]);
            Ok(Self { source, dest })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_remapping() {
        let mut report = CoverageReport::default();

        report
            .files
            .insert(PathBuf::from("/root/src/lib.rs"), CoverageResult::default());
        report.files.insert(
            PathBuf::from("/home/root/src/lib.rs"),
            CoverageResult::default(),
        );

        let remapping = PathRemapping {
            source: PathBuf::from("/root/src"),
            dest: PathBuf::from("/home/me/src"),
        };

        report.apply_remapping(&remapping);

        assert_eq!(report.files.len(), 2);
        assert!(report
            .files
            .contains_key(&PathBuf::from("/home/me/src/lib.rs")));
        assert!(report
            .files
            .contains_key(&PathBuf::from("/home/root/src/lib.rs")));
    }
}
