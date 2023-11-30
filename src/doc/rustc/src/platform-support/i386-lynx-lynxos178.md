# `i386-lynx-lynxos178`

**Tier: 3**

[Lynx] [LynxOS-178] POSIX Real-time operating system.

This support has been implemented by [Ferrous Systems] as part of the Ferrocene
Compiler.

Note that *LynxOS-178* is a different RTOS to *LynxOS*. There is no support for
*LynxOS* in this target, only *LynxOS-178*.

[Lynx]: https://www.lynx.com/
[LynxOS-178]: https://www.lynx.com/products/lynxos-178-do-178c-certified-posix-rtos
[Ferrous Systems]: https://www.ferrous-systems.com

## Target maintainers

- Jonathan Pallant `jonathan.pallant@ferrous-systems.com`, https://ferrous-systems.com

## Requirements

This target is cross-compiled. There is no support for `std`. There is no
default allocator, but it's possible to use `alloc` by supplying an allocator.

This target generates binaries in the ELF format. Any alternate formats or
special considerations for binary layout will require linker options or linker
scripts. The target requires you to have the `i386-lynx-lynxos178-gcc-7.1.0`
linker from [Lynx mosa-ic] 2022.07 or similar.

[Lynx mosa-ic]: https://www.lynx.com/products/lynx-mosaic-modular-development-framework

## Building Rust programs

Precompiled arfefacts are only supplied with the Ferrocene compiler. Only
`libcore` is available, not `libstd`.

```text
# install cross-compile toolchain
criticalup target add i386-lynx-lynxos178
# target flag may be used with any cargo or rustc command
cargo build --target i386-lynx-lynxos178
```

### Small example application

```rust,ignore (platform-specific)
#![no_std]
#![no_main]

use core::fmt::Write;

/// Represents our standard output stream.
pub struct STDOUT;

impl core::fmt::Write for &STDOUT {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            write(1, s.as_ptr(), s.len());
        }
        Ok(())
    }
}

#[link(name = "c")]
extern "C" {
    pub fn write(fd: i32, ptr: *const u8, len: usize) -> i32;
    pub fn _exit(code: i32);
}

#[no_mangle]
pub extern "C" fn main() -> isize {
    writeln!(&STDOUT, "Hello, LynxOS-178").unwrap();
    0
}

/// The default panic handler - prints the panic to stdout.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = writeln!(&STDOUT, "Panic!: {:#?}", info);
    unsafe {
        _exit(1);
    }
    loop {}
}
```

This example will need to be linked against both `lib/libc.a` and
`lib/libpthread.a`. You can use a `build.rs` file to do this:

```rust,no_run
fn main() {
    let bsp_path = std::env::var("BSP_ROOT").expect("Set BSP_ROOT");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=pthread");
    println!("cargo:rustc-link-search=native={}/lib", bsp_path);
}
```

## Testing

As `i386-lynx-lynxos178` does not support `std`, this target does not support
running the Rust test suite.
