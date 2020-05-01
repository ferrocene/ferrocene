//! Support for symbolication using the `gimli` crate on crates.io
//!
//! This implementation is largely a work in progress and is off by default for
//! all platforms, but it's hoped to be developed over time! Long-term this is
//! intended to wholesale replace the `libbacktrace.rs` implementation.

use self::gimli::read::EndianSlice;
use self::gimli::LittleEndian as Endian;
use self::mmap::Mmap;
use crate::symbolize::dladdr;
use crate::symbolize::ResolveWhat;
use crate::types::BytesOrWideString;
use crate::SymbolName;
use addr2line::gimli;
use core::mem;
use core::u32;
use libc::c_void;
use std::convert::TryInto;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;
use std::prelude::v1::*;

#[cfg(windows)]
#[path = "gimli/mmap_windows.rs"]
mod mmap;
#[cfg(unix)]
#[path = "gimli/mmap_unix.rs"]
mod mmap;

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
    let len = file.metadata().ok()?.len().try_into().ok()?;
    unsafe { Mmap::map(&file, len) }
}

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        use goblin::pe::{self, PE};
        use goblin::strtab::Strtab;
        use std::cmp;
        use std::convert::TryFrom;

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

        fn native_libraries() -> Vec<Library> {
            Vec::new()
        }
    } else if #[cfg(target_os = "macos")] {
        use goblin::mach::MachO;
        use std::os::unix::prelude::*;
        use std::ffi::{OsStr, CStr};

        struct Object<'a> {
            macho: MachO<'a>,
            dwarf: Option<usize>,
            syms: Vec<(&'a str, u64)>,
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
                let mut syms = Vec::new();
                if let Some(s) = &macho.symbols {
                    syms = s.iter()
                        .filter_map(|e| e.ok())
                        .filter(|(name, nlist)| name.len() > 0 && !nlist.is_undefined())
                        .map(|(name, nlist)| (name, nlist.n_value))
                        .collect();
                }
                syms.sort_unstable_by_key(|(_, addr)| *addr);
                Some(Object { macho, dwarf, syms })
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

            fn search_symtab<'b>(&'b self, addr: u64) -> Option<&'b [u8]> {
                let i = match self.syms.binary_search_by_key(&addr, |(_, addr)| *addr) {
                    Ok(i) => i,
                    Err(i) => i.checked_sub(1)?,
                };
                let (sym, _addr) = self.syms.get(i)?;
                Some(sym.as_bytes())
            }
        }

        #[allow(deprecated)]
        fn native_libraries() -> Vec<Library> {
            let mut ret = Vec::new();
            unsafe {
                for i in 0..libc::_dyld_image_count() {
                    ret.extend(native_library(i));
                }
            }
            return ret;
        }

        #[allow(deprecated)]
        unsafe fn native_library(i: u32) -> Option<Library> {
            let name = libc::_dyld_get_image_name(i);
            if name.is_null() {
                return None;
            }
            let name = CStr::from_ptr(name);
            let header = libc::_dyld_get_image_header(i);
            if header.is_null() {
                return None;
            }
            let mut segments = Vec::new();
            match (*header).magic {
                libc::MH_MAGIC => {
                    let mut next_cmd = header.offset(1) as *const libc::load_command;
                    for _ in 0..(*header).ncmds {
                        segments.extend(segment(next_cmd));
                        next_cmd = (next_cmd as usize + (*next_cmd).cmdsize as usize) as *const _;
                    }
                }
                libc::MH_MAGIC_64 => {
                    let header = header as *const libc::mach_header_64;
                    let mut next_cmd = header.offset(1) as *const libc::load_command;
                    for _ in 0..(*header).ncmds {
                        segments.extend(segment(next_cmd));
                        next_cmd = (next_cmd as usize + (*next_cmd).cmdsize as usize) as *const _;
                    }
                }
                _ => return None,
            }
            Some(Library {
                name: OsStr::from_bytes(name.to_bytes()).to_owned(),
                segments,
                bias: libc::_dyld_get_image_vmaddr_slide(i) as *const u8,
            })
        }

        unsafe fn segment(cmd: *const libc::load_command) -> Option<LibrarySegment> {
            Some(match (*cmd).cmd {
                libc::LC_SEGMENT => {
                    let cmd = cmd as *const libc::segment_command;
                    LibrarySegment {
                        len: (*cmd).vmsize as usize,
                        stated_virtual_memory_address: (*cmd).vmaddr as *const u8,
                    }
                }
                libc::LC_SEGMENT_64 => {
                    let cmd = cmd as *const libc::segment_command_64;
                    LibrarySegment {
                        len: (*cmd).vmsize as usize,
                        stated_virtual_memory_address: (*cmd).vmaddr as *const u8,
                    }
                }
                _ => return None,
            })
        }
    } else {
        use goblin::elf::Elf;
        use std::os::unix::prelude::*;
        use std::ffi::{OsStr, CStr};

        struct Object<'a> {
            elf: Elf<'a>,
            data: &'a [u8],
            // List of pre-parsed and sorted symbols by base address. The
            // boolean indicates whether it comes from the dynamic symbol table
            // or the normal symbol table, affecting where it's symbolicated.
            syms: Vec<(goblin::elf::Sym, bool)>,
        }

        impl<'a> Object<'a> {
            fn parse(data: &'a [u8]) -> Option<Object<'a>> {
                let elf = Elf::parse(data).ok()?;
                if !elf.little_endian {
                    return None;
                }
                let mut syms = elf
                    .syms
                    .iter()
                    .map(|s| (s, false))
                    .chain(elf.dynsyms.iter().map(|s| (s, true)))
                    // Only look at function/object symbols. This mirrors what
                    // libbacktrace does and in general we're only symbolicating
                    // function addresses in theory. Object symbols correspond
                    // to data, and maybe someone's crazy enough to have a
                    // function go into static data?
                    .filter(|(s, _)| {
                        s.is_function() || s.st_type() == goblin::elf::sym::STT_OBJECT
                    })
                    // skip anything that's in an undefined section header,
                    // since it means it's an imported function and we're only
                    // symbolicating with locally defined functions.
                    .filter(|(s, _)| {
                        s.st_shndx != goblin::elf::section_header::SHN_UNDEF as usize
                    })
                    .collect::<Vec<_>>();
                syms.sort_unstable_by_key(|s| s.0.st_value);
                Some(Object {
                    syms,
                    elf,
                    data,
                })
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
                // Same sort of binary search as Windows above
                let i = match self.syms.binary_search_by_key(&addr, |s| s.0.st_value) {
                    Ok(i) => i,
                    Err(i) => i.checked_sub(1)?,
                };
                let (sym, dynamic) = self.syms.get(i)?;
                if sym.st_value <= addr && addr <= sym.st_value + sym.st_size {
                    let strtab = if *dynamic {
                        &self.elf.dynstrtab
                    } else {
                        &self.elf.strtab
                    };
                    Some(strtab.get(sym.st_name)?.ok()?.as_bytes())
                } else {
                    None
                }
            }
        }

        fn native_libraries() -> Vec<Library> {
            let mut ret = Vec::new();
            unsafe {
                libc::dl_iterate_phdr(Some(callback), &mut ret as *mut _ as *mut _);
            }
            return ret;
        }

        unsafe extern "C" fn callback(
            info: *mut libc::dl_phdr_info,
            _size: libc::size_t,
            vec: *mut libc::c_void,
        ) -> libc::c_int {
            let libs = &mut *(vec as *mut Vec<Library>);
            let name = if (*info).dlpi_name.is_null() || *(*info).dlpi_name == 0{
                if libs.is_empty() {
                    std::env::current_exe().map(|e| e.into()).unwrap_or_default()
                } else {
                    OsString::new()
                }
            } else {
                let bytes = CStr::from_ptr((*info).dlpi_name).to_bytes();
                OsStr::from_bytes(bytes).to_owned()
            };
            let headers = std::slice::from_raw_parts((*info).dlpi_phdr, (*info).dlpi_phnum as usize);
            libs.push(Library {
                name,
                segments: headers
                    .iter()
                    .map(|header| LibrarySegment {
                        len: (*header).p_memsz as usize,
                        stated_virtual_memory_address: (*header).p_vaddr as *const u8,
                    })
                    .collect(),
                bias: (*info).dlpi_addr as *const u8,
            });
            0
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
}

#[derive(Default)]
struct Cache {
    /// All known shared libraries that have been loaded.
    libraries: Vec<Library>,

    /// Mappings cache where we retain parsed dwarf information.
    ///
    /// This list has a fixed capacity for its entire liftime which never
    /// increases. The `usize` element of each pair is an index into `libraries`
    /// above where `usize::max_value()` represents the current executable. The
    /// `Mapping` is corresponding parsed dwarf information.
    ///
    /// Note that this is basically an LRU cache and we'll be shifting things
    /// around in here as we symbolize addresses.
    mappings: Vec<(usize, Mapping)>,
}

struct Library {
    name: OsString,
    segments: Vec<LibrarySegment>,
    bias: *const u8,
}

struct LibrarySegment {
    len: usize,
    stated_virtual_memory_address: *const u8,
}

// unsafe because this is required to be externally synchronized
pub unsafe fn clear_symbol_cache() {
    Cache::with_global(|cache| cache.mappings.clear());
}

impl Cache {
    fn new() -> Cache {
        Cache {
            mappings: Vec::with_capacity(MAPPINGS_CACHE_SIZE),
            libraries: native_libraries(),
        }
    }

    // unsafe because this is required to be externally synchronized
    unsafe fn with_global(f: impl FnOnce(&mut Self)) {
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

        f(MAPPINGS_CACHE.get_or_insert_with(|| Cache::new()))
    }

    fn avma_to_svma(&self, addr: *const u8) -> Option<(usize, *const u8)> {
        // Note that we don't implement iterating native libraries on Windows,
        // so we just unhelpfully assume that the address is an SVMA.
        // Surprisingly it seems to at least somewhat work on Wine on Linux
        // though...
        //
        // This probably means ASLR on Windows is busted.
        if cfg!(windows) {
            return Some((usize::max_value(), addr));
        }

        self.libraries
            .iter()
            .enumerate()
            .filter_map(|(i, lib)| {
                // First up, test if this `lib` has any segment containing the
                // `addr` (handling relocation). If this check passes then we
                // can continue below and actually translate the address.
                if !lib.segments.iter().any(|s| {
                    let svma = s.stated_virtual_memory_address as usize;
                    let start = svma + lib.bias as usize;
                    let end = start + s.len;
                    let address = addr as usize;
                    start <= address && address < end
                }) {
                    return None;
                }

                // Now that we know `lib` contains `addr`, we can offset with
                // the bias to find the stated virutal memory address.
                let svma = addr as usize - lib.bias as usize;
                Some((i, svma as *const u8))
            })
            .next()
    }

    fn mapping_for_lib<'a>(&'a mut self, lib: usize) -> Option<&'a Context<'a>> {
        let idx = self.mappings.iter().position(|(idx, _)| *idx == lib);

        // Invariant: after this conditional completes without early returning
        // from an error, the cache entry for this path is at index 0.

        if let Some(idx) = idx {
            // When the mapping is already in the cache, move it to the front.
            if idx != 0 {
                let entry = self.mappings.remove(idx);
                self.mappings.insert(0, entry);
            }
        } else {
            // When the mapping is not in the cache, create a new mapping,
            // insert it into the front of the cache, and evict the oldest cache
            // entry if necessary.
            let storage;
            let path = match self.libraries.get(lib) {
                Some(lib) => &lib.name,
                None => {
                    storage = env::current_exe().ok()?.into();
                    &storage
                }
            };
            let mapping = Mapping::new(path.as_ref())?;

            if self.mappings.len() == MAPPINGS_CACHE_SIZE {
                self.mappings.pop();
            }

            self.mappings.insert(0, (lib, mapping));
        }

        let cx: &'a Context<'static> = &self.mappings[0].1.cx;
        // don't leak the `'static` lifetime, make sure it's scoped to just
        // ourselves
        Some(unsafe { mem::transmute::<&'a Context<'static>, &'a Context<'a>>(cx) })
    }
}

pub unsafe fn resolve(what: ResolveWhat, cb: &mut FnMut(&super::Symbol)) {
    let addr = what.address_or_ip();
    let mut cb = DladdrFallback {
        cb,
        addr,
        called: false,
    };

    Cache::with_global(|cache| {
        let (lib, addr) = match cache.avma_to_svma(addr as *const u8) {
            Some(pair) => pair,
            None => return,
        };

        // Finally, get a cached mapping or create a new mapping for this file, and
        // evaluate the DWARF info to find the file/line/name for this address.
        let cx = match cache.mapping_for_lib(lib) {
            Some(cx) => cx,
            None => return,
        };
        if let Ok(mut frames) = cx.dwarf.find_frames(addr as u64) {
            while let Ok(Some(frame)) = frames.next() {
                cb.call(Symbol::Frame {
                    addr: addr as *mut c_void,
                    location: frame.location,
                    name: frame.function.map(|f| f.name.slice()),
                });
            }
        }

        if !cb.called {
            if let Some(name) = cx.object.search_symtab(addr as u64) {
                cb.call(Symbol::Symtab {
                    addr: addr as *mut c_void,
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
        location: Option<addr2line::Location<'a>>,
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
            Symbol::Frame { location, .. } => {
                let file = location.as_ref()?.file?;
                Some(BytesOrWideString::Bytes(file.as_bytes()))
            }
            Symbol::Symtab { .. } => None,
        }
    }

    pub fn filename(&self) -> Option<&Path> {
        match self {
            Symbol::Dladdr(s) => return s.filename(),
            Symbol::Frame { location, .. } => {
                let file = location.as_ref()?.file?;
                Some(Path::new(file))
            }
            Symbol::Symtab { .. } => None,
        }
    }

    pub fn lineno(&self) -> Option<u32> {
        match self {
            Symbol::Dladdr(s) => return s.lineno(),
            Symbol::Frame { location, .. } => location.as_ref()?.line,
            Symbol::Symtab { .. } => None,
        }
    }
}
