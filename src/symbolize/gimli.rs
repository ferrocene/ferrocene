//! Support for symbolication using the `gimli` crate on crates.io
//!
//! This implementation is largely a work in progress and is off by default for
//! all platforms, but it's hoped to be developed over time! Long-term this is
//! intended to wholesale replace the `libbacktrace.rs` implementation.

use self::gimli::read::EndianRcSlice;
use self::gimli::RunTimeEndian;
use crate::symbolize::dladdr;
use crate::symbolize::ResolveWhat;
use crate::types::BytesOrWideString;
use crate::SymbolName;
use addr2line::gimli;
use addr2line::object::{self, Object};
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

type Symbols<'map> = object::SymbolMap<'map>;

struct Mapping {
    dwarf: addr2line::Context,
    // 'static lifetime is a lie to hack around lack of support for self-referential structs.
    symbols: Symbols<'static>,
    _map: Mmap,
}

macro_rules! mk {
    (Mapping { $map:expr, $object:expr }) => {{
        Mapping {
            dwarf: addr2line::Context::new(&$object).ok()?,
            // Convert to 'static lifetimes since the symbols should
            // only borrow `map` and we're preserving `map` below.
            //
            // TODO: how do we know that `symbol_map` *only* borrows `map`?
            symbols: unsafe { mem::transmute::<Symbols, Symbols<'static>>($object.symbol_map()) },
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
    fn new(path: &Path) -> Option<Mapping> {
        if cfg!(target_os = "macos") {
            Mapping::new_find_dsym(path)
        } else {
            let map = mmap(path)?;
            let object = object::ElfFile::parse(&map).ok()?;
            Some(mk!(Mapping { map, object }))
        }
    }

    fn new_find_dsym(path: &Path) -> Option<Mapping> {
        // First up we need to load the unique UUID which is stored in the macho
        // header of the file we're reading, specified at `path`.
        let map = mmap(path)?;
        let object = object::MachOFile::parse(&map).ok()?;
        let uuid = get_uuid(&object)?;

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
        return Some(mk!(Mapping { map, object }));

        fn load_dsym(dir: &Path, uuid: &[u8]) -> Option<Mapping> {
            for entry in dir.read_dir().ok()? {
                let entry = entry.ok()?;
                let map = mmap(&entry.path())?;
                let object = object::MachOFile::parse(&map).ok()?;
                let entry_uuid = get_uuid(&object)?;
                if &entry_uuid[..] != uuid {
                    continue;
                }
                return Some(mk!(Mapping { map, object }));
            }

            None
        }

        fn get_uuid(object: &object::MachOFile) -> Option<[u8; 16]> {
            use goblin::mach::load_command::CommandVariant;

            object
                .macho()
                .load_commands
                .iter()
                .filter_map(|cmd| match cmd.command {
                    CommandVariant::Uuid(u) => Some(u.uuid),
                    _ => None,
                })
                .next()
        }
    }

    // Ensure the 'static lifetimes don't leak.
    fn rent<F>(&self, mut f: F)
    where
        F: FnMut(&addr2line::Context, &Symbols),
    {
        f(&self.dwarf, &self.symbols)
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
    F: FnMut(&addr2line::Context, &Symbols),
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
    let (addr, path) = match so_info {
        None => return,
        Some((a, p)) => (a, p),
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
    with_mapping_for_path(path, |dwarf, symbols| {
        let mut found_sym = false;
        if let Ok(mut frames) = dwarf.find_frames(addr.0 as u64) {
            while let Ok(Some(frame)) = frames.next() {
                let name = frame.function.as_ref().and_then(|f| f.raw_name().ok());
                let name = name.as_ref().map(|n| n.as_bytes() as *const _);
                cb.call(&super::Symbol {
                    inner: Symbol::Frame {
                        addr: addr.0 as *mut c_void,
                        frame,
                        name,
                    },
                });
                found_sym = true;
            }
        }

        // No DWARF info found, so fallback to the symbol table.
        if !found_sym {
            if let Some(name) = symbols.get(addr.0 as u64).and_then(|x| x.name()) {
                let sym = super::Symbol {
                    inner: Symbol::Symbol {
                        addr: addr.0 as usize as *mut c_void,
                        name: name.as_bytes(),
                    },
                };
                cb.call(&sym);
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
    fn call(&mut self, sym: &super::Symbol) {
        self.called = true;
        (self.cb)(sym);
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

pub enum Symbol {
    /// We were able to locate frame information for this symbol, and
    /// `addr2line`'s frame internally has all the nitty gritty details.
    Frame {
        addr: *mut c_void,
        frame: addr2line::Frame<EndianRcSlice<RunTimeEndian>>,
        // Note that this would ideally be `&[u8]`, but we currently can't have
        // a lifetime parameter on this type. Eventually in the next breaking
        // change of this crate we should add a lifetime parameter to the
        // `Symbol` type in the top-level module, and then thread that lifetime
        // through to here.
        name: Option<*const [u8]>,
    },
    /// We weren't able to find anything in the debuginfo for this address, but
    /// we were able to find an entry into the symbol table for the symbol name.
    Symbol {
        addr: *mut c_void,
        // see note on `name` above
        name: *const [u8],
    },
    /// We weren't able to find anything in the original file, so we had to fall
    /// back to using `dladdr` which had a hit.
    Dladdr(dladdr::Symbol),
}

impl Symbol {
    pub fn name(&self) -> Option<SymbolName> {
        match self {
            Symbol::Dladdr(s) => s.name(),
            Symbol::Frame { name, .. } => {
                let name = name.as_ref()?;
                unsafe { Some(SymbolName::new(&**name)) }
            }
            Symbol::Symbol { name, .. } => unsafe { Some(SymbolName::new(&**name)) },
        }
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        match self {
            Symbol::Dladdr(s) => s.addr(),
            Symbol::Frame { addr, .. } => Some(*addr),
            Symbol::Symbol { addr, .. } => Some(*addr),
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
            Symbol::Symbol { .. } => None,
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
            Symbol::Symbol { .. } => None,
        }
    }

    pub fn lineno(&self) -> Option<u32> {
        match self {
            Symbol::Dladdr(s) => return s.lineno(),
            Symbol::Frame { frame, .. } => {
                let location = frame.location.as_ref()?;
                location.line.and_then(|l| u32::try_from(l).ok())
            }
            Symbol::Symbol { .. } => None,
        }
    }
}
