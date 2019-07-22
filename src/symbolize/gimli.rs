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

struct Context<'a> {
    dwarf: addr2line::Context<EndianSlice<'a, Endian>>,
    object: Object<'a>,
}

struct Mapping {
    // 'static lifetime is a lie to hack around lack of support for self-referential structs.
    cx: Context<'static>,
    _map: Mmap,
}

fn cx<'data>(object: Object<'data>) -> Option<Context<'data>> {
    fn load_section<'data, S>(obj: &Object<'data>) -> S
    where
        S: gimli::Section<gimli::EndianSlice<'data, Endian>>,
    {
        let data = obj.section(S::section_name()).unwrap_or(&[]);
        S::from(EndianSlice::new(data, Endian))
    }

    let dwarf = addr2line::Context::from_sections(
        load_section(&object),
        load_section(&object),
        load_section(&object),
        load_section(&object),
        load_section(&object),
        load_section(&object),
        load_section(&object),
        load_section(&object),
        load_section(&object),
        gimli::EndianSlice::new(&[], Endian),
    )
    .ok()?;
    Some(Context { dwarf, object })
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

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        use std::cmp;
        use goblin::pe::{self, PE};
        use goblin::strtab::Strtab;

        struct Object<'a> {
            pe: PE<'a>,
            data: &'a [u8],
            symbols: Vec<(usize, pe::symbol::Symbol)>,
            strtab: Strtab<'a>,
        }

        impl<'a> Object<'a> {
            fn parse(data: &'a [u8]) -> Option<Object<'a>> {
                let pe = PE::parse(data).ok()?;
                let syms = pe.header.coff_header.symbols(data).ok()?;
                let strtab = pe.header.coff_header.strings(data).ok()?;

                // Collect all the symbols into a local vector which is sorted
                // by address and contains enough data to learn about the symbol
                // name. Note that we only look at function symbols and also
                // note that the sections are 1-indexed because the zero section
                // is special (apparently).
                let mut symbols = Vec::new();
                for (_, _, sym) in syms.iter() {
                    if sym.derived_type() != pe::symbol::IMAGE_SYM_DTYPE_FUNCTION
                        || sym.section_number == 0
                    {
                        continue;
                    }
                    let addr = usize::try_from(sym.value).ok()?;
                    let section = pe.sections.get(usize::try_from(sym.section_number).ok()? - 1)?;
                    let va = usize::try_from(section.virtual_address).ok()?;
                    symbols.push((addr + va + pe.image_base, sym));
                }
                symbols.sort_unstable_by_key(|x| x.0);
                Some(Object { pe, data, symbols, strtab })
            }

            fn section(&self, name: &str) -> Option<&'a [u8]> {
                let section = self.pe
                    .sections
                    .iter()
                    .find(|section| section.name().ok() == Some(name));
                section
                    .and_then(|section| {
                        let offset = section.pointer_to_raw_data as usize;
                        let size = cmp::min(section.virtual_size, section.size_of_raw_data) as usize;
                        self.data.get(offset..).and_then(|data| data.get(..size))
                    })
            }

            fn search_symtab<'b>(&'b self, addr: u64) -> Option<&'b [u8]> {
                // Note that unlike other formats COFF doesn't embed the size of
                // each symbol. As a last ditch effort search for the *closest*
                // symbol to a particular address and return that one. This gets
                // really wonky once symbols start getting removed because the
                // symbols returned here can be totally incorrect, but we have
                // no idea of knowing how to detect that.
                let addr = usize::try_from(addr).ok()?;
                let i = match self.symbols.binary_search_by_key(&addr, |p| p.0) {
                    Ok(i) => i,
                    // typically `addr` isn't in the array, but `i` is where
                    // we'd insert it, so the previous position must be the
                    // greatest less than `addr`
                    Err(i) => i.checked_sub(1)?,
                };
                Some(self.symbols[i].1.name(&self.strtab).ok()?.as_bytes())
            }
        }
    } else if #[cfg(target_os = "macos")] {
        use goblin::mach::MachO;

        struct Object<'a> {
            macho: MachO<'a>,
            dwarf: Option<usize>,
        }

        impl<'a> Object<'a> {
            fn parse(macho: MachO<'a>) -> Option<Object<'a>> {
                if !macho.little_endian {
                    return None;
                }
                let dwarf = macho
                    .segments
                    .iter()
                    .enumerate()
                    .find(|(_, segment)| segment.name().ok() == Some("__DWARF"))
                    .map(|p| p.0);
                Some(Object { macho, dwarf })
            }

            fn section(&self, name: &str) -> Option<&'a [u8]> {
                let dwarf = self.dwarf?;
                let dwarf = &self.macho.segments[dwarf];
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
            }

            fn search_symtab<'b>(&'b self, _addr: u64) -> Option<&'b [u8]> {
                // So far it seems that we don't need to implement this. Maybe
                // `dladdr` on OSX has us covered? Maybe there's not much in the
                // symbol table? In any case our relevant tests are passing
                // without this being implemented, so let's skip it for now.
                None
            }
        }
    } else {
        use goblin::elf::Elf;

        struct Object<'a> {
            elf: Elf<'a>,
            data: &'a [u8],
        }

        impl<'a> Object<'a> {
            fn parse(data: &'a [u8]) -> Option<Object<'a>> {
                let elf = Elf::parse(data).ok()?;
                if !elf.little_endian {
                    return None;
                }
                Some(Object { elf, data })
            }

            fn section(&self, name: &str) -> Option<&'a [u8]> {
                let section = self.elf.section_headers.iter().find(|section| {
                    match self.elf.shdr_strtab.get(section.sh_name) {
                        Some(Ok(section_name)) => section_name == name,
                        _ => false,
                    }
                });
                section
                    .and_then(|section| {
                        self.data.get(section.sh_offset as usize..)
                            .and_then(|data| data.get(..section.sh_size as usize))
                    })
            }

            fn search_symtab<'b>(&'b self, addr: u64) -> Option<&'b [u8]> {
                let sym = self.elf.syms.iter()
                    .find(|sym| sym.st_value <= addr && addr <= sym.st_value + sym.st_size)?;
                Some(self.elf.strtab.get(sym.st_name)?.ok()?.as_bytes())
            }
        }
    }
}

impl Mapping {
    #[cfg(not(target_os = "macos"))]
    fn new(path: &Path) -> Option<Mapping> {
        let map = mmap(path)?;
        let cx = cx(Object::parse(&map)?)?;
        Some(mk!(Mapping { map, cx }))
    }

    // The loading path for OSX is is so different we just have a completely
    // different implementation of the function here. On OSX we need to go
    // probing the filesystem for a bunch of files.
    #[cfg(target_os = "macos")]
    fn new(path: &Path) -> Option<Mapping> {
        // First up we need to load the unique UUID which is stored in the macho
        // header of the file we're reading, specified at `path`.
        let map = mmap(path)?;
        let macho = MachO::parse(&map, 0).ok()?;
        let uuid = find_uuid(&macho)?;

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
        let inner = cx(Object::parse(macho)?)?;
        return Some(mk!(Mapping { map, inner }));

        fn load_dsym(dir: &Path, uuid: &[u8; 16]) -> Option<Mapping> {
            for entry in dir.read_dir().ok()? {
                let entry = entry.ok()?;
                let map = mmap(&entry.path())?;
                let macho = MachO::parse(&map, 0).ok()?;
                let entry_uuid = find_uuid(&macho)?;
                if entry_uuid != uuid {
                    continue;
                }
                if let Some(cx) = Object::parse(macho).and_then(cx) {
                    return Some(mk!(Mapping { map, cx }));
                }
            }

            None
        }

        fn find_uuid<'a>(object: &'a MachO) -> Option<&'a [u8; 16]> {
            use goblin::mach::load_command::CommandVariant;

            object
                .load_commands
                .iter()
                .filter_map(|cmd| match &cmd.command {
                    CommandVariant::Uuid(u) => Some(&u.uuid),
                    _ => None,
                })
                .next()
        }
    }

    // Ensure the 'static lifetimes don't leak.
    fn rent<F>(&self, mut f: F)
    where
        F: FnMut(&Context<'_>),
    {
        f(&self.cx)
    }
}

type Cache = Vec<(PathBuf, Mapping)>;

// unsafe because this is required to be externally synchronized
unsafe fn with_cache(f: impl FnOnce(&mut Cache)) {
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
    static mut MAPPINGS_CACHE: Option<Cache> = None;

    f(MAPPINGS_CACHE.get_or_insert_with(|| Vec::with_capacity(MAPPINGS_CACHE_SIZE)))
}

// unsafe because this is required to be externally synchronized
pub unsafe fn clear_symbol_cache() {
    with_cache(|cache| cache.clear());
}

unsafe fn with_mapping_for_path<F>(path: PathBuf, f: F)
where
    F: FnMut(&Context<'_>),
{
    with_cache(|cache| {
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
    with_mapping_for_path(path, |cx| {
        let mut called = false;
        if let Ok(mut frames) = cx.dwarf.find_frames(addr.0 as u64) {
            while let Ok(Some(mut frame)) = frames.next() {
                let function = frame.function.take();
                let name = function.as_ref().and_then(|f| f.raw_name().ok());
                let name = name.as_ref().map(|n| n.as_bytes());
                called = true;
                cb.call(Symbol::Frame {
                    addr: addr.0 as *mut c_void,
                    frame,
                    name,
                });
            }
        }

        if !called {
            if let Some(name) = cx.object.search_symtab(addr.0 as u64) {
                cb.call(Symbol::Symtab {
                    addr: addr.0 as *mut c_void,
                    name,
                });
            }
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
    /// Couldn't find debug information, but we found it in the symbol table of
    /// the elf executable.
    Symtab { addr: *mut c_void, name: &'a [u8] },
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
            Symbol::Symtab { name, .. } => Some(SymbolName::new(name)),
        }
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        match self {
            Symbol::Dladdr(s) => s.addr(),
            Symbol::Frame { addr, .. } => Some(*addr),
            Symbol::Symtab { .. } => None,
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
            Symbol::Symtab { .. } => None,
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
            Symbol::Symtab { .. } => None,
        }
    }

    pub fn lineno(&self) -> Option<u32> {
        match self {
            Symbol::Dladdr(s) => return s.lineno(),
            Symbol::Frame { frame, .. } => {
                let location = frame.location.as_ref()?;
                location.line.and_then(|l| u32::try_from(l).ok())
            }
            Symbol::Symtab { .. } => None,
        }
    }
}
