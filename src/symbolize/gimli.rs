//! Support for symbolication using the `gimli` crate on crates.io
//!
//! This implementation is largely a work in progress and is off by default for
//! all platforms, but it's hoped to be developed over time! Long-term this is
//! intended to wholesale replace the `libbacktrace.rs` implementation.

use self::gimli::read::EndianSlice;
use self::gimli::LittleEndian as Endian;
use crate::symbolize::dladdr;
use crate::symbolize::ResolveWhat;
use crate::types::BytesOrWideString;
use crate::SymbolName;
use addr2line::gimli;
use core::cell::RefCell;
use core::convert::TryFrom;
use core::mem;
use core::u32;
use findshlibs::{self, Segment, SharedLibrary};
use libc::c_void;
use memmap::Mmap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::prelude::v1::*;

const MAPPINGS_CACHE_SIZE: usize = 4;

type Context<'a> = addr2line::Context<EndianSlice<'a, Endian>>;

struct Mapping {
    // 'static lifetime is a lie to hack around lack of support for self-referential structs.
    cx: Context<'static>,
    _map: Mmap,
}

fn cx<'data>(load: impl Fn(&str) -> &'data [u8]) -> Option<Context<'data>> {
    fn load_section<'data, S>(load: impl Fn(&str) -> &'data [u8]) -> S
    where
        S: gimli::Section<gimli::EndianSlice<'data, Endian>>,
    {
        S::from(EndianSlice::new(load(S::section_name()), Endian))
    }

    let debug_abbrev: gimli::DebugAbbrev<_> = load_section(&load);
    let debug_addr: gimli::DebugAddr<_> = load_section(&load);
    let debug_info: gimli::DebugInfo<_> = load_section(&load);
    let debug_line: gimli::DebugLine<_> = load_section(&load);
    let debug_line_str: gimli::DebugLineStr<_> = load_section(&load);
    let debug_ranges: gimli::DebugRanges<_> = load_section(&load);
    let debug_rnglists: gimli::DebugRngLists<_> = load_section(&load);
    let debug_str: gimli::DebugStr<_> = load_section(&load);
    let debug_str_offsets: gimli::DebugStrOffsets<_> = load_section(&load);
    let default_section = gimli::EndianSlice::new(&[], Endian);

    addr2line::Context::from_sections(
        debug_abbrev,
        debug_addr,
        debug_info,
        debug_line,
        debug_line_str,
        debug_ranges,
        debug_rnglists,
        debug_str,
        debug_str_offsets,
        default_section,
    )
    .ok()
}

fn assert_lifetimes<'a>(_: &'a Mmap, _: &Context<'a>) {}

macro_rules! mk {
    (Mapping { $map:expr, $inner:expr }) => {{
        assert_lifetimes(&$map, &$inner);
        Mapping {
            // Convert to 'static lifetimes since the symbols should
            // only borrow `map` and we're preserving `map` below.
            cx: unsafe { mem::transmute::<Context<'_>, Context<'static>>($inner) },
            _map: $map,
        }
    }};
}

fn mmap(path: &Path) -> Option<Mmap> {
    let file = File::open(path).ok()?;
    // TODO: not completely safe, see https://github.com/danburkert/memmap-rs/issues/25
    unsafe { Mmap::map(&file).ok() }
}

impl Mapping {
    #[cfg(not(any(target_os = "macos", windows)))]
    fn new(path: &Path) -> Option<Mapping> {
        use goblin::elf::*;

        let map = mmap(path)?;
        let object = Elf::parse(&map).ok()?;
        if !object.little_endian {
            return None;
        }
        let cx = cx(|name| {
            let section = object.section_headers.iter().find(|section| {
                match object.shdr_strtab.get(section.sh_name) {
                    Some(Ok(section_name)) => section_name == name,
                    _ => false,
                }
            });
            section
                .and_then(|section| {
                    map.get(section.sh_offset as usize..)
                        .and_then(|data| data.get(..section.sh_size as usize))
                })
                .unwrap_or(&[])
        })?;
        Some(mk!(Mapping { map, cx }))
    }

    #[cfg(windows)]
    fn new(path: &Path) -> Option<Mapping> {
        use core::cmp;
        use goblin::pe::*;

        let map = mmap(path)?;
        let object = PE::parse(&map).ok()?;
        let cx = cx(|name| {
            let section = object
                .sections
                .iter()
                .find(|section| section.name().ok() == Some(name));
            section
                .and_then(|section| {
                    let offset = section.pointer_to_raw_data as usize;
                    let size = cmp::min(section.virtual_size, section.size_of_raw_data) as usize;
                    map.get(offset..).and_then(|data| data.get(..size))
                })
                .unwrap_or(&[])
        })?;
        Some(mk!(Mapping { map, cx }))
    }

    #[cfg(target_os = "macos")]
    fn new(path: &Path) -> Option<Mapping> {
        use goblin::mach::*;

        // First up we need to load the unique UUID which is stored in the macho
        // header of the file we're reading, specified at `path`.
        let map = mmap(path)?;
        let object = MachO::parse(&map, 0).ok()?;
        let uuid = find_uuid(&object)?;

        // Next we need to look for a `*.dSYM` file. For now we just probe the
        // containing directory and look around for something that matches
        // `*.dSYM`. Once it's found we root through the dwarf resources that it
        // contains and try to find a macho file which has a matching UUID as
        // the one of our own file. If we find a match that's the dwarf file we
        // want to return.
        let parent = path.parent()?;
        for entry in parent.read_dir().ok()? {
            let entry = entry.ok()?;
            let filename = match entry.file_name().into_string() {
                Ok(name) => name,
                Err(_) => continue,
            };
            if !filename.ends_with(".dSYM") {
                continue;
            }
            let candidates = entry.path().join("Contents/Resources/DWARF");
            if let Some(mapping) = load_dsym(&candidates, &uuid) {
                return Some(mapping);
            }
        }

        // Looks like nothing matched our UUID, so let's at least return our own
        // file. This should have the symbol table for at least some
        // symbolication purposes.
        return match macho2inner(&object) {
            Some(inner) => Some(mk!(Mapping { map, inner })),
            None => None,
        };

        fn load_dsym(dir: &Path, uuid: &[u8; 16]) -> Option<Mapping> {
            for entry in dir.read_dir().ok()? {
                let entry = entry.ok()?;
                let map = mmap(&entry.path())?;
                let object = MachO::parse(&map, 0).ok()?;
                let entry_uuid = find_uuid(&object)?;
                if entry_uuid != uuid {
                    continue;
                }
                if let Some(inner) = macho2inner(&object) {
                    return Some(mk!(Mapping { map, inner }));
                }
            }

            None
        }

        fn macho2inner<'a>(object: &MachO<'a>) -> Option<Context<'a>> {
            if !object.little_endian {
                return None;
            }
            let dwarf = object
                .segments
                .iter()
                .find(|segment| segment.name().ok() == Some("__DWARF"))?;
            cx(|name| {
                dwarf
                    .into_iter()
                    .filter_map(|s| s.ok())
                    .find(|(section, _data)| {
                        let section_name = match section.name() {
                            Ok(s) => s,
                            Err(_) => return false,
                        };
                        &section_name[..] == name || {
                            section_name.starts_with("__")
                                && name.starts_with(".")
                                && &section_name[2..] == &name[1..]
                        }
                    })
                    .map(|p| p.1)
                    .unwrap_or(&[])
            })
        }

        fn find_uuid<'a>(object: &'a MachO) -> Option<&'a [u8; 16]> {
            object
                .load_commands
                .iter()
                .filter_map(|cmd| match &cmd.command {
                    load_command::CommandVariant::Uuid(u) => Some(&u.uuid),
                    _ => None,
                })
                .next()
        }
    }

    // Ensure the 'static lifetimes don't leak.
    fn rent<F>(&self, mut f: F)
    where
        F: FnMut(&addr2line::Context<EndianSlice<'_, Endian>>),
    {
        f(&self.cx)
    }
}

thread_local! {
    // A very small, very simple LRU cache for debug info mappings.
    //
    // The hit rate should be very high, since the typical stack doesn't cross
    // between many shared libraries.
    //
    // The `addr2line::Context` structures are pretty expensive to create. Its
    // cost is expected to be amortized by subsequent `locate` queries, which
    // leverage the structures built when constructing `addr2line::Context`s to
    // get nice speedups. If we didn't have this cache, that amortization would
    // never happen, and symbolicating backtraces would be ssssllllooooowwww.
    static MAPPINGS_CACHE: RefCell<Vec<(PathBuf, Mapping)>>
        = RefCell::new(Vec::with_capacity(MAPPINGS_CACHE_SIZE));
}

fn with_mapping_for_path<F>(path: PathBuf, f: F)
where
    F: FnMut(&addr2line::Context<EndianSlice<'_, Endian>>),
{
    MAPPINGS_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();

        let idx = cache.iter().position(|&(ref p, _)| p == &path);

        // Invariant: after this conditional completes without early returning
        // from an error, the cache entry for this path is at index 0.

        if let Some(idx) = idx {
            // When the mapping is already in the cache, move it to the front.
            if idx != 0 {
                let entry = cache.remove(idx);
                cache.insert(0, entry);
            }
        } else {
            // When the mapping is not in the cache, create a new mapping,
            // insert it into the front of the cache, and evict the oldest cache
            // entry if necessary.
            let mapping = match Mapping::new(&path) {
                None => return,
                Some(m) => m,
            };

            if cache.len() == MAPPINGS_CACHE_SIZE {
                cache.pop();
            }

            cache.insert(0, (path, mapping));
        }

        cache[0].1.rent(f);
    });
}

pub unsafe fn resolve(what: ResolveWhat, cb: &mut FnMut(&super::Symbol)) {
    let addr = what.address_or_ip();
    let mut cb = DladdrFallback {
        cb,
        addr,
        called: false,
    };

    // First, find the file containing the segment that the given AVMA (after
    // relocation) address falls within. Use the containing segment to compute
    // the SVMA (before relocation) address.
    //
    // Note that the OS APIs that `SharedLibrary::each` is implemented with hold
    // a lock for the duration of the `each` call, so we want to keep this
    // section as short as possible to avoid contention with other threads
    // capturing backtraces.
    //
    // Also note that for now `findshlibs` is unimplemented on Windows, so we
    // just unhelpfully assume that the address is an SVMA. Surprisingly it
    // seems to at least somewhat work on Wine on Linux though...
    let (addr, path) = if cfg!(windows) {
        let addr = findshlibs::Svma(addr as *mut u8 as *const u8);
        (addr, String::new())
    } else {
        let addr = findshlibs::Avma(addr as *mut u8 as *const u8);
        let mut so_info = None;
        findshlibs::TargetSharedLibrary::each(|so| {
            use findshlibs::IterationControl::*;

            for segment in so.segments() {
                if segment.contains_avma(so, addr) {
                    let addr = so.avma_to_svma(addr);
                    let path = so.name().to_string_lossy();
                    so_info = Some((addr, path.to_string()));
                    return Break;
                }
            }

            Continue
        });
        match so_info {
            None => return,
            Some((a, p)) => (a, p),
        }
    };

    // Second, fixup the path. Empty path means that this address falls within
    // the main executable, not a shared library.
    let path = if path.is_empty() {
        match env::current_exe() {
            Err(_) => return,
            Ok(p) => p,
        }
    } else {
        PathBuf::from(path)
    };

    // Finally, get a cached mapping or create a new mapping for this file, and
    // evaluate the DWARF info to find the file/line/name for this address.
    with_mapping_for_path(path, |dwarf| {
        let mut frames = match dwarf.find_frames(addr.0 as u64) {
            Ok(frames) => frames,
            Err(_) => return,
        };
        while let Ok(Some(mut frame)) = frames.next() {
            let function = frame.function.take();
            let name = function.as_ref().and_then(|f| f.raw_name().ok());
            let name = name.as_ref().map(|n| n.as_bytes());
            cb.call(Symbol::Frame {
                addr: addr.0 as *mut c_void,
                frame,
                name,
            });
        }
    });

    drop(cb);
}

struct DladdrFallback<'a, 'b> {
    addr: *mut c_void,
    called: bool,
    cb: &'a mut (FnMut(&super::Symbol) + 'b),
}

impl DladdrFallback<'_, '_> {
    fn call(&mut self, sym: Symbol) {
        self.called = true;

        // Extend the lifetime of `sym` to `'static` since we are unfortunately
        // required to here, but it's ony ever going out as a reference so no
        // reference to it should be persisted beyond this frame anyway.
        let sym = unsafe { mem::transmute::<Symbol, Symbol<'static>>(sym) };
        (self.cb)(&super::Symbol { inner: sym });
    }
}

impl Drop for DladdrFallback<'_, '_> {
    fn drop(&mut self) {
        if self.called {
            return;
        }
        unsafe {
            dladdr::resolve(self.addr, &mut |sym| {
                (self.cb)(&super::Symbol {
                    inner: Symbol::Dladdr(sym),
                })
            });
        }
    }
}

pub enum Symbol<'a> {
    /// We were able to locate frame information for this symbol, and
    /// `addr2line`'s frame internally has all the nitty gritty details.
    Frame {
        addr: *mut c_void,
        frame: addr2line::Frame<EndianSlice<'a, Endian>>,
        name: Option<&'a [u8]>,
    },
    /// We weren't able to find anything in the original file, so we had to fall
    /// back to using `dladdr` which had a hit.
    Dladdr(dladdr::Symbol<'a>),
}

impl Symbol<'_> {
    pub fn name(&self) -> Option<SymbolName> {
        match self {
            Symbol::Dladdr(s) => s.name(),
            Symbol::Frame { name, .. } => {
                let name = name.as_ref()?;
                Some(SymbolName::new(name))
            }
        }
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        match self {
            Symbol::Dladdr(s) => s.addr(),
            Symbol::Frame { addr, .. } => Some(*addr),
        }
    }

    pub fn filename_raw(&self) -> Option<BytesOrWideString> {
        match self {
            Symbol::Dladdr(s) => return s.filename_raw(),
            Symbol::Frame { frame, .. } => {
                let location = frame.location.as_ref()?;
                let file = location.file.as_ref()?;
                Some(BytesOrWideString::Bytes(file.as_bytes()))
            }
        }
    }

    pub fn filename(&self) -> Option<&Path> {
        match self {
            Symbol::Dladdr(s) => return s.filename(),
            Symbol::Frame { frame, .. } => {
                let location = frame.location.as_ref()?;
                let file = location.file.as_ref()?;
                Some(Path::new(file))
            }
        }
    }

    pub fn lineno(&self) -> Option<u32> {
        match self {
            Symbol::Dladdr(s) => return s.lineno(),
            Symbol::Frame { frame, .. } => {
                let location = frame.location.as_ref()?;
                location.line.and_then(|l| u32::try_from(l).ok())
            }
        }
    }
}
