//! Support for symbolication using the `gimli` crate on crates.io
//!
//! This implementation is largely a work in progress and is off by default for
//! all platforms, but it's hoped to be developed over time! Long-term this is
//! intended to wholesale replace the `libbacktrace.rs` implementation.

use self::gimli::read::EndianSlice;
use self::gimli::LittleEndian as Endian;
use self::mmap::Mmap;
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
        use object::{Bytes, LittleEndian as LE};
        use object::pe::{ImageDosHeader, ImageSymbol};
        use object::read::StringTable;
        use object::read::pe::{ImageNtHeaders, ImageOptionalHeader, SectionTable};
        #[cfg(target_pointer_width = "32")]
        type Pe = object::pe::ImageNtHeaders32;
        #[cfg(target_pointer_width = "64")]
        type Pe = object::pe::ImageNtHeaders64;
        use std::convert::TryFrom;

        struct Object<'a> {
            data: Bytes<'a>,
            sections: SectionTable<'a>,
            symbols: Vec<(usize, &'a ImageSymbol)>,
            strings: StringTable<'a>,
        }

        impl<'a> Object<'a> {
            fn parse(data: &'a [u8]) -> Option<Object<'a>> {
                let data = Bytes(data);
                let dos_header = ImageDosHeader::parse(data).ok()?;
                let (nt_headers, _, nt_tail) = dos_header.nt_headers::<Pe>(data).ok()?;
                let sections = nt_headers.sections(nt_tail).ok()?;
                let symtab = nt_headers.symbols(data).ok()?;
                let strings = symtab.strings();
                let image_base = usize::try_from(nt_headers.optional_header().image_base()).ok()?;

                // Collect all the symbols into a local vector which is sorted
                // by address and contains enough data to learn about the symbol
                // name. Note that we only look at function symbols and also
                // note that the sections are 1-indexed because the zero section
                // is special (apparently).
                let mut symbols = Vec::new();
                let mut i = 0;
                let len = symtab.len();
                while i < len {
                    let sym = symtab.symbol(i)?;
                    i += 1 + sym.number_of_aux_symbols as usize;
                    let section_number = sym.section_number.get(LE);
                    if sym.derived_type() != object::pe::IMAGE_SYM_DTYPE_FUNCTION
                        || section_number == 0
                    {
                        continue;
                    }
                    let addr = usize::try_from(sym.value.get(LE)).ok()?;
                    let section = sections.section(usize::try_from(section_number).ok()?).ok()?;
                    let va = usize::try_from(section.virtual_address.get(LE)).ok()?;
                    symbols.push((addr + va + image_base, sym));
                }
                symbols.sort_unstable_by_key(|x| x.0);
                Some(Object { data, sections, strings, symbols })
            }

            fn section(&self, name: &str) -> Option<&'a [u8]> {
                Some(self.sections
                    .section_by_name(self.strings, name.as_bytes())?
                    .1
                    .pe_data(self.data)
                    .ok()?
                    .0)
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
                self.symbols[i].1.name(self.strings).ok()
            }
        }

        fn native_libraries() -> Vec<Library> {
            Vec::new()
        }
    } else if #[cfg(target_os = "macos")] {
        use std::os::unix::prelude::*;
        use std::ffi::{OsStr, CStr};
        use object::{Bytes, NativeEndian};
        use object::macho;
        use object::read::macho::{MachHeader, Section, Segment as _, Nlist};

        #[cfg(target_pointer_width = "32")]
        type Mach = object::macho::MachHeader32<NativeEndian>;
        #[cfg(target_pointer_width = "64")]
        type Mach = object::macho::MachHeader64<NativeEndian>;
        type MachSegment = <Mach as MachHeader>::Segment;
        type MachSection = <Mach as MachHeader>::Section;
        type MachNlist = <Mach as MachHeader>::Nlist;

        struct Object<'a> {
            endian: NativeEndian,
            data: Bytes<'a>,
            dwarf: Option<&'a [MachSection]>,
            syms: Vec<(&'a [u8], u64)>,
        }

        impl<'a> Object<'a> {
            fn parse(mach: &'a Mach, endian: NativeEndian, data: Bytes<'a>) -> Option<Object<'a>> {
                let mut dwarf = None;
                let mut syms = Vec::new();
                let mut commands = mach.load_commands(endian, data).ok()?;
                while let Ok(Some(command)) = commands.next() {
                    if let Some((segment, section_data)) = MachSegment::from_command(command).ok()? {
                        if segment.name() == b"__DWARF" {
                            dwarf = segment.sections(endian, section_data).ok();
                        }
                    } else if let Some(symtab) = command.symtab().ok()? {
                        let symbols = symtab.symbols::<Mach>(endian, data).ok()?;
                        syms = symbols.iter()
                            .filter_map(|nlist: &MachNlist| {
                                let name = nlist.name(endian, symbols.strings()).ok()?;
                                if name.len() > 0 && !nlist.is_undefined() {
                                    Some((name, nlist.n_value(endian)))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        syms.sort_unstable_by_key(|(_, addr)| *addr);
                    }
                }

                Some(Object { endian, data, dwarf, syms })
            }

            fn section(&self, name: &str) -> Option<&'a [u8]> {
                let name = name.as_bytes();
                let dwarf = self.dwarf?;
                let section = dwarf
                    .into_iter()
                    .find(|section| {
                        let section_name = section.name();
                        section_name == name || {
                            section_name.starts_with(b"__")
                                && name.starts_with(b".")
                                && &section_name[2..] == &name[1..]
                        }
                    })?;
                Some(section.data(self.endian, self.data).ok()?.0)
            }

            fn search_symtab<'b>(&'b self, addr: u64) -> Option<&'b [u8]> {
                let i = match self.syms.binary_search_by_key(&addr, |(_, addr)| *addr) {
                    Ok(i) => i,
                    Err(i) => i.checked_sub(1)?,
                };
                let (sym, _addr) = self.syms.get(i)?;
                Some(sym)
            }
        }

        fn find_header(mut data: Bytes<'_>) -> Option<(&'_ Mach, Bytes<'_>)> {
            use object::endian::BigEndian;

            let desired_cpu = || {
                if cfg!(target_arch = "x86") {
                    Some(macho::CPU_TYPE_X86)
                } else if cfg!(target_arch = "x86_64") {
                    Some(macho::CPU_TYPE_X86_64)
                } else if cfg!(target_arch = "arm") {
                    Some(macho::CPU_TYPE_ARM)
                } else if cfg!(target_arch = "aarch64") {
                    Some(macho::CPU_TYPE_ARM64)
                } else {
                    None
                }
            };

            match data.clone().read::<object::endian::U32<NativeEndian>>().ok()?.get(NativeEndian) {
                macho::MH_MAGIC_64
                | macho::MH_CIGAM_64
                | macho::MH_MAGIC
                | macho::MH_CIGAM => {}

                macho::FAT_MAGIC | macho::FAT_CIGAM => {
                    let mut header_data = data;
                    let endian = BigEndian;
                    let header = header_data.read::<macho::FatHeader>().ok()?;
                    let nfat = header.nfat_arch.get(endian);
                    let arch = (0..nfat)
                        .filter_map(|_| header_data.read::<macho::FatArch32>().ok())
                        .find(|arch| desired_cpu() == Some(arch.cputype.get(endian)))?;
                    let offset = arch.offset.get(endian);
                    let size = arch.size.get(endian);
                    data = data.read_bytes_at(
                        offset.try_into().ok()?,
                        size.try_into().ok()?,
                    ).ok()?;
                }

                macho::FAT_MAGIC_64 | macho::FAT_CIGAM_64 => {
                    let mut header_data = data;
                    let endian = BigEndian;
                    let header = header_data.read::<macho::FatHeader>().ok()?;
                    let nfat = header.nfat_arch.get(endian);
                    let arch = (0..nfat)
                        .filter_map(|_| header_data.read::<macho::FatArch64>().ok())
                        .find(|arch| desired_cpu() == Some(arch.cputype.get(endian)))?;
                    let offset = arch.offset.get(endian);
                    let size = arch.size.get(endian);
                    data = data.read_bytes_at(
                        offset.try_into().ok()?,
                        size.try_into().ok()?,
                    ).ok()?;
                }

                _ => return None,
            }

            Mach::parse(data).ok().map(|h| (h, data))
        }

        #[allow(deprecated)]
        fn native_libraries() -> Vec<Library> {
            let mut ret = Vec::new();
            let images = unsafe { libc::_dyld_image_count() };
            for i in 0..images {
                ret.extend(native_library(i));
            }
            return ret;
        }

        #[allow(deprecated)]
        fn native_library(i: u32) -> Option<Library> {
            // Fetch the name of this library which corresponds to the path of
            // where to load it as well.
            let name = unsafe {
                let name = libc::_dyld_get_image_name(i);
                if name.is_null() {
                    return None;
                }
                CStr::from_ptr(name)
            };

            // Load the image header of this library and delegate to `object` to
            // parse all the load commands so we can figure out all the segments
            // involved here.
            let (mut load_commands, endian) = unsafe {
                let header = libc::_dyld_get_image_header(i);
                if header.is_null() {
                    return None;
                }
                match (*header).magic {
                    macho::MH_MAGIC => {
                        let endian = NativeEndian;
                        let header = &*(header as *const macho::MachHeader32<NativeEndian>);
                        let data = core::slice::from_raw_parts(
                            header as *const _ as *const u8,
                            mem::size_of_val(header) + header.sizeofcmds.get(endian) as usize
                        );
                        (header.load_commands(endian, Bytes(data)).ok()?, endian)
                    }
                    macho::MH_MAGIC_64 => {
                        let endian = NativeEndian;
                        let header = &*(header as *const macho::MachHeader64<NativeEndian>);
                        let data = core::slice::from_raw_parts(
                            header as *const _ as *const u8,
                            mem::size_of_val(header) + header.sizeofcmds.get(endian) as usize
                        );
                        (header.load_commands(endian, Bytes(data)).ok()?, endian)
                    }
                    _ => return None,
                }
            };

            // Iterate over the segments and register known regions for segments
            // that we find. Additionally record information bout text segments
            // for processing later, see comments below.
            let mut segments = Vec::new();
            let mut first_text = 0;
            let mut text_fileoff_zero = false;
            while let Some(cmd) = load_commands.next().ok()? {
                if let Some((seg, _)) = cmd.segment_32().ok()? {
                    if seg.name() == b"__TEXT" {
                        first_text = segments.len();
                        if seg.fileoff(endian) == 0 && seg.filesize(endian) > 0 {
                            text_fileoff_zero = true;
                        }
                    }
                    segments.push(LibrarySegment {
                        len: seg.vmsize(endian).try_into().ok()?,
                        stated_virtual_memory_address: seg.vmaddr(endian).try_into().ok()?,
                    });
                }
                if let Some((seg, _)) = cmd.segment_64().ok()? {
                    if seg.name() == b"__TEXT" {
                        first_text = segments.len();
                        if seg.fileoff(endian) == 0 && seg.filesize(endian) > 0 {
                            text_fileoff_zero = true;
                        }
                    }
                    segments.push(LibrarySegment {
                        len: seg.vmsize(endian).try_into().ok()?,
                        stated_virtual_memory_address: seg.vmaddr(endian).try_into().ok()?,
                    });
                }
            }

            // Determine the "slide" for this library which ends up being the
            // bias we use to figure out where in memory objects are loaded.
            // This is a bit of a weird computation though and is the result of
            // trying a few things in the wild and seeing what sticks.
            //
            // The general idea is that the `bias` plus a segment's
            // `stated_virtual_memory_address` is going to be where in the
            // actual address space the segment resides. The other thing we rely
            // on though is that a real address minus the `bias` is the index to
            // look up in the symbol table and debuginfo.
            //
            // It turns out, though, that for system loaded libraries these
            // calculations are incorrect. For native executables, however, it
            // appears correct. Lifting some logic from LLDB's source it has
            // some special-casing for the first `__TEXT` section loaded from
            // file offset 0 with a nonzero size. For whatever reason when this
            // is present it appears to mean that the symbol table is relative
            // to just the vmaddr slide for the library. If it's *not* present
            // then the symbol table is relative to the the vmaddr slide plus
            // the segment's stated address.
            //
            // To handle this situation if we *don't* find a text section at
            // file offset zero then we increase the bias by the first text
            // sections's stated address and decrease all stated addresses by
            // that amount as well. That way the symbol table is always appears
            // relative to the library's bias amount. This appears to have the
            // right results for symbolizing via the symbol table.
            //
            // Honestly I'm not entirely sure whether this is right or if
            // there's something else that should indicate how to do this. For
            // now though this seems to work well enough (?) and we should
            // always be able to tweak this over time if necessary.
            //
            // For some more information see #318
            let mut slide = unsafe { libc::_dyld_get_image_vmaddr_slide(i) as usize };
            if !text_fileoff_zero {
                let adjust = segments[first_text].stated_virtual_memory_address;
                for segment in segments.iter_mut() {
                    segment.stated_virtual_memory_address -= adjust;
                }
                slide += adjust;
            }

            Some(Library {
                name: OsStr::from_bytes(name.to_bytes()).to_owned(),
                segments,
                bias: slide,
            })
        }
    } else {
        use std::os::unix::prelude::*;
        use std::ffi::{OsStr, CStr};
        use object::{Bytes, NativeEndian};
        use object::read::StringTable;
        use object::read::elf::{FileHeader, SectionHeader, SectionTable, Sym};
        #[cfg(target_pointer_width = "32")]
        type Elf = object::elf::FileHeader32<NativeEndian>;
        #[cfg(target_pointer_width = "64")]
        type Elf = object::elf::FileHeader64<NativeEndian>;

        struct ParsedSym {
            address: u64,
            size: u64,
            name: u32,
        }

        struct Object<'a> {
            /// Zero-sized type representing the native endianness.
            ///
            /// We could use a literal instead, but this helps ensure correctness.
            endian: NativeEndian,
            /// The entire file data.
            data: Bytes<'a>,
            sections: SectionTable<'a, Elf>,
            strings: StringTable<'a>,
            /// List of pre-parsed and sorted symbols by base address.
            syms: Vec<ParsedSym>,
        }

        impl<'a> Object<'a> {
            fn parse(data: &'a [u8]) -> Option<Object<'a>> {
                let data = object::Bytes(data);
                let elf = Elf::parse(data).ok()?;
                let endian = elf.endian().ok()?;
                let sections = elf.sections(endian, data).ok()?;
                let mut syms = sections.symbols(endian, data, object::elf::SHT_SYMTAB).ok()?;
                if syms.is_empty() {
                    syms = sections.symbols(endian, data, object::elf::SHT_DYNSYM).ok()?;
                }
                let strings = syms.strings();

                let mut syms = syms
                    .iter()
                    // Only look at function/object symbols. This mirrors what
                    // libbacktrace does and in general we're only symbolicating
                    // function addresses in theory. Object symbols correspond
                    // to data, and maybe someone's crazy enough to have a
                    // function go into static data?
                    .filter(|sym| {
                        let st_type = sym.st_type();
                        st_type == object::elf::STT_FUNC || st_type == object::elf::STT_OBJECT
                    })
                    // skip anything that's in an undefined section header,
                    // since it means it's an imported function and we're only
                    // symbolicating with locally defined functions.
                    .filter(|sym| {
                        sym.st_shndx(endian) != object::elf::SHN_UNDEF
                    })
                    .map(|sym| {
                        let address = sym.st_value(endian);
                        let size = sym.st_size(endian);
                        let name = sym.st_name(endian);
                        ParsedSym {
                            address,
                            size,
                            name,
                        }
                    })
                    .collect::<Vec<_>>();
                syms.sort_unstable_by_key(|s| s.address);
                Some(Object {
                    endian,
                    data,
                    sections,
                    strings,
                    syms,
                })
            }

            fn section(&self, name: &str) -> Option<&'a [u8]> {
                Some(self.sections
                    .section_by_name(self.endian, name.as_bytes())?
                    .1
                    .data(self.endian, self.data)
                    .ok()?
                    .0)
            }

            fn search_symtab<'b>(&'b self, addr: u64) -> Option<&'b [u8]> {
                // Same sort of binary search as Windows above
                let i = match self.syms.binary_search_by_key(&addr, |sym| sym.address) {
                    Ok(i) => i,
                    Err(i) => i.checked_sub(1)?,
                };
                let sym = self.syms.get(i)?;
                if sym.address <= addr && addr <= sym.address + sym.size {
                    self.strings.get(sym.name).ok()
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
                        stated_virtual_memory_address: (*header).p_vaddr as usize,
                    })
                    .collect(),
                bias: (*info).dlpi_addr as usize,
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
        let (macho, data) = find_header(Bytes(&map))?;
        let endian = macho.endian().ok()?;
        let uuid = macho.uuid(endian, data).ok()??;

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
            if let Some(mapping) = load_dsym(&candidates, uuid) {
                return Some(mapping);
            }
        }

        // Looks like nothing matched our UUID, so let's at least return our own
        // file. This should have the symbol table for at least some
        // symbolication purposes.
        let inner = cx(Object::parse(macho, endian, data)?)?;
        return Some(mk!(Mapping { map, inner }));

        fn load_dsym(dir: &Path, uuid: [u8; 16]) -> Option<Mapping> {
            for entry in dir.read_dir().ok()? {
                let entry = entry.ok()?;
                let map = mmap(&entry.path())?;
                let (macho, data) = find_header(Bytes(&map))?;
                let endian = macho.endian().ok()?;
                let entry_uuid = macho.uuid(endian, data).ok()??;
                if entry_uuid != uuid {
                    continue;
                }
                if let Some(cx) = Object::parse(macho, endian, data).and_then(cx) {
                    return Some(mk!(Mapping { map, cx }));
                }
            }

            None
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
    /// Segments of this library loaded into memory, and where they're loaded.
    segments: Vec<LibrarySegment>,
    /// The "bias" of this library, typically where it's loaded into memory.
    /// This value is added to each segment's stated address to get the actual
    /// virtual memory address that the segment is loaded into. Additionally
    /// this bias is subtracted from real virtual memory addresses to index into
    /// debuginfo and the symbol table.
    bias: usize,
}

struct LibrarySegment {
    /// The stated address of this segment in the object file. This is not
    /// actually where the segment is loaded, but rather this address plus the
    /// containing library's `bias` is where to find it.
    stated_virtual_memory_address: usize,
    /// The size of ths segment in memory.
    len: usize,
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
    let mut call = |sym: Symbol| {
        // Extend the lifetime of `sym` to `'static` since we are unfortunately
        // required to here, but it's ony ever going out as a reference so no
        // reference to it should be persisted beyond this frame anyway.
        let sym = mem::transmute::<Symbol, Symbol<'static>>(sym);
        (cb)(&super::Symbol { inner: sym });
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
        let mut any_frames = false;
        if let Ok(mut frames) = cx.dwarf.find_frames(addr as u64) {
            while let Ok(Some(frame)) = frames.next() {
                any_frames = true;
                call(Symbol::Frame {
                    addr: addr as *mut c_void,
                    location: frame.location,
                    name: frame.function.map(|f| f.name.slice()),
                });
            }
        }

        if !any_frames {
            if let Some(name) = cx.object.search_symtab(addr as u64) {
                call(Symbol::Symtab {
                    addr: addr as *mut c_void,
                    name,
                });
            }
        }
    });
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
}

impl Symbol<'_> {
    pub fn name(&self) -> Option<SymbolName> {
        match self {
            Symbol::Frame { name, .. } => {
                let name = name.as_ref()?;
                Some(SymbolName::new(name))
            }
            Symbol::Symtab { name, .. } => Some(SymbolName::new(name)),
        }
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        match self {
            Symbol::Frame { addr, .. } => Some(*addr),
            Symbol::Symtab { .. } => None,
        }
    }

    pub fn filename_raw(&self) -> Option<BytesOrWideString> {
        match self {
            Symbol::Frame { location, .. } => {
                let file = location.as_ref()?.file?;
                Some(BytesOrWideString::Bytes(file.as_bytes()))
            }
            Symbol::Symtab { .. } => None,
        }
    }

    pub fn filename(&self) -> Option<&Path> {
        match self {
            Symbol::Frame { location, .. } => {
                let file = location.as_ref()?.file?;
                Some(Path::new(file))
            }
            Symbol::Symtab { .. } => None,
        }
    }

    pub fn lineno(&self) -> Option<u32> {
        match self {
            Symbol::Frame { location, .. } => location.as_ref()?.line,
            Symbol::Symtab { .. } => None,
        }
    }
}
