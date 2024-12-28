// Target
// Release
// Build threads
// Config Files

// cargo xtask run target
// cargo xtask build target
// cargo xtask clean target

mod clean;
mod cli;
mod iso;
mod kbuild;
mod run;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use clean::clean;
use cli::{Target, Task};
use kbuild::{build_aarch64, build_riscv64, build_x86_64};
use run::{qemu_open_aarch64, qemu_open_riscv64, qemu_open_x86_64};

fn main() -> Result<()> {
    let task = Task::parse();

    match task {
        Task::Run { target } => match target {
            Target::X86_64 => {
                qemu_open_x86_64().context("Failed to open qemu_x86_64")?;
                Ok(())
            }
            Target::Aarch64 => {
                qemu_open_aarch64().context("Failed to open qemu_aarch64")?;
                Ok(())
            }
            Target::Riscv64 => {
                qemu_open_riscv64().context("Failed to open qemu_riscv64")?;
                Ok(())
            }
            Target::All => {
                //qemu_open_x86_64();
                //qemu_open_aarch64();
                //qemu_open_riscv64();
                panic!("This is an unsafe operation!");
                // Ok(())
            }
        },
        Task::Build { target } => match target {
            Target::X86_64 => {
                build_x86_64()?;
                Ok(())
            }
            Target::Aarch64 => {
                build_aarch64();
                Ok(())
            }
            Target::Riscv64 => {
                build_riscv64();
                Ok(())
            }
            Target::All => {
                build_x86_64()?;
                build_aarch64();
                build_riscv64();
                Ok(())
            }
        },
        Task::Clean { target } => match target {
            Target::X86_64 => {
                clean("x86_64")?;
                Ok(())
            }
            Target::Aarch64 => {
                clean("aarch64")?;
                Ok(())
            }
            Target::Riscv64 => {
                clean("riscv64")?;
                Ok(())
            }
            Target::All => {
                clean("x86_64")?;
                clean("aarch64")?;
                clean("riscv64")?;
                Ok(())
            }
        },
    }
}
