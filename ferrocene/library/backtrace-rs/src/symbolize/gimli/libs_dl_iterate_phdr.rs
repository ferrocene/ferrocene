// Other Unix (e.g. Linux) platforms use ELF as an object file format
// and typically implement an API called `dl_iterate_phdr` to load
// native libraries.

use super::mystd::borrow::ToOwned;
use super::mystd::env;
use super::mystd::ffi::{CStr, OsStr};
use super::mystd::os::unix::prelude::*;
use super::{Library, LibrarySegment, OsString, Vec};
use core::slice;

pub(super) fn native_libraries() -> Vec<Library> {
    let mut ret = Vec::new();
    unsafe {
        libc::dl_iterate_phdr(Some(callback), core::ptr::addr_of_mut!(ret).cast());
    }
    return ret;
}

fn infer_current_exe(base_addr: usize) -> OsString {
    cfg_if::cfg_if! {
        if #[cfg(not(target_os = "hurd"))] {
                if let Ok(entries) = super::parse_running_mmaps::parse_maps() {
                let opt_path = entries
                    .iter()
                    .find(|e| e.ip_matches(base_addr) && e.pathname().len() > 0)
                    .map(|e| e.pathname())
                    .cloned();
                if let Some(path) = opt_path {
                    return path;
                }
            }
        }
    }
    env::current_exe().map(|e| e.into()).unwrap_or_default()
}

/// # Safety
/// `info` must be a valid pointer.
/// `vec` must be a valid pointer to `Vec<Library>`
#[forbid(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn callback(
    info: *mut libc::dl_phdr_info,
    _size: libc::size_t,
    vec: *mut libc::c_void,
) -> libc::c_int {
    // SAFETY: We are guaranteed these fields:
    let dlpi_addr = unsafe { (*info).dlpi_addr };
    let dlpi_name = unsafe { (*info).dlpi_name };
    let dlpi_phdr = unsafe { (*info).dlpi_phdr };
    let dlpi_phnum = unsafe { (*info).dlpi_phnum };
    // SAFETY: We assured this.
    let libs = unsafe { &mut *vec.cast::<Vec<Library>>() };
    // most implementations give us the main program first
    let is_main = libs.is_empty();
    // we may be statically linked, which means we are main and mostly one big blob of code
    let is_static = dlpi_addr == 0;
    // sometimes we get a null or 0-len CStr, based on libc's whims, but these mean the same thing
    let no_given_name = dlpi_name.is_null()
        // SAFETY: we just checked for null
        || unsafe { *dlpi_name == 0 };
    let name = if is_static {
        // don't try to look up our name from /proc/self/maps, it'll get silly
        env::current_exe().unwrap_or_default().into_os_string()
    } else if is_main && no_given_name {
        infer_current_exe(dlpi_addr as usize)
    } else {
        // this fallback works even if we are main, because some platforms give the name anyways
        if dlpi_name.is_null() {
            OsString::new()
        } else {
            // SAFETY: we just checked for nullness
            OsStr::from_bytes(unsafe { CStr::from_ptr(dlpi_name) }.to_bytes()).to_owned()
        }
    };
    let headers = if dlpi_phdr.is_null() || dlpi_phnum == 0 {
        &[]
    } else {
        // SAFETY: We just checked for nullness or 0-len slices
        unsafe { slice::from_raw_parts(dlpi_phdr, dlpi_phnum as usize) }
    };
    libs.push(Library {
        name,
        segments: headers
            .iter()
            .map(|header| LibrarySegment {
                len: (*header).p_memsz as usize,
                stated_virtual_memory_address: (*header).p_vaddr as usize,
            })
            .collect(),
        bias: dlpi_addr as usize,
    });
    0
}
