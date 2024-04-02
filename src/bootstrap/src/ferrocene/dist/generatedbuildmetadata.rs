// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::fs;

use serde_json::json;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::t;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct GenerateBuildMetadata;

impl Step for GenerateBuildMetadata {
    type Output = ();
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-build-metadata")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(GenerateBuildMetadata);
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        if builder.config.dry_run() {
            return;
        }

        let git_info = builder.rust_info();
        let (Some(sha1_full), Some(sha1_short)) = (&git_info.sha(), &git_info.sha_short()) else {
            panic!("generating the build metadata requires git information");
        };

        let dist_dir = "build/dist";

        let ferrocene_version = t!(fs::read_to_string("ferrocene/version"));
        let ferrocene_version = ferrocene_version.trim();
        let ferrocene_channel = &builder.config.ferrocene_raw_channel;
        let src_version = t!(fs::read_to_string("src/version"));
        let src_version = src_version.trim();

        // Perform validation on the contents of the version field, to avoid generating
        // artifacts that will break the release process.
        if ferrocene_channel == "rolling" && ferrocene_version != "rolling" {
            panic!(
                "error: ferrocene/version must be 'rolling' when ferrocene/ci/channel is 'rolling' but it was '{ferrocene_version}'"
            );
        }

        let channel = crate::ferrocene::ferrocene_channel(builder, ferrocene_version);

        // Whenever the contents of this JSON file change, even just adding new fields,
        // make sure to increase the metadata version number and update publish-release
        // accordingly. Note that new releases *won't* be made until publish-release and
        // this use the same version number.
        let data = json!({
            "metadata_version": 4,
            "rust_version": src_version,
            "rust_channel": builder.config.channel,
            "ferrocene_version": ferrocene_version,
            "ferrocene_channel": ferrocene_channel,
            "channel": channel,
            "sha1_full": sha1_full,
            "sha1_short": sha1_short,
        })
        .to_string();

        builder.create_dir(dist_dir.as_ref());

        builder.create(format!("{dist_dir}/ferrocene-ci-metadata.json").as_ref(), &data);

        // Add the list of packages to include in the release to the artifacts, so that
        // publish-release knows what to expect for this commit.
        builder.copy_link_to_folder("ferrocene/packages.toml".as_ref(), dist_dir.as_ref());
    }
}
