# Generate SBOM for the Ferrocene Repository

## Prerequisite

- cyclonedx-cli (https://github.com/CycloneDX/cyclonedx-cli)
- Cargo
- `cargo install cargo-sbom`
- uv
- npm
- yarn (`npm install -g corepack`)

  According to the [installation guide](https://yarnpkg.com/getting-started/install),
  you currently need to run `corepack enable` to get the correct yarn version.
  Switch to the new yarn version by running `yarn set version stable`.
  Alternatively, run `corepack install`, because the correct yarn version is already set in the package.json file.

  Then run `yarn install --immutable` to get the packages, but without changing the lockfile.

  **Note:** This may fail if installation would require the lockfile to change, which should only be the case if new packages were added. In this case, run the command without `--immutable` since changes are correctly expected.

## Currently Excluded

The SBOM does not include dependencies of the following packages:
- `compiler/rustc_codegen_cranelift` ... likely not relevant for Ferrocene, because Cranelift as backend is not supported as of now
- `compiler/rustc_codegen_gcc` ... likely not relevant for Ferrocene, because GCC as backend is not supported as of now
- `ferrocene/tools/flip-link` ... external Rust tool for embedded projects. Best to generate an SBOM for flip-link separately
- `library/backtrace` ... covered by `ferrocene/library/backtrace-rs`
- `src/doc/book`
- `src/gcc` ... likely not relevant for Ferrocene, because GCC as backend is not supported as of now
- `src/llvm-project` ... No existing SBOM, but maybe used LLVM version is enough?
- `src/tools/cargo`
- `src/tools/enzyme` ... external CPP application. Couldn't find an existing SBOM 
- `src/tools/error_index_generator` ... used by rustbook
- `src/tools/rust-analyzer`
- `src/tools/rustbook`

## Steps
### Automatic Creation

To automatically generate the SBOM file as done in the Manual Creation section,
run the shell script `sbom_gen.sh` from the root of the repository.
This will create the SBOM file in the `target/sboms` folder.

```bash
./ferrocene/tools/sbom-gen/sbom_gen.sh
```

**Note:** Requires prerequisites and bash to be installed.

### Manual Creation

1. At repository root: 

   1. Run: `cargo sbom --output-format=cyclone_dx_json_1_6 > root_cargo_cdx_sbom.json`

      This generates a SBOM file for the Cargo.toml file located at root level.
      It contains:

      - Everything from the `compiler` folder except the excluded crates `rustc_codegen_cranelift` and `rustc_codegen_gcc`.

        Cranelift and GCC are Rust backends that are currently not used for Ferrocene, so excluding them from our SBOM is fine.

      - Everything from `src` except
        - `bootstrap`
        - `gcc`
        - `librustdoc`
        - `llvm-project`
        - `tools/cargo`
        - `tools/enzyme`
        - `tools/error_index_generator`
        - `tools/rust-analyzer`
        - `tools/rustbook`
        - `tools/rustc-perf`
        
   2. Run: `yarn dlx -q @cyclonedx/yarn-plugin-cyclonedx -o root_yarn_cdx_sbom.json`

      This generates a SBOM file for the yarn.lock file located at root level.
      The packages must be installed using `yarn install --immutable` before running this command as mentioned in the prerequisite section.

2. At `ferrocene/doc` run: `uv export --format=cyclonedx1.5 --preview-features sbom-export --all-packages -o ferrocene_doc_uv_cdx_sbom.json`

   This generates a SBOM file for the uv.lock file located in the ferrocene/doc folder.

   **Note:** Needs uv version >= 0.9.x.

3. At `ferrocene/library/backtrace-rs` run: `cargo sbom --output-format=cyclone_dx_json_1_6 > backtrace-rs_cargo_cdx_sbom.json`

   **Note:** Might be replaced by the official submodule at `library/backtrace`. In that case, the submodule must be checked out and the SBOM must be generated for it instead.

4. At `ferrocene/library/libc` run: `cargo sbom --output-format=cyclone_dx_json_1_6 > ferrocene_libc_cargo_cdx_sbom.json`

5. At `ferrocene/tools` run: `cargo sbom --output-format=cyclone_dx_json_1_6 > ferrocene_tools_cargo_cdx_sbom.json`

6. At `ferrocene/tools/automations-common` run: `uv export --format=cyclonedx1.5 --all-packages -o ferrocene_automations-common_uv_cdx_sbom.json`

7. At `library`:

   1. Run: `cargo +nightly sbom --output-format=cyclone_dx_json_1_6 > lib_cargo_cdx_sbom.json`

      This generates a SBOM for the Cargo.toml file under `library` except `stdarch` and `windows-targets`.
      The `windows-targets` Crate has no other Crate dependencies and is already listed in the root SBOM,
      so no need for its own SBOM file.

      The generated SBOM file does not contain the `backtrace-rs` crate, because it is included via `path` in `std/src/lib.rs` to the local version of the crate located at `ferrocene/library/backtrace-rs`.
      Therefore, another SBOM file must be generated for the local crate.

      **Note:** The `+nightly` is needed, because the file contains the nightly feature `profile-rustflags`. 

   2. Change to the `stdarch` folder and run: `cargo +nightly sbom --output-format=cyclone_dx_json_1_6 > lib_stdarch_cargo_cdx_sbom.json`

      This generates a SBOM for the Cargo.toml file under `stdarch`. The excluded crates are generated and do not exist by default. Therefore, no need to generate a SBOM file for them.

      **Note:** The `+nightly` is needed, because the file contains the nightly feature `profile-rustflags`. 

8. At `src/bootstrap` run: `cargo sbom --output-format=cyclone_dx_json_1_6 > bootstrap_cargo_cdx_sbom.json`
9. At `src/librustdoc` run: `cargo sbom --output-format=cyclone_dx_json_1_6 > librustdoc_cargo_cdx_sbom.json`
10. At `src/tools/rustc-perf` run: `cargo sbom --output-format=cyclone_dx_json_1_6 > rustc-perf_cargo_cdx_sbom.json`

11. Combine all generated SBOM files

    Requires to have `cyclonedx-cli` installed.

    ```
    cyclonedx merge --input-files <space separated list of SBOM files> --output-file ferrocene_sbom.json --output-version=v1_6
    ```

    **Note:** SBOM files generated from the steps above may be grouped into different files to highlight, that some SBOMs only contain documentation dependencies and do not affect compiler code directly.
