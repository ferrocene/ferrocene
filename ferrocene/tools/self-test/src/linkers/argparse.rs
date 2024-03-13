//! Code for parsing Linker arguments

// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

/// Linker options we know about
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LinkerArg<'a> {
    // Unqualified option
    /// An input file (an argument with no option before it)
    Input(&'a str),

    // Single character options
    /// The `-ofoo.elf` option
    Output(&'a str),
    /// The `-Lpath/foo` library path option
    LibraryPath(&'a str),
    /// The `-X` / `--discard-all` option
    DiscardAll,
    /// The `-Zfoo` option
    Keyword(&'a str),
    /// The `-lfoo` option
    Link(&'a str),
    /// The `-mfoo` option
    Emulation(&'a str),

    // Long options
    /// The `-plugin` option
    Plugin(&'a str),
    /// The `-plugin-opt=` option
    PluginOpt(&'a str),
    /// The `-EL` option
    LittleEndian,
    /// The `-pie` option
    PicExecutable,
    /// The `-no-pie` option
    NonPicExecutable,
    /// The `-dynamic-linker foo/bar` option
    DynamicLinker(&'a str),
    /// The `--sysroot=` option
    Sysroot(&'a str),
    /// The `--build-id`` option
    BuildId,
    /// The `--eh-frame-hdr`` option
    EhFrameHeader,
    /// The `--hash-style=` option
    HashStyle(&'a str),
    /// The `--as-needed` option
    AsNeeded,
    /// The `--no-as-needed` option
    NoAsNeeded,
    /// The `--push-state` option
    PushState,
    /// The `--pop-state` option
    PopState,
    /// The `--fix-cortex-a53-843419` option
    FixCortexA53_843419,

    // Anything else goes here
    /// We didn't recognise this option
    Unknown(&'a str),
}

/// Parse single letter linker arguments.
///
/// This might be "-ofoo" or "-o foo".
fn short_arg<'a, F, I>(option: char, arg: &'a str, args_iter: &mut I, f: F) -> Option<LinkerArg<'a>>
where
    I: Iterator<Item = &'a str>,
    F: FnOnce(&'a str) -> LinkerArg<'a>,
{
    let option = format!("-{option}");
    if arg == option {
        if let Some(next) = args_iter.next() {
            Some(f(next))
        } else {
            Some(LinkerArg::Unknown(arg))
        }
    } else {
        arg.strip_prefix(&option).map(f)
    }
}

/// Parse single letter linker options.
///
/// This might be "-X".
fn short_opt<'a, F>(option: char, arg: &'a str, f: F) -> Option<LinkerArg<'a>>
where
    F: FnOnce() -> LinkerArg<'a>,
{
    let option = format!("-{option}");
    (arg == option).then(f)
}

/// Parse multi-letter linker arguments.
///
/// This might be "--foo=bar" or "--foo bar" or "-foo bar" or "-foo=bar"
fn long_arg<'a, F, I>(option: &str, arg: &'a str, args_iter: &mut I, f: F) -> Option<LinkerArg<'a>>
where
    I: Iterator<Item = &'a str>,
    F: FnOnce(&'a str) -> LinkerArg<'a>,
{
    let onedash = format!("-{option}");
    let twodash = format!("--{option}");
    if arg == onedash || arg == twodash {
        if let Some(next) = args_iter.next() {
            Some(f(next))
        } else {
            Some(LinkerArg::Unknown(arg))
        }
    } else if let Some(tail) = arg.strip_prefix(&format!("{onedash}=")) {
        Some(f(tail))
    } else {
        arg.strip_prefix(&format!("{twodash}=")).map(f)
    }
}

/// Parse multi-letter linker options.
///
/// This might be "--eh-frame-hdr" or "-eh-frame-hdr"
fn long_opt<'a, F>(option: &str, arg: &'a str, f: F) -> Option<LinkerArg<'a>>
where
    F: FnOnce() -> LinkerArg<'a>,
{
    let onedash = format!("-{option}");
    let twodash = format!("--{option}");
    (arg == onedash || arg == twodash).then(f)
}

/// Clean up split linker arguments so they can be more easily processed
#[rustfmt::skip]
pub fn rationalise_linker_args<'a, I>(mut args_iter: I) -> Vec<LinkerArg<'a>>
where
    I: Iterator<Item = &'a str>,
{
    let mut output = Vec::new();
    while let Some(arg) = args_iter.next() {
        if let Some(result) = short_arg('o', arg, &mut args_iter, LinkerArg::Output) {
            output.push(result);
        } else if let Some(result) = short_arg('L', arg, &mut args_iter, LinkerArg::LibraryPath) {
            output.push(result);
        } else if let Some(result) = short_opt('X', arg, || LinkerArg::DiscardAll) {
            output.push(result);
        } else if let Some(result) = long_opt("discard-all", arg, || LinkerArg::DiscardAll) {
            output.push(result);
        } else if let Some(result) = short_arg('z', arg, &mut args_iter, LinkerArg::Keyword) {
            output.push(result);
        } else if let Some(result) = short_arg('l', arg, &mut args_iter, LinkerArg::Link) {
            output.push(result);
        } else if let Some(result) = long_arg("library-path", arg, &mut args_iter, LinkerArg::Link) {
            output.push(result);
        } else if let Some(result) = short_arg('m', arg, &mut args_iter, LinkerArg::Emulation) {
            output.push(result);
        } else if let Some(result) = long_arg("plugin", arg, &mut args_iter, LinkerArg::Plugin) {
            output.push(result);
        } else if let Some(result) = long_arg("plugin-opt", arg, &mut args_iter, LinkerArg::PluginOpt) {
            output.push(result);
        } else if let Some(result) = long_opt("EL", arg, || LinkerArg::LittleEndian) {
            output.push(result);
        } else if let Some(result) = long_opt("pie", arg, || LinkerArg::PicExecutable) {
            output.push(result);
        } else if let Some(result) = long_opt("pic-executable", arg, || LinkerArg::PicExecutable) {
            output.push(result);
        } else if let Some(result) = long_opt("no-pie", arg, || LinkerArg::NonPicExecutable) {
            output.push(result);
        } else if let Some(result) = short_arg('I', arg, &mut args_iter, LinkerArg::DynamicLinker) {
            output.push(result);
        } else if let Some(result) = long_arg("dynamic-linker", arg, &mut args_iter, LinkerArg::DynamicLinker) {
            output.push(result);
        } else if let Some(result) = long_arg("sysroot", arg, &mut args_iter, LinkerArg::Sysroot) {
            output.push(result);
        } else if let Some(result) = long_opt("build-id", arg, || LinkerArg::BuildId) {
            output.push(result);
        } else if let Some(result) = long_opt("eh-frame-hdr", arg, || LinkerArg::EhFrameHeader) {
            output.push(result);
        } else if let Some(result) = long_arg("hash-style", arg, &mut args_iter, LinkerArg::HashStyle) {
            output.push(result);
        } else if let Some(result) = long_opt("as-needed", arg, || LinkerArg::AsNeeded) {
            output.push(result);
        } else if let Some(result) = long_opt("no-as-needed", arg, || LinkerArg::NoAsNeeded) {
            output.push(result);
        } else if let Some(result) = long_opt("push-state", arg, || LinkerArg::PushState) {
            output.push(result);
        } else if let Some(result) = long_opt("pop-state", arg, || LinkerArg::PopState) {
            output.push(result);
        } else if let Some(result) = long_opt("fix-cortex-a53-843419", arg, || LinkerArg::FixCortexA53_843419) {
            output.push(result);
        } else if arg.starts_with('-') {
            output.push(LinkerArg::Unknown(arg));
        } else {
            output.push(LinkerArg::Input(arg));
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rationalise_linker_args() {
        // these are the ones we saw from gcc
        let input = include_str!("snapshots/rationalise_linker_args.input");
        let output = rationalise_linker_args(input.lines());
        insta::with_settings!({ prepend_module_to_snapshot => false }, {
            insta::assert_debug_snapshot!(output);
        });
    }

    #[test]
    fn test_rationalise_linker_extra_args() {
        // these are extra ones we added in case we see them
        let input = include_str!("snapshots/rationalise_linker_extra_args.input");
        let output = rationalise_linker_args(input.lines());
        insta::with_settings!({ prepend_module_to_snapshot => false }, {
            insta::assert_debug_snapshot!(output);
        });
    }
}
