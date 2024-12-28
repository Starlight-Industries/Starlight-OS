<div align="center">

# Starlight OS ✨

### An open-source, cross-platform, hobby Operating System

[Installation](#usage) • [Docs](#documentation) • [Roadmap](#roadmap) • [Community](#community)


[![Discord](https://img.shields.io/discord/1258146131372806217)](https://discord.gg/kv3jKuPW9F)

</div>

---

## What is StarlightOS?

StarlightOS is a hobby Operating System built on top of the [Comet package manager](https://github.com/Starlight-Industries/Comet). Designed to be lightweight, fast, and sensible by default.
## Why?

StarlightOS aims to provide:

* Easy to use, yet powerful package management.
* Sane defaults for a wide range of use cases.
* A modern, intuitive developer experience.
* A focus on clever architecture rather than flashy features.
## Quick Start

### Installation

> [!IMPORTANT]
> StarlightOS (and by extension Comet) is still in early development, and is not yet ready for use in most environments.

#### Method 1. Build from source


0. Install the rust toolchain 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. Clone the repository
```bash
git clone https://github.com/Starlight-Industries/Starlight-OS.git
```
2. Navigate to the project directory
```bash
cd Starlight-OS
```
3. Install QEMU

**Ubuntu/Debian**
```bash
sudo apt-get install qemu-system
```
**Fedora/Rhel**
```bash
sudo dnf install qemu
```
**Arch**
```bash
sudo pacman -S qemu-full
```

5. Install xorriso
**Ubuntu/Debian**
```bash
sudo apt install xorriso
```
**Fedora/Rhel**
```bash
sudo dnf install xorriso
```
**Arch**
```bash
sudo pacman -S xorriso
```

4. Build the project
```bash
cargo xtask run x86_64
```

> [!NOTE]
> If you are on windows, you will need to install WSL2 and run the command in the WSL2 environment.
#### Method 2. Test using Prebuilt ISO
> [!NOTE]
> For this example, we will be using VirtualBox on Windows.

As a prerequisite, you will need to install VirtualBox on your machine and download the ISO from the [releases](https://github.com/Starlight-Industries/Starlight-OS/releases) page.

1. Open VirtualBox and create a new virtual machine.

2. Select the ISO file you downloaded earlier.

3. For VM Type, select "Other" and for Version, select "Other/Unknown (64-bit)".

4. For Memory Size, select 1024 MB.

5. Click finish.

6. Open the virtual machines settings and click on the "System" tab.

7. Under Motherboard, locate the "Extended Features" section and enable the "Enable EFI (special OSes only)" option.

8. Click OK.

9. Your done! You can now boot into StarlightOS.

> ![NOTE]
> If you are using a different virtualization software, please refer to their documentation for instructions on how to run unknown operating systems.
> If you would like to update this documentation, please submit a pull request. or reach out to us!

### Usage

<div align="center">

**[WIP]** Check back later. StarlightOS currently lacks an input driver. Or you could [Contribute!]() 📚

</div>

## Roadmap

<div align="center">

**Current Status**: Project planing ❓

</div>

### 🌐 Platform Support
⭐ - The most support is avalible for this platform.

🔥 - This platform is currently supported with high priority.

⚠️ - The platform is planned to be supported in the future but is not currently supported for external reasons. (eg. MacOS requiring developer accounts, needding apple hardware to run outside of a CI/CD environment,etc)

📉 - This platform is niche or uncommon, its priority is low but still on our radar

| Platform | Status          | Priority      |
| :--------: | :-------------: |:----------: |
| x86_64/AMD64   | ✅ Supported   | ⭐ Main       |
| Aarch64  | 🔄 In progress | 🔥 High       |
| RiscV    | 🔎 Planned     | 📉 Low       |

> [!NOTE]
> We are always looking for new contributors to help us achieve our goals, so if you're interested and posess a targeted device, please reach out to us on [Discord](https://discord.com/invite/xJX4GXvbME) (Eg. MacOS 🍎).

## Community/Support
Here are the following links to all of our socials, if you discover a different account on any platform not listed here claiming to be affilated they are NOT affilated with the project, starlight-industries, or any of our related projects, products, or services.
<div align="center">

[![Discord Banner](https://img.shields.io/discord/1258146131372806217?style=for-the-badge&logo=discord)](https://discord.gg/xJX4GXvbME)

</div>

## FAQ


</details>

<details>
<summary>What will StalightOS do differently?</summary>
StarlightOS is a hobby Operating made to compliment the 
<a href="https://github.com/Starlight-Industries/Starlight-OS/issues/1">
Comet package manager
</a>
Because of this, It is not inherently bound to any specific standard or specifications (such as POSIX). By extension, it is not bound by any specific kernel architecture and is designed to be flexible, chohesive, extensible, User, and most importantly of all, **Secure**.
</details>

<div align="center">

## Contributing

Thank you for considering contributing to StarlightOS! We appreciate your interest in helping us improve the project. Your contributions are valuable and help us make StarlightOS even better. As of now, we are still in the early stages of development, and we are planning the roadmap for the future. untill then, we are not accepting any major contributions.
<a href="https://github.com/Starlight-Industries/Comet/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Starlight-Industries/Comet" />
</a>

For more information on how to contribute, please refer to the [Contributing Guide](CONTRIBUTING.md).

---

Made with ❤️ by Starlight-industries & the open source community
  <br>
  <sub>🌟 Star us on GitHub | 📢 Share with friends | 🤝 Join the community!</sub>
  </div>
</div>
