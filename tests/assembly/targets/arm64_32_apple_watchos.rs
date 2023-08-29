// assembly-output: emit-asm
// compile-flags: --target arm64_32-apple-watchos
// needs-llvm-components: aarch64

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
