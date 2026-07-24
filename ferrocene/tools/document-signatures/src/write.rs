use std::{io::{BufWriter, ErrorKind, Seek as _, Write as _}, path::Path};

use anyhow::{Context as _, Result};
use tempfile::NamedTempFile;

pub(crate) fn write_atomic<F>(dst: &Path, f: F ) -> Result<()>
where F: FnOnce(&mut BufWriter<NamedTempFile>) -> Result<()> {
    let parent = dst.parent().unwrap();
    std::fs::create_dir_all(parent)?;

    let mut output = BufWriter::new(NamedTempFile::new_in(parent)?);
    f(&mut output)?;

    output.flush()?;
    std::fs::rename(output.into_inner()?.path(), &dst)?;
    Ok(())
}

pub(crate) fn persist_atomic(src: NamedTempFile, dst: &Path) -> Result<()> {
    let src_path = src.path().to_path_buf();
    persist_inner(src, dst).context(format!("failed to rename {src_path:?} -> {dst:?}"))
}

fn persist_inner(src: NamedTempFile, dst: &Path) -> Result<()> {
    let parent = dst.parent().unwrap();
    std::fs::create_dir_all(parent)?;

    if let Err(e) = src.persist(dst) {
        if e.error.kind() == ErrorKind::CrossesDevices {
            copy_atomic(e.file, dst)?;
        } else {
            return Err(e.into());
        }
    }
    Ok(())
}

fn copy_atomic(mut src: NamedTempFile, dst: &Path) -> Result<()> {
    src.rewind()?;
    write_atomic(dst, |output| {
        std::io::copy(&mut src, output)?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_device_fallback_copies_from_start() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let mut src = NamedTempFile::new()?;
        src.write_all(b"stable archive contents")?;

        // Reproduce hashing the archive before persistence.
        src.rewind()?;
        std::io::copy(&mut src, &mut std::io::sink())?;
        assert_eq!(src.stream_position()?, src.as_file().metadata()?.len());

        let dst = dir.path().join("archive.tar.gz");
        copy_atomic(src, &dst)?;

        assert_eq!(std::fs::read(dst)?, b"stable archive contents");
        Ok(())
    }
}
