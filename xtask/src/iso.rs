use std::fs;
use std::process::Command;

use anyhow::{Context, Result};

use crate::kbuild::{build_aarch64, build_limine, build_riscv64, build_x86_64};

pub fn iso_create_x86_64() -> Result<()> {
    build_limine().context("Failed to build limine")?;
    build_x86_64().context("Failed to build x86_64 ISO")?;

    iso_root_setup().context("Failed to setup iso_root")?;
    create_iso_with_bootloader_x86_64().context("Failed to build ISO with x86_64 bootloader")?;
    iso_root_remove().context("Failed to remove iso root")?;
    Ok(())
}

pub fn iso_create_aarch64() -> Result<()> {
    build_limine().context("Failed to build limine")?;
    build_aarch64();

    iso_root_setup().context("Failed to setup iso_root")?;
    create_iso_with_bootloader_aarch64();
    iso_root_remove().context("Failed to remove iso root")?;
    Ok(())
}

pub fn iso_create_riscv64() -> Result<()> {
    build_limine().context("Failed to build limine")?;
    build_riscv64();

    iso_root_setup().context("Failed to set up iso_root")?;
    create_iso_with_bootloader_riscv64();
    iso_root_remove().context("Failed to build riscv64 ISO")?;
    Ok(())
}

fn iso_root_setup() -> Result<()> {
    if fs::metadata("iso_root").is_ok() {
        Command::new("rm")
            .arg("-rf")
            .arg("iso_root")
            .output()
            .context("Failed to remove iso_root directory")?;
    }

    fs::create_dir_all("iso_root/boot").context("Failed to create iso_root/boot directory")?;
    fs::copy("kernel/kernel", "iso_root/boot/kernel")
        .context("Failed to copy kernel/kernel binary into iso_root/boot/kernel")?;
    fs::create_dir_all("iso_root/boot/limine")
        .context("Failed to create iso_root/boot/limine directory")?;
    fs::copy("limine.conf", "iso_root/boot/limine/limine.conf")
        .context("Failed to copy limine.conf to iso_root/boot/limine/limine.conf")?;
    fs::create_dir_all("iso_root/EFI/BOOT").context("Failed to create iso_root/EFI/BOOT")?;

    Ok(())
}

fn iso_root_remove() -> Result<()> {
    match fs::metadata("iso_root") {
        Ok(_) => {
            fs::remove_dir_all("iso_root")?;
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(format!("Error: {e}"))),
    }
}

fn create_iso_with_bootloader_x86_64() -> Result<()> {
    fs::copy(
        "limine/limine-bios.sys",
        "iso_root/boot/limine/limine-bios-cd.bin",
    )
    .context("Failed to copy limine/limine-bios.sys into iso_root")?;

    fs::copy(
        "limine/limine-uefi-cd.bin",
        "iso_root/boot/limine/limine-uefi-cd.bin",
    )
    .context("Failed to copy limine-uefi-cd into iso_root")?;

    fs::copy("limine/BOOTX64.EFI", "iso_root/EFI/BOOT/BOOTX64.EFI")
        .context("Failed to copy limine BOOTX64.EFI")?;
    fs::copy("limine/BOOTIA32.EFI", "iso_root/EFI/BOOT/BOOTIA32.EFI")
        .context("Failed to copy limine BOOTIA32.EFI")?;

    let status = Command::new("xorriso")
        .args(&[
            "-as",
            "mkisofs",
            "-b",
            "boot/limine/limine-bios-cd.bin",
            "-no-emul-boot",
            "-boot-load-size",
            "4",
            "-boot-info-table",
            "--efi-boot",
            "boot/limine/limine-uefi-cd.bin",
            "-efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
            "iso_root",
            "-o",
            &format!("{}.iso", "ineptos-x86_64"),
        ])
        .status()
        .context("failed to pack ISO with xorriso")?;

    if !status.success() {
        println!("xorriso failed to create ISO image");
        std::process::exit(1)
    }

    Command::new("./limine/limine")
        .args(&["bios-install", &format!("{}.iso", "ineptos-x86_64")])
        .status()
        .context("Failed to run limine bios-install")?;
    iso_root_remove().context("Failed to remove iso_root")?;

    Ok(())
}

fn create_iso_with_bootloader_aarch64() {
    unimplemented!();
}

fn create_iso_with_bootloader_riscv64() {
    unimplemented!();
}

pub fn ovmf_setup(target: &str) -> Result<()> {
    //mkdir -p ovmf
    fs::create_dir_all("ovmf").context("Failed to create ovmf directory")?;
    ovmf_code_download(target)?;
    ovmf_vars_download(target)?;

    Ok(())
}

fn ovmf_code_download(target: &str) -> Result<()> {
    //curl -Lo ovmf/ovmf-vars-x86_64.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-code-x86_64.fd
    let ovmf_file = format!("ovmf-code-{}.fd", target);
    let ovmf_file_path = format!("ovmf/{}", ovmf_file);

    let url = format!(
        "https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/{}",
        ovmf_file
    );

    Command::new("curl")
        .arg("-Lo")
        .arg(ovmf_file_path)
        .arg(url)
        .output()
        .context("Failed to run curl")?;
    Ok(())
}

fn ovmf_vars_download(target: &str) -> Result<()> {
    //curl -Lo ovmf/ovmf-code-x86_64.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-code-x86_64.fd
    let ovmf_file = format!("ovmf-vars-{}.fd", target);
    let ovmf_file_path = format!("ovmf/{}", ovmf_file);
    let url = format!(
        "https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/{}",
        ovmf_file
    );

    Command::new("curl")
        .arg("-Lo")
        .arg(ovmf_file_path)
        .arg(url)
        .output()
        .context("Failed to run curl to obtain ovmf")?;

    Ok(())
}
