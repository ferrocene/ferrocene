/// Checks whether the provided path is the expected path of a package manifest, and if so returns
/// the information about that manifest derived from the path.
///
/// Package manifests are supposed to be located in `share/criticaltrust/${product}/${package}.json`.
pub(super) fn is_package_manifest(path: &str) -> Option<FoundPackageManifest<'_>> {
    let mut iter = ReverseSegmentsIter::new(path);

    let package = iter
        .next()
        .and_then(|s| s.strip_suffix(".json"))
        .filter(|s| !s.is_empty())?;
    let product = iter.next()?;
    iter.next().filter(|s| *s == "criticaltrust")?;
    iter.next().filter(|s| *s == "share")?;

    Some(FoundPackageManifest {
        package,
        product,
        prefix: iter.remaining(),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct FoundPackageManifest<'a> {
    pub(super) package: &'a str,
    pub(super) product: &'a str,
    pub(super) prefix: Option<&'a str>,
}

/// This iterator is similar to [`std::path::Components`], but makes some different choices:
///
/// * It operates over `str` instead of `Path`, to avoid the roundtrip through `Path`.
///
/// * It operates in the opposite direction, returning the rightmost segments first. This allows
///   the caller to stop as soon as the right path suffix is found and obtain the remainder.
///
/// * It returns `None` whenever a segment is empty, even if the path is not fully consumed. This
///   allows short-circuiting with `?` in the caller whenever an expected segment is empty.
///
/// * When calling `remaining`, it includes the trailing slash instead of omitting it. This allows
///   the rest of the code to join the prefix/remainder without having to insert an extra slash
///   (which could be different depending on the platform).
///
struct ReverseSegmentsIter<'a> {
    first: bool,
    cursor: &'a str,
}

impl<'a> ReverseSegmentsIter<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            first: true,
            cursor: input,
        }
    }

    fn remaining(&self) -> Option<&'a str> {
        none_if_empty(self.cursor)
    }
}

impl<'a> Iterator for ReverseSegmentsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let result = loop {
            // The `remaining` method needs to preserve the slash at the end, so we can't remove it
            // from the cursor. The code here can't have it though, otherwise when finding the
            // rightmost slash it will immediately find it. To avoid the problem, when accessing the
            // cursor slice inside this function we ignore the last byte if it's not the first segment.
            let end = self.cursor.len().saturating_sub(!self.first as usize);
            self.first = false;

            if let Some(pos) = self.cursor[..end].rfind('/') {
                let (new_cursor, result) = self.cursor[..end].split_at(pos + 1);
                self.cursor = new_cursor;
                if !result.is_empty() {
                    break result;
                }
            } else {
                let result = &self.cursor[..end];
                self.cursor = "";
                break result;
            }
        };

        none_if_empty(result)
    }
}

fn none_if_empty(input: &str) -> Option<&str> {
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_package_manifest() {
        const CASES: &[(&str, Option<FoundPackageManifest>)] = &[
            (
                "share/criticaltrust/product/package.json",
                Some(FoundPackageManifest {
                    package: "package",
                    product: "product",
                    prefix: None,
                }),
            ),
            (
                "share/criticaltrust/other-product/other-package.json",
                Some(FoundPackageManifest {
                    package: "other-package",
                    product: "other-product",
                    prefix: None,
                }),
            ),
            (
                "product/share/criticaltrust/product/package.json",
                Some(FoundPackageManifest {
                    package: "package",
                    product: "product",
                    prefix: Some("product/"),
                }),
            ),
            (
                "/usr/share/criticaltrust/product/package.json",
                Some(FoundPackageManifest {
                    package: "package",
                    product: "product",
                    prefix: Some("/usr/"),
                }),
            ),
            (
                "/usr/local/share/criticaltrust/product/package.json",
                Some(FoundPackageManifest {
                    package: "package",
                    product: "product",
                    prefix: Some("/usr/local/"),
                }),
            ),
            (
                "/home/pietro/.criticalup/files/share/criticaltrust/product/package.json",
                Some(FoundPackageManifest {
                    package: "package",
                    product: "product",
                    prefix: Some("/home/pietro/.criticalup/files/"),
                }),
            ),
            // Missing segments
            ("criticaltrust/product/package.json", None),
            ("share/product/package.json", None),
            ("share/criticaltrust/package.json", None),
            ("share/criticaltrust/product", None),
            // Wrong extension
            ("share/criticaltrust/product/package.toml", None),
            // Missing extension
            ("share/criticaltrust/product/package", None),
            // Only extension
            ("share/criticaltrust/product/.json", None),
            // Empty product name
            ("share/criticaltrust//package.json", None),
        ];

        for (case, expected) in CASES {
            assert_eq!(*expected, is_package_manifest(case), "\n  case: `{case}`\n");
        }
    }

    #[test]
    fn test_reverse_segments_iter_relative() {
        // The backslash should not be considered a directory separator, and it should instead be
        // treated like any other character. This won't be enough on Windows, but we're not
        // supporting it for the time being.
        let mut iter = ReverseSegmentsIter::new("foo/ba\\r//baz.json");
        assert_eq!(Some("foo/ba\\r//baz.json"), iter.remaining());

        assert_eq!(Some("baz.json"), iter.next());
        assert_eq!(Some("foo/ba\\r//"), iter.remaining());

        assert_eq!(Some("ba\\r"), iter.next());
        assert_eq!(Some("foo/"), iter.remaining());

        assert_eq!(Some("foo"), iter.next());
        assert_eq!(None, iter.remaining());

        assert_eq!(None, iter.next());
        assert_eq!(None, iter.remaining());
    }

    #[test]
    fn test_reverse_segments_iter_absolute() {
        // The backslash should not be considered a directory separator, and it should instead be
        // treated like any other character. This won't be enough on Windows, but we're not
        // supporting it for the time being.
        let mut iter = ReverseSegmentsIter::new("/foo/ba\\r//baz.json");
        assert_eq!(Some("/foo/ba\\r//baz.json"), iter.remaining());

        assert_eq!(Some("baz.json"), iter.next());
        assert_eq!(Some("/foo/ba\\r//"), iter.remaining());

        assert_eq!(Some("ba\\r"), iter.next());
        assert_eq!(Some("/foo/"), iter.remaining());

        assert_eq!(Some("foo"), iter.next());
        assert_eq!(Some("/"), iter.remaining());

        assert_eq!(None, iter.next());
        assert_eq!(None, iter.remaining());
    }
}
