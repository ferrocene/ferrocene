# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# Merges all generated SBOMs that end with '_sbom.json' into one.

set -euo pipefail

DST_DIR=target/sboms
COMBINED_SBOM=ferrocene_cdx_sbom.json

cd $DST_DIR

cyclonedx merge --name="Ferrocene" --version=$(cat ../../ferrocene/version) --hierarchical  --input-files $(ls *"_sbom.json" 2>/dev/null | tr '\n' ' ') --output-version=v1_6 --output-file $COMBINED_SBOM
