use std::{
    env,
    error::Error,
    fs::File,
    io::Write,
    path::PathBuf,
};
use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET")?;

    has_fpu(&target);

    let out = PathBuf::from(
        env::var_os("OUT_DIR")
            .unwrap()
    );

    println!("target: {}", target);
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");

    File::create(out.join("link.x"))?
        .write_all(include_bytes!("scripts/linker/link.x.in"))?;
    println!("cargo:rerun-if-changed=link.x.in");

    Build::new().file("asm/hard_fault_trampoline.s")
        .compile("hard_fault_trampoline");
    println!("cargo:rerun-if-changed=asm/hard_fault_trampoline.s");

    Build::new().file("asm/irq_handler_trampoline.s")
        .compile("irq_handler_trampoline");
    println!("cargo:rerun-if-changed=asm/irq_handler_trampoline.s");

    Build::new().file("asm/primask_read.s")
        .compile("primask");
    println!("cargo:rerun-if-changed=asm/primask_read.s");

    Build::new().file("asm/cpsid.s")
        .compile("cpsid");
    println!("cargo:rerun-if-changed=asm/cpsid.s");

    Build::new().file("asm/cpsie.s")
        .compile("cpsie");
    println!("cargo:rerun-if-changed=asm/cpsie.s");

    Build::new().file("asm/msp_r.s")
        .compile("msp_r");
    println!("cargo:rerun-if-changed=asm/msp_r.s");

    Build::new().file("asm/msp_w.s")
        .compile("msp_w");
    println!("cargo:rerun-if-changed=asm/msp_w.s");

    Ok(())
}

fn has_fpu(target: &str) {
    if target.ends_with("eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}
