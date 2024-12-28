use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

pub fn build_x86_64() -> Result<()> {
    //RUSTFLAGS="-C relocation-model=static"
    //cargo build -j12 --manifest-path kernel/Cargo.toml --target=x86_64-unknown-none
    //cp kernel/target/x86_64-unknown-none/debug/./ineptOS kernel/kernel

    env::set_var("RUSTFLAGS", "-C relocation-model=static");

    Command::new("cargo")
        .arg("build")
        .arg("-j12")
        .arg("--manifest-path")
        .arg("kernel/Cargo.toml")
        .arg("--target=x86_64-unknown-none")
        .output()
        .context("Failed to build kernel")?;
    let project_root = project_root::get_project_root()?;
    println!("Project root: {}", project_root.display());
    let ineptos_binary_path = project_root.join("target/x86_64-unknown-none/debug/ineptOS");
    println!("binary path: {}", ineptos_binary_path.display());
    fs::create_dir_all("kernel").expect("Failed to create kernel directory");
    // println!("{}", ineptos_binary_path.canonicalize().unwrap().display());
    fs::copy(&ineptos_binary_path, &PathBuf::from("kernel/kernel"))
        .context("Failed to find kernel binary")?;

    Ok(())
}

pub fn build_aarch64() {
    unimplemented!()
}

pub fn build_riscv64() {
    unimplemented!()
}

pub fn build_limine() -> Result<()> {
    Command::new("make")
        .arg("-C")
        .arg("limine")
        .output()
        .context("Failed to build limine")?;
    Ok(())
}
