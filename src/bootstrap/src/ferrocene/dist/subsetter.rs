// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::builder::Builder;
use crate::utils::tarball::Tarball;

pub(super) struct Subsetter<'a> {
    builder: &'a Builder<'a>,
    name_prefix: String,
    output_prefix: PathBuf,

    pub(super) tarballs: BTreeMap<Option<String>, Rc<Tarball<'a>>>,
    directory_subset: Option<String>,
}

impl<'a> Subsetter<'a> {
    pub(super) fn new(builder: &'a Builder<'a>, name_prefix: &str, output_prefix: &str) -> Self {
        Self {
            builder,
            name_prefix: name_prefix.into(),
            output_prefix: output_prefix.into(),
            tarballs: BTreeMap::new(),
            directory_subset: None,
        }
    }

    pub(super) fn add_directory(&mut self, root: &Path, path: &Path) {
        let old_subset = self.directory_subset.clone();

        let subset_file = path.join("ferrocene-subset");
        match std::fs::read_to_string(&subset_file) {
            Ok(data) => self.directory_subset = Some(self.parse_subset_file(&subset_file, &data)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => panic!("failed to read ferrocene-subset in {}: {err}", path.display()),
        }

        for entry in std::fs::read_dir(path).unwrap() {
            let path = entry.as_ref().unwrap().path();
            if path.is_file() {
                self.add_file(root, &path);
            } else if path.is_dir() {
                self.add_directory(root, &path);
            }
        }

        self.directory_subset = old_subset;
    }

    pub(super) fn add_file(&mut self, root: &Path, path: &Path) {
        let mut subset = self.directory_subset.clone();

        // Allow overriding the directory subset with per-file subsets.
        let mut subset_file = path.to_path_buf();
        if subset_file.to_str().map(|p| !p.ends_with(".ferrocene-subset")).unwrap_or(true) {
            // If the file itself is the $name.ferrocene-subset file, include it in the same subset
            // it references.
            subset_file.as_mut_os_string().push(".ferrocene-subset");
        }
        match std::fs::read_to_string(&subset_file) {
            Ok(data) => subset = Some(self.parse_subset_file(&subset_file, &data)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => panic!("failed to read {}: {err}", subset_file.display()),
        }

        let tarball = match self.tarballs.get(&subset) {
            Some(tarball) => tarball.clone(),
            None => {
                let name = match &subset {
                    Some(name) => format!("{}-{name}", self.name_prefix),
                    None => self.name_prefix.clone(),
                };
                let tarball = Rc::new(Tarball::new_targetless(self.builder, &name));
                self.tarballs.insert(subset, tarball.clone());
                tarball
            }
        };

        let relative = path.strip_prefix(root).unwrap();
        let mode = if self.is_executable(path) { 0o755 } else { 0o644 };
        tarball.add_file(path, self.output_prefix.join(relative).parent().unwrap(), mode);
    }

    #[cfg(unix)]
    fn is_executable(&self, path: &Path) -> bool {
        use std::os::unix::prelude::PermissionsExt;
        std::fs::metadata(path).unwrap().permissions().mode() & 0o111 > 0
    }
    #[cfg(not(unix))]
    fn is_executable(&self, _: &Path) -> bool {
        panic!("generating accurate tarballs on non-unix-like platforms is not yet supported");
    }

    fn parse_subset_file(&self, path: &Path, contents: &str) -> String {
        let mut lines = contents
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with('#'))
            .filter(|line| !line.is_empty());

        let Some(subset) = lines.next() else {
            panic!("no content in subset file {}", path.display());
        };
        if !subset.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            panic!("subset name {subset:?} contains invalid chars (in {})", path.display());
        }
        if lines.next().is_some() {
            panic!("multiple subset names in {}", path.display());
        }

        subset.into()
    }

    pub(super) fn into_tarballs(self) -> impl Iterator<Item = Tarball<'a>> {
        self.tarballs.into_values().map(|tarball| Rc::try_unwrap(tarball).map_err(|_| ()).unwrap())
    }
}
