use std::process::Command;

use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;

use crate::iso::{iso_create_aarch64, iso_create_riscv64, iso_create_x86_64, ovmf_setup};

pub fn qemu_open_x86_64() -> Result<()> {
    /*
    qemu-system-x86_64 \
    -M q35 \
    -drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on \
    -drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd \
    -cdrom starlightOS-x86_64.iso \
    -m 2G
     */

    ovmf_setup(&"x86_64").context("Failed to set up OVMF")?;
    iso_create_x86_64().context("Failed to create x86_64 ISO")?;

    Command::new("qemu-system-x86_64")
        .arg("-M")
        .arg("q35")
        .arg("-drive")
        .arg("if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-x86_64.fd,readonly=on")
        .arg("-drive")
        .arg("if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-x86_64.fd")
        .arg("-cdrom")
        .arg("starlightOS-x86_64.iso")
        .arg("-m")
        .arg("2G")
        .status()
        .context("Failed to execute QEMU")?;
    Ok(())
}

pub fn qemu_open_aarch64() -> Result<()> {
    ovmf_setup(&"aarch64").context("test")?;
    iso_create_aarch64().context("Failed to create aarch64 ISO")?;
    unimplemented!();
}

pub fn qemu_open_riscv64() -> Result<()> {
    ovmf_setup(&"riscv64").context("sjdf")?;
    iso_create_riscv64().context("Failed to create riscV ISO")?;
    unimplemented!();
}
