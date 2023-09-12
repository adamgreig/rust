use crate::{
    abi::Endian,
    spec::{base::linux_musl, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions},
};

pub fn target() -> Target {
    let mut base = linux_musl::opts();
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m32"]);
    base.max_atomic_width = Some(32);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: "powerpc-unknown-linux-musl".into(),
        pointer_width: 32,
        data_layout: "E-m:e-p:32:32-Fn32-i64:64-n32".into(),
        arch: "powerpc".into(),
        options: TargetOptions { endian: Endian::Big, mcount: "_mcount".into(), ..base },
    }
}
