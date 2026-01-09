#!/bin/bash
# bash needed for pushd/popd

set -e

# Ensure that yarn lockfile is still up to date
# Cargo and uv SBOM tools are able to resolve this themselves.
yarn install --immutable

DST_DIR=target/sboms

ROOT_CARGO_SBOM=root_cargo_cdx_sbom.json
ROOT_YARN_SBOM=root_yarn_cdx_sbom.json
FERROCENE_DOC_UV_SBOM=ferrocene_doc_uv_cdx_sbom.json
BACKTRACE_RS_CARGO_SBOM=backtrace-rs_cargo_cdx_sbom.json
FERROCENE_LIBC_CARGO_SBOM=ferrocene_libc_cargo_cdx_sbom.json
FERROCENE_TOOLS_CARGO_SBOM=ferrocene_tools_cargo_cdx_sbom.json
FERROCENE_AUTOMATIONS_COMMON_UV_SBOM=ferrocene_automations-common_uv_cdx_sbom.json
LIBRARY_CARGO_SBOM=library_cargo_cdx_sbom.json
STDARCH_CARGO_SBOM=library_stdarch_cargo_cdx_sbom.json
BOOTSTRAP_CARGO_SBOM=bootstrap_cargo_cdx_sbom.json
LIBRUSTDOC_CARGO_SBOM=librustdoc_cargo_cdx_sbom.json
RUSTC_PERF_CARGO_SBOM=rustc-perf_cargo_cdx_sbom.json

COMBINED_SBOM=ferrocene_cdx_sbom.json

mkdir -p $DST_DIR

cargo sbom --output-format=cyclone_dx_json_1_6 > $ROOT_CARGO_SBOM && mv $ROOT_CARGO_SBOM $DST_DIR/$ROOT_CARGO_SBOM
yarn cyclonedx -o $ROOT_YARN_SBOM && mv $ROOT_YARN_SBOM $DST_DIR/$ROOT_YARN_SBOM

pushd ferrocene/doc
uv export --format=cyclonedx1.5 --preview-features sbom-export --all-packages -o $FERROCENE_DOC_UV_SBOM && mv $FERROCENE_DOC_UV_SBOM ../../$DST_DIR/$FERROCENE_DOC_UV_SBOM
popd

pushd ferrocene/library/backtrace-rs
cargo sbom --output-format=cyclone_dx_json_1_6 > $BACKTRACE_RS_CARGO_SBOM && mv $BACKTRACE_RS_CARGO_SBOM ../../../$DST_DIR/$BACKTRACE_RS_CARGO_SBOM
popd

pushd ferrocene/library/libc
cargo sbom --output-format=cyclone_dx_json_1_6 > $FERROCENE_LIBC_CARGO_SBOM && mv $FERROCENE_LIBC_CARGO_SBOM ../../../$DST_DIR/$FERROCENE_LIBC_CARGO_SBOM
popd

pushd ferrocene/tools
cargo sbom --output-format=cyclone_dx_json_1_6 > $FERROCENE_TOOLS_CARGO_SBOM && mv $FERROCENE_TOOLS_CARGO_SBOM ../../$DST_DIR/$FERROCENE_TOOLS_CARGO_SBOM
popd

pushd ferrocene/tools/automations-common
uv export --format=cyclonedx1.5 --preview-features sbom-export --all-packages -o $FERROCENE_AUTOMATIONS_COMMON_UV_SBOM && mv $FERROCENE_AUTOMATIONS_COMMON_UV_SBOM ../../../$DST_DIR/$FERROCENE_AUTOMATIONS_COMMON_UV_SBOM
popd

pushd library
cargo +nightly sbom --output-format=cyclone_dx_json_1_6 > $LIBRARY_CARGO_SBOM && mv $LIBRARY_CARGO_SBOM ../$DST_DIR/$LIBRARY_CARGO_SBOM
popd

pushd library/stdarch
cargo +nightly sbom --output-format=cyclone_dx_json_1_6 > $STDARCH_CARGO_SBOM && mv $STDARCH_CARGO_SBOM ../../$DST_DIR/$STDARCH_CARGO_SBOM
popd

pushd src/bootstrap
cargo sbom --output-format=cyclone_dx_json_1_6 > $BOOTSTRAP_CARGO_SBOM && mv $BOOTSTRAP_CARGO_SBOM ../../$DST_DIR/$BOOTSTRAP_CARGO_SBOM
popd

pushd src/librustdoc
cargo sbom --output-format=cyclone_dx_json_1_6 > $LIBRUSTDOC_CARGO_SBOM && mv $LIBRUSTDOC_CARGO_SBOM ../../$DST_DIR/$LIBRUSTDOC_CARGO_SBOM
popd

pushd src/tools/rustc-perf
cargo sbom --output-format=cyclone_dx_json_1_6 > $RUSTC_PERF_CARGO_SBOM && mv $RUSTC_PERF_CARGO_SBOM ../../../$DST_DIR/$RUSTC_PERF_CARGO_SBOM
popd
