# Changelog

## [Unreleased]
## [0.2.167](https://github.com/rust-lang/libc/compare/0.2.166...0.2.167) - 2024-11-28

### Added

- Solarish: add `st_fstype` to `stat` <https://github.com/rust-lang/libc/pull/4145>
- Trusty: Add `intptr_t` and `uintptr_t` ([#4161](https://github.com/rust-lang/libc/pull/4161))

### Fixed

- Fix the build with `rustc-dep-of-std` <https://github.com/rust-lang/libc/pull/4158>
- Wasi: Add back unsafe block for `clockid_t` static variables ([#4157](https://github.com/rust-lang/libc/pull/4157))

### Cleanup

- Create an internal prelude <https://github.com/rust-lang/libc/pull/4161>
- Fix `unused_qualifications`<https://github.com/rust-lang/libc/pull/4132>

### Other

- CI: Check various FreeBSD versions ([#4159](https://github.com/rust-lang/libc/pull/4159))
- CI: add a timeout for all jobs <https://github.com/rust-lang/libc/pull/4164>
- CI: verify MSRV for `wasm32-wasi` <https://github.com/rust-lang/libc/pull/4157>
- Migrate to the 2021 edition <https://github.com/rust-lang/libc/pull/4132>

### Removed

- Remove one unused import after the edition 2021 bump

## [0.2.166](https://github.com/rust-lang/libc/compare/0.2.165...0.2.166) - 2024-11-26

### Fixed

This release resolves two cases of unintentional breakage from the previous release:

- Revert removal of array size hacks [#4150](https://github.com/rust-lang/libc/pull/4150)
- Ensure `const extern` functions are always enabled [#4151](https://github.com/rust-lang/libc/pull/4151)

## [0.2.165](https://github.com/rust-lang/libc/compare/0.2.164...0.2.165) - 2024-11-25

### Added

- Android: add `mkostemp`, `mkostemps` <https://github.com/rust-lang/libc/pull/3601>
- Android: add a few API 30 calls <https://github.com/rust-lang/libc/pull/3604>
- Android: add missing syscall constants <https://github.com/rust-lang/libc/pull/3558>
- Apple: add `in6_ifreq` <https://github.com/rust-lang/libc/pull/3617>
- Apple: add missing `sysctl` net types <https://github.com/rust-lang/libc/pull/4022> (before release: remove `if_family_id` ([#4137](https://github.com/rust-lang/libc/pulls/4137)))
- Freebsd: add `kcmp` call support <https://github.com/rust-lang/libc/pull/3746>
- Hurd: add `MAP_32BIT` and `MAP_EXCL` <https://github.com/rust-lang/libc/pull/4127>
- Hurd: add `domainname` field to `utsname` ([#4089](https://github.com/rust-lang/libc/pulls/4089))
- Linux GNU: add `f_flags` to struct `statfs` for arm, mips, powerpc and x86 <https://github.com/rust-lang/libc/pull/3663>
- Linux GNU: add `malloc_stats` <https://github.com/rust-lang/libc/pull/3596>
- Linux: add ELF relocation-related structs <https://github.com/rust-lang/libc/pull/3583>
- Linux: add `ptp_*` structs <https://github.com/rust-lang/libc/pull/4113>
- Linux: add `ptp_clock_caps` <https://github.com/rust-lang/libc/pull/4128>
- Linux: add `ptp_pin_function` and most `PTP_` constants <https://github.com/rust-lang/libc/pull/4114>
- Linux: add missing AF_XDP structs & constants <https://github.com/rust-lang/libc/pull/3956>
- Linux: add missing netfilter consts ([#3734](https://github.com/rust-lang/libc/pulls/3734))
- Linux: add struct and constants for the `mount_setattr` syscall <https://github.com/rust-lang/libc/pull/4046>
- Linux: add wireless API <https://github.com/rust-lang/libc/pull/3441>
- Linux: expose the `len8_dlc` field of `can_frame` <https://github.com/rust-lang/libc/pull/3357>
- Musl: add `utmpx` API <https://github.com/rust-lang/libc/pull/3213>
- Musl: add missing syscall constants <https://github.com/rust-lang/libc/pull/4028>
- NetBSD: add `mcontext`-related data for RISCV64 <https://github.com/rust-lang/libc/pull/3468>
- Redox: add new `netinet` constants <https://github.com/rust-lang/libc/pull/3586>)
- Solarish: add `_POSIX_VDISABLE` ([#4103](https://github.com/rust-lang/libc/pulls/4103))
- Tests: Add a test that the `const extern fn` macro works <https://github.com/rust-lang/libc/pull/4134>
- Tests: Add test of primitive types against `std` <https://github.com/rust-lang/libc/pull/3616>
- Unix: Add `htonl`, `htons`, `ntohl`, `ntohs` <https://github.com/rust-lang/libc/pull/3669>
- Unix: add `aligned_alloc` <https://github.com/rust-lang/libc/pull/3843>
- Windows: add `aligned_realloc` <https://github.com/rust-lang/libc/pull/3592>

### Fixed

- **breaking** Hurd: fix `MAP_HASSEMAPHORE` name ([#4127](https://github.com/rust-lang/libc/pulls/4127))
- **breaking** ulibc Mips: fix `SA_*` mismatched types ([#3211](https://github.com/rust-lang/libc/pulls/3211))
- Aix: fix an enum FFI safety warning <https://github.com/rust-lang/libc/pull/3644>
- Haiku: fix some typos ([#3664](https://github.com/rust-lang/libc/pulls/3664))
- Tests: fix `Elf{32,64}_Relr`-related tests <https://github.com/rust-lang/libc/pull/3647>
- Tests: fix libc-tests for `loongarch64-linux-musl`
- Tests: fix some clippy warnings <https://github.com/rust-lang/libc/pull/3855>
- Tests: fix tests on `riscv64gc-unknown-freebsd` <https://github.com/rust-lang/libc/pull/4129>

### Deprecated

- Apple: deprecate `iconv_open` <https://github.com/rust-lang/libc/commit/25e022a22eca3634166ef472b748c297e60fcf7f>
- Apple: deprecate `mach_task_self` <https://github.com/rust-lang/libc/pull/4095>
- Apple: update `mach` deprecation notices for things that were removed in `main` <https://github.com/rust-lang/libc/pull/4097>

### Cleanup

- Adjust the `f!` macro to be more flexible <https://github.com/rust-lang/libc/pull/4107>
- Aix: remove duplicate constants <https://github.com/rust-lang/libc/pull/3643>
- CI: make scripts more uniform <https://github.com/rust-lang/libc/pull/4042>
- Drop the `libc_align` conditional <https://github.com/rust-lang/libc/commit/b5b553d0ee7de0d4781432a9a9a0a6445dd7f34f>
- Drop the `libc_cfg_target_vendor` conditional <https://github.com/rust-lang/libc/pull/4060>
- Drop the `libc_const_size_of` conditional <https://github.com/rust-lang/libc/commit/5a43dd2754366f99b3a83881b30246ce0e51833c>
- Drop the `libc_core_cvoid` conditional <https://github.com/rust-lang/libc/pull/4060>
- Drop the `libc_int128` conditional <https://github.com/rust-lang/libc/pull/4060>
- Drop the `libc_non_exhaustive` conditional <https://github.com/rust-lang/libc/pull/4060>
- Drop the `libc_packedN` conditional <https://github.com/rust-lang/libc/pull/4060>
- Drop the `libc_priv_mod_use` conditional <https://github.com/rust-lang/libc/commit/19c59376d11b015009fb9b04f233a30a1bf50a91>
- Drop the `libc_union` conditional <https://github.com/rust-lang/libc/commit/b9e4d8012f612dfe24147da3e69522763f92b6e3>
- Drop the `long_array` conditional <https://github.com/rust-lang/libc/pull/4096>
- Drop the `ptr_addr_of` conditional <https://github.com/rust-lang/libc/pull/4065>
- Drop warnings about deprecated cargo features <https://github.com/rust-lang/libc/pull/4060>
- Eliminate uses of `struct_formatter` <https://github.com/rust-lang/libc/pull/4074>
- Fix a few other array size hacks <https://github.com/rust-lang/libc/commit/d63be8b69b0736753213f5d933767866a5801ee7>
- Glibc: remove redundant definitions ([#3261](https://github.com/rust-lang/libc/pulls/3261))
- Musl: remove redundant definitions ([#3261](https://github.com/rust-lang/libc/pulls/3261))
- Musl: unify definitions of `siginfo_t` ([#3261](https://github.com/rust-lang/libc/pulls/3261))
- Musl: unify definitions of statfs and statfs64 ([#3261](https://github.com/rust-lang/libc/pulls/3261))
- Musl: unify definitions of statvfs and statvfs64 ([#3261](https://github.com/rust-lang/libc/pulls/3261))
- Musl: unify statx definitions ([#3978](https://github.com/rust-lang/libc/pulls/3978))
- Remove array size hacks for Rust < 1.47 <https://github.com/rust-lang/libc/commit/27ee6fe02ca0848b2af3cd747536264e4c7b697d>
- Remove repetitive words <https://github.com/rust-lang/libc/commit/77de375891285e18a81616f7dceda6d52732eed6>
- Use #[derive] for Copy/Clone in s! and friends <https://github.com/rust-lang/libc/pull/4038>
- Use some tricks to format macro bodies <https://github.com/rust-lang/libc/pull/4107>

### Other

- Apply formatting to macro bodies <https://github.com/rust-lang/libc/pull/4107>
- Bump libc-test to Rust 2021 Edition <https://github.com/rust-lang/libc/pull/3905>
- CI: Add a check that semver files don't contain duplicate entries <https://github.com/rust-lang/libc/pull/4087>
- CI: Add `fanotify_event_info_fid` to FAM-exempt types <https://github.com/rust-lang/libc/pull/4038>
- CI: Allow rustfmt to organize imports ([#4136](https://github.com/rust-lang/libc/pulls/4136))
- CI: Always run rustfmt <https://github.com/rust-lang/libc/pull/4120>
- CI: Change 32-bit Docker images to use EOL repos <https://github.com/rust-lang/libc/pull/4120>
- CI: Change 64-bit Docker images to ubuntu:24.10 <https://github.com/rust-lang/libc/pull/4120>
- CI: Disable the check for >1 s! invocation <https://github.com/rust-lang/libc/pull/4107>
- CI: Ensure build channels get run even if FILTER is unset <https://github.com/rust-lang/libc/pull/4125>
- CI: Ensure there is a fallback for no_std <https://github.com/rust-lang/libc/pull/4125>
- CI: Fix cases where unset variables cause errors <https://github.com/rust-lang/libc/pull/4108>
- CI: Naming adjustments and cleanup <https://github.com/rust-lang/libc/pull/4124>
- CI: Only invoke rustup if running in CI <https://github.com/rust-lang/libc/pull/4107>
- CI: Remove the logic to handle old rust versions <https://github.com/rust-lang/libc/pull/4068>
- CI: Set -u (error on unset) in all script files <https://github.com/rust-lang/libc/pull/4108>
- CI: add support for `loongarch64-unknown-linux-musl` <https://github.com/rust-lang/libc/pull/4092>
- CI: make `aarch64-apple-darwin` not a nightly-only target <https://github.com/rust-lang/libc/pull/4068>
- CI: run shellcheck on all scripts <https://github.com/rust-lang/libc/pull/4042>
- CI: update musl headers to Linux 6.6 <https://github.com/rust-lang/libc/pull/3921>
- CI: use qemu-sparc64 to run sparc64 tests <https://github.com/rust-lang/libc/pull/4133>
- Drop the `libc_const_extern_fn` conditional <https://github.com/rust-lang/libc/commit/674cc1f47f605038ef1aa2cce8e8bc9dac128276>
- Drop the `libc_underscore_const_names` conditional <https://github.com/rust-lang/libc/commit/f0febd5e2e50b38e05259d3afad3c9783711bcf0>
- Explicitly set the edition to 2015 <https://github.com/rust-lang/libc/pull/4058>
- Introduce a `git-blame-ignore-revs` file <https://github.com/rust-lang/libc/pull/4107>
- Tests: Ignore fields as required on Ubuntu 24.10 <https://github.com/rust-lang/libc/pull/4120>
- Tests: skip `ATF_*` constants for OpenBSD <https://github.com/rust-lang/libc/pull/4088>
- Triagebot: Add an autolabel for CI <https://github.com/rust-lang/libc/pull/4052>

## [0.2.164](https://github.com/rust-lang/libc/compare/0.2.163...0.2.164) - 2024-11-16

### MSRV

This release increases the MSRV of `libc` to 1.63.

### Other

- CI: remove tests with rust < 1.63 <https://github.com/rust-lang/libc/pull/4051>
- MSRV: document the MSRV of the stable channel to be 1.63 <https://github.com/rust-lang/libc/pull/4040>
- MacOS: move ifconf to s_no_extra_traits <https://github.com/rust-lang/libc/pull/4051>

## [0.2.163](https://github.com/rust-lang/libc/compare/0.2.162...0.2.163) - 2024-11-16

### Added

- Aix: add more `dlopen` flags <https://github.com/rust-lang/libc/pull/4044>
- Android: add group calls <https://github.com/rust-lang/libc/pull/3499>
- FreeBSD: add `TCP_FUNCTION_BLK` and `TCP_FUNCTION_ALIAS` <https://github.com/rust-lang/libc/pull/4047>
- Linux: add `confstr` <https://github.com/rust-lang/libc/pull/3612>
- Solarish: add `aio` <https://github.com/rust-lang/libc/pull/4033>
- Solarish: add `arc4random*` <https://github.com/rust-lang/libc/pull/3944>

### Changed

- Emscripten: upgrade emsdk to 3.1.68 <https://github.com/rust-lang/libc/pull/3962>
- Hurd: use more standard types <https://github.com/rust-lang/libc/pull/3733>
- Hurd: use the standard `ssize_t = isize` <https://github.com/rust-lang/libc/pull/4029>
- Solaris: fix `confstr` and `ucontext_t` <https://github.com/rust-lang/libc/pull/4035>

### Other

- CI: add Solaris <https://github.com/rust-lang/libc/pull/4035>
- CI: add `i686-unknown-freebsd` <https://github.com/rust-lang/libc/pull/3997>
- CI: ensure that calls to `sort` do not depend on locale <https://github.com/rust-lang/libc/pull/4026>
- Specify `rust-version` in `Cargo.toml` <https://github.com/rust-lang/libc/pull/4041>

## [0.2.162](https://github.com/rust-lang/libc/compare/0.2.161...0.2.162) - 2024-11-07

### Added

- Android: fix the alignment of `uc_mcontext` on arm64 <https://github.com/rust-lang/libc/pull/3894>
- Apple: add `host_cpu_load_info` <https://github.com/rust-lang/libc/pull/3916>
- ESP-IDF: add a time flag <https://github.com/rust-lang/libc/pull/3993>
- FreeBSD: add the `CLOSE_RANGE_CLOEXEC` flag<https://github.com/rust-lang/libc/pull/3996>
- FreeBSD: fix test errors regarding `__gregset_t` <https://github.com/rust-lang/libc/pull/3995>
- FreeBSD: fix tests on x86 FreeBSD 15 <https://github.com/rust-lang/libc/pull/3948>
- FreeBSD: make `ucontext_t` and `mcontext_t` available on all architectures  <https://github.com/rust-lang/libc/pull/3848>
- Haiku: add `getentropy` <https://github.com/rust-lang/libc/pull/3991>
- Illumos: add `syncfs` <https://github.com/rust-lang/libc/pull/3990>
- Illumos: add some recently-added constants <https://github.com/rust-lang/libc/pull/3999>
- Linux: add `ioctl` flags <https://github.com/rust-lang/libc/pull/3960>
- Linux: add epoll busy polling parameters <https://github.com/rust-lang/libc/pull/3922>
- NuttX: add `pthread_[get/set]name_np` <https://github.com/rust-lang/libc/pull/4003>
- RTEMS: add `arc4random_buf` <https://github.com/rust-lang/libc/pull/3989>
- Trusty OS: add initial support <https://github.com/rust-lang/libc/pull/3942>
- WASIp2: expand socket support <https://github.com/rust-lang/libc/pull/3981>

### Fixed

- Emscripten: don't pass `-lc` <https://github.com/rust-lang/libc/pull/4002>
- Hurd: change `st_fsid` field to `st_dev` <https://github.com/rust-lang/libc/pull/3785>
- Hurd: fix the definition of `utsname` <https://github.com/rust-lang/libc/pull/3992>
- Illumos/Solaris: fix `FNM_CASEFOLD` definition <https://github.com/rust-lang/libc/pull/4004>
- Solaris: fix all tests <https://github.com/rust-lang/libc/pull/3864>

### Other

- CI: Add loongarch64 <https://github.com/rust-lang/libc/pull/4000>
- CI: Check that semver files are sorted <https://github.com/rust-lang/libc/pull/4018>
- CI: Re-enable the FreeBSD 15 job <https://github.com/rust-lang/libc/pull/3988>
- Clean up imports and `extern crate` usage <https://github.com/rust-lang/libc/pull/3897>
- Convert `mode_t` constants to octal <https://github.com/rust-lang/libc/pull/3634>
- Remove the `wasm32-wasi` target that has been deleted upstream <https://github.com/rust-lang/libc/pull/4013>

## [0.2.161](https://github.com/rust-lang/libc/compare/0.2.160...0.2.161) - 2024-10-17

### Fixed

- OpenBSD: fix `FNM_PATHNAME` and `FNM_NOESCAPE` values <https://github.com/rust-lang/libc/pull/3983>

## [0.2.160](https://github.com/rust-lang/libc/compare/0.2.159...0.2.160) - 2024-10-17

### Added

- Android: add `PR_GET_NAME` and `PR_SET_NAME` <https://github.com/rust-lang/libc/pull/3941>
- Apple: add `F_TRANSFEREXTENTS` <https://github.com/rust-lang/libc/pull/3925>
- Apple: add `mach_error_string` <https://github.com/rust-lang/libc/pull/3913>
- Apple: add additional `pthread` APIs <https://github.com/rust-lang/libc/pull/3846>
- Apple: add the `LOCAL_PEERTOKEN` socket option <https://github.com/rust-lang/libc/pull/3929>
- BSD: add `RTF_*`, `RTA_*`, `RTAX_*`, and `RTM_*` definitions <https://github.com/rust-lang/libc/pull/3714>
- Emscripten: add `AT_EACCESS` <https://github.com/rust-lang/libc/pull/3911>
- Emscripten: add `getgrgid`, `getgrnam`, `getgrnam_r` and `getgrgid_r` <https://github.com/rust-lang/libc/pull/3912>
- Emscripten: add `getpwnam_r` and `getpwuid_r` <https://github.com/rust-lang/libc/pull/3906>
- FreeBSD: add `POLLRDHUP` <https://github.com/rust-lang/libc/pull/3936>
- Haiku: add `arc4random` <https://github.com/rust-lang/libc/pull/3945>
- Illumos: add `ptsname_r` <https://github.com/rust-lang/libc/pull/3867>
- Linux: add `fanotify` interfaces <https://github.com/rust-lang/libc/pull/3695>
- Linux: add `tcp_info` <https://github.com/rust-lang/libc/pull/3480>
- Linux: add additional AF_PACKET options <https://github.com/rust-lang/libc/pull/3540>
- Linux: make Elf constants always available <https://github.com/rust-lang/libc/pull/3938>
- Musl x86: add `iopl` and `ioperm` <https://github.com/rust-lang/libc/pull/3720>
- Musl: add `posix_spawn` chdir functions <https://github.com/rust-lang/libc/pull/3949>
- Musl: add `utmpx.h` constants <https://github.com/rust-lang/libc/pull/3908>
- NetBSD: add `sysctlnametomib`, `CLOCK_THREAD_CPUTIME_ID` and `CLOCK_PROCESS_CPUTIME_ID` <https://github.com/rust-lang/libc/pull/3927>
- Nuttx: initial support <https://github.com/rust-lang/libc/pull/3909>
- RTEMS: add `getentropy` <https://github.com/rust-lang/libc/pull/3973>
- RTEMS: initial support <https://github.com/rust-lang/libc/pull/3866>
- Solarish: add `POLLRDHUP`, `POSIX_FADV_*`, `O_RSYNC`, and `posix_fallocate` <https://github.com/rust-lang/libc/pull/3936>
- Unix: add `fnmatch.h` <https://github.com/rust-lang/libc/pull/3937>
- VxWorks: add riscv64 support <https://github.com/rust-lang/libc/pull/3935>
- VxWorks: update constants related to the scheduler  <https://github.com/rust-lang/libc/pull/3963>

### Changed

- Redox: change `ino_t` to be `c_ulonglong` <https://github.com/rust-lang/libc/pull/3919>

### Fixed

- ESP-IDF: fix mismatched constants and structs <https://github.com/rust-lang/libc/pull/3920>
- FreeBSD: fix `struct stat` on FreeBSD 12+ <https://github.com/rust-lang/libc/pull/3946>

### Other

- CI: Fix CI for FreeBSD 15 <https://github.com/rust-lang/libc/pull/3950>
- Docs: link to `windows-sys` <https://github.com/rust-lang/libc/pull/3915>

## [0.2.159](https://github.com/rust-lang/libc/compare/0.2.158...0.2.159) - 2024-09-24

### Added

- Android: add more `AT_*` constants in <https://github.com/rust-lang/libc/pull/3779>
- Apple: add missing `NOTE_*` constants in <https://github.com/rust-lang/libc/pull/3883>
- Hermit: add missing error numbers in <https://github.com/rust-lang/libc/pull/3858>
- Hurd: add `__timeval` for 64-bit support in <https://github.com/rust-lang/libc/pull/3786>
- Linux: add `epoll_pwait2` in <https://github.com/rust-lang/libc/pull/3868>
- Linux: add `mq_notify` in <https://github.com/rust-lang/libc/pull/3849>
- Linux: add missing `NFT_CT_*` constants in <https://github.com/rust-lang/libc/pull/3844>
- Linux: add the `fchmodat2` syscall in <https://github.com/rust-lang/libc/pull/3588>
- Linux: add the `mseal` syscall in <https://github.com/rust-lang/libc/pull/3798>
- OpenBSD: add `sendmmsg` and `recvmmsg` in <https://github.com/rust-lang/libc/pull/3831>
- Unix: add `IN6ADDR_ANY_INIT` and `IN6ADDR_LOOPBACK_INIT` in <https://github.com/rust-lang/libc/pull/3693>
- VxWorks: add `S_ISVTX` in <https://github.com/rust-lang/libc/pull/3768>
- VxWorks: add `vxCpuLib` and `taskLib` functions <https://github.com/rust-lang/libc/pull/3861>
- WASIp2: add definitions for `std::net` support in <https://github.com/rust-lang/libc/pull/3892>

### Fixed

- Correctly handle version checks when `clippy-driver` is used <https://github.com/rust-lang/libc/pull/3893>

### Changed

- EspIdf: change signal constants to c_int in <https://github.com/rust-lang/libc/pull/3895>
- HorizonOS: update network definitions in <https://github.com/rust-lang/libc/pull/3863>
- Linux: combine `ioctl` APIs in <https://github.com/rust-lang/libc/pull/3722>
- WASI: enable CI testing in <https://github.com/rust-lang/libc/pull/3869>
- WASIp2: enable CI testing in <https://github.com/rust-lang/libc/pull/3870>

## [0.2.158](https://github.com/rust-lang/libc/compare/0.2.157...0.2.158) - 2024-08-19

### Other
- WASI: fix missing `Iterator` with `rustc-dep-of-std` in <https://github.com/rust-lang/libc/pull/3856#event-13924913068>

## [0.2.157](https://github.com/rust-lang/libc/compare/0.2.156...0.2.157) - 2024-08-17

### Added

- Apple: add `_NSGetArgv`, `_NSGetArgc` and `_NSGetProgname` in <https://github.com/rust-lang/libc/pull/3702>
- Build: add `RUSTC_WRAPPER` support in <https://github.com/rust-lang/libc/pull/3845>
- FreeBSD: add `execvpe` support from 14.1 release in <https://github.com/rust-lang/libc/pull/3745>
- Fuchsia: add `SO_BINDTOIFINDEX`
- Linux: add `klogctl` in <https://github.com/rust-lang/libc/pull/3777>
- MacOS: add `fcntl` OFD commands in <https://github.com/rust-lang/libc/pull/3563>
- NetBSD: add `_lwp_park` in <https://github.com/rust-lang/libc/pull/3721>
- Solaris: add missing networking support in <https://github.com/rust-lang/libc/pull/3717>
- Unix: add `pthread_equal` in <https://github.com/rust-lang/libc/pull/3773>
- WASI: add `select`, `FD_SET`, `FD_ZERO`, `FD_ISSET ` in <https://github.com/rust-lang/libc/pull/3681>

### Fixed
- TEEOS: fix octal notation for `O_*` constants in <https://github.com/rust-lang/libc/pull/3841>

### Changed
- FreeBSD: always use freebsd12 when `rustc_dep_of_std` is set in <https://github.com/rust-lang/libc/pull/3723>

## [0.2.156](https://github.com/rust-lang/libc/compare/v0.2.155...v0.2.156) - 2024-08-15

### Added
- Apple: add `F_ALLOCATEPERSIST` in <https://github.com/rust-lang/libc/pull/3712>
- Apple: add `os_sync_wait_on_address` and related definitions in <https://github.com/rust-lang/libc/pull/3769>
- BSD: generalise `IPV6_DONTFRAG` to all BSD targets in <https://github.com/rust-lang/libc/pull/3716>
- FreeBSD/DragonFly: add `IP_RECVTTL`/`IPV6_RECVHOPLIMIT` in <https://github.com/rust-lang/libc/pull/3751>
- Hurd: add `XATTR_CREATE`, `XATTR_REPLACE` in <https://github.com/rust-lang/libc/pull/3739>
- Linux GNU: `confstr` API and `_CS_*` in <https://github.com/rust-lang/libc/pull/3771>
- Linux musl: add `preadv2` and `pwritev2` (1.2.5 min.) in <https://github.com/rust-lang/libc/pull/3762>
- VxWorks: add the constant `SOMAXCONN` in <https://github.com/rust-lang/libc/pull/3761>
- VxWorks: add a few errnoLib related constants in <https://github.com/rust-lang/libc/pull/3780>

### Fixed
- Solaris/illumos: Change `ifa_flags` type to u64 in <https://github.com/rust-lang/libc/pull/3729>
- QNX 7.0: Disable `libregex` in <https://github.com/rust-lang/libc/pull/3775>

### Changed
- QNX NTO: update platform support in <https://github.com/rust-lang/libc/pull/3815>
- `addr_of!(EXTERN_STATIC)` is now considered safe in <https://github.com/rust-lang/libc/pull/3776>

### Removed
- Apple: remove `rmx_state` in <https://github.com/rust-lang/libc/pull/3776>

### Other
- Update or remove CI tests that have been failing
