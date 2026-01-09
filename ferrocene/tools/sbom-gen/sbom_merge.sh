# Merges all generated SBOMs that end with '_sbom.json' into one.

set -e

DST_DIR=target/sboms
COMBINED_SBOM=ferrocene_cdx_sbom.json

cd $DST_DIR

cyclonedx merge --input-files $(ls *"_sbom.json" 2>/dev/null | tr '\n' ' ') --output-version=v1_6 --output-file $COMBINED_SBOM
