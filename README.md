[![License](https://img.shields.io/badge/License-zlib/libpng-pink)
](./LICENSE)

# 🪐 Nebula OS
A simple OS written in Rust, for learning and entertainment.

# 📖 Table of Contents
- [🚀 Compiling](#compiling)
- [💾 Emulating](#emulating)
- [💻 Installing](#installing)
- [📦 Project Structure](#project-structure)

# 🚀 Compiling <a name="compiling"></a>
To compile the source, you must have [`rust`](https://www.rust-lang.org/)
installed.
If you do not want to compile the source, please look at the releases.

To allow compilation, you must use a nightly rust compiler with the
`llvm-tools-preview` component and the `x86_64-unknown-none` target.
To do this, execute the following commands in the project's directory:
```
$ rustup override set nightly
$ rustup component add llvm-tools-preview
$ rustup target add x86_64-unknown-none
```

Now, to compile the source, run:
```
$ cargo build
```
This will generate a bootable disk image.

# 💾 Emulating <a name="emulating"></a>
```
$ cargo run
```
will emulate the OS in `qemu-system-x86_64`, if installed.

A way to change emulator might be added in the future.

# 💻 Installing <a name="installing"></a>
Currently, the only way to install the OS is to brutally paste the disk image
into a disk.

An installer may be added in the future.

# 📦 Project Structure <a name="project-structre"></a>
The OS boots using
[`bootloader`](https://github.com/rust-osdev/bootloader).
The kernel source is in the [`kernel`](./kernel) directory.
