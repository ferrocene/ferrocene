driver_impl_ice = the compiler unexpectedly panicked. this is a bug.
driver_impl_ice_bug_report = we would appreciate a bug report: {$bug_report_url}
driver_impl_ice_bug_report_internal_feature = using internal features is not supported and expected to cause internal compiler errors when used incorrectly
driver_impl_ice_bug_report_outdated =
    it seems that this compiler `{$version}` is outdated, a newer nightly should have been released in the mean time
    .update = please consider running `rustup update nightly` to update the nightly channel and check if this problem still persists
    .url = if the problem still persists, we would appreciate a bug report: {$bug_report_url}
driver_impl_ice_exclude_cargo_defaults = some of the compiler flags provided by cargo are hidden

driver_impl_ice_flags = compiler flags: {$flags}
driver_impl_ice_path = please attach the file at `{$path}` to your bug report
driver_impl_ice_path_error = the ICE couldn't be written to `{$path}`: {$error}
driver_impl_ice_path_error_env = the environment variable `RUSTC_ICE` is set to `{$env_var}`
driver_impl_ice_version = rustc {$version} running on {$triple}

driver_impl_rlink_empty_version_number = The input does not contain version number

driver_impl_rlink_encoding_version_mismatch = .rlink file was produced with encoding version `{$version_array}`, but the current version is `{$rlink_version}`

driver_impl_rlink_no_a_file = rlink must be a file

driver_impl_rlink_rustc_version_mismatch = .rlink file was produced by rustc version `{$rustc_version}`, but the current version is `{$current_version}`

driver_impl_rlink_unable_to_read = failed to read rlink file: `{$err}`

driver_impl_rlink_wrong_file_type = The input does not look like a .rlink file
