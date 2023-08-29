// assembly-output: emit-asm
// compile-flags: --target sparc64-unknown-openbsd
// needs-llvm-components: sparc

#![feature(no_core, lang_items)]
#![no_std]
#![no_core]
#![crate_type = "lib"]

#[lang = "sized"]
trait Sized {}

pub fn test() -> u8 {
    42
}

// CHECK: .section
