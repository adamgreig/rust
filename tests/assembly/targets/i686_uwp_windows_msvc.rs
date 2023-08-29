// assembly-output: emit-asm
// compile-flags: --target i686-uwp-windows-msvc
// needs-llvm-components: x86

#![feature(no_core, lang_items)]
#![no_std]
#![no_core]
#![crate_type = "lib"]

#[lang = "sized"]
trait Sized {}

pub fn test() -> u8 {
    42
}

// CHECK: .text
