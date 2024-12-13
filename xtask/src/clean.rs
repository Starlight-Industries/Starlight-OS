use anyhow::{Context, Result};
use std::fs;
use std::process::Command;
pub fn clean(target: &str) -> Result<()> {
    clean_files(target)?;
    clean_kernel()?;

    Ok(())
}

fn clean_files(target: &str) -> Result<()> {
    //ineptos-x86_64

    let image_name = format!("ineptos-{}", { target });
    let iso = &format!("{}.iso", image_name);
    let hdd = &format!("{}.hdd", image_name);
    println!("removing iso root");
    fs::remove_dir("iso_root")?;
    println!("removing iso");
    fs::remove_dir(iso)?;
    println!("removing directory");
    fs::remove_dir(hdd)?;

    Ok(())
}

fn clean_kernel() -> Result<()> {
    //cargo clean
    //rm -rf kernel

    let _ = Command::new("cargo")
        .arg("clean")
        .output()
        .context("Failed to run cargo clean")?;

    fs::remove_dir_all("kernel").context("Failed to clean kernel directory")?;

    Ok(())
}
