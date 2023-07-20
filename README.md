# towboot

a bootloader for Multiboot kernels (version 1 and version 2) on UEFI systems

## usage

towboot is a UEFI application. If you're developing an operating system or just
want to create a boot medium, there are several possible options for where to
place towboot and its configuration:

### removable media

This is the easiest one: It works for all architectures and requires no
configuration of the system.
Simply place the 32-bit build at `\EFI\bootia32.efi`, the 64-bit build at
`\EFI\bootx64.efi` and a configuration file at `\towboot.toml` on the ESP.

### installed system

Place an appropriate build at `\EFI\yourOS\towboot.efi` and the configuration
at `\EFI\yourOS\towboot.toml` on the ESP and add a boot option for
`\EFI\yourOS\towboot.efi -c \EFI\yourOS\towboot.toml`.

(You can also configure towboot just with command line arguments instead of
using a configuration file; see below.)

### chainloading from another bootloader

If you already have a bootloader capable of loading UEFI applications but
without support for Multiboot, you can add an entry like
`towboot.efi -kernel "mykernel.elf quiet" -module "initramfs.img initrd"`.

(You can use a configuration file instead of passing the information directly
on the command line; see above. Please note that towboot and its configuration
file currently have to be on the same partition.)

## development

If you want to compile towboot yourself, here are the instructions:

### dependencies

You'll need a nightly Rust compiler.
The version doesn't really matter,
though `rustc 1.73.0-nightly (39f42ad9e 2023-07-19)` definitely works.
If you don't know how to install one,
please take a look at [rustup.rs](https://rustup.rs/).
(You can configure rustup to use a nightly toolchain just for the current folder
by running `rustup override set nightly`.)

You'll also need the appropriate targets. If you're using rustup,
you can add them like this:

```sh
rustup target add i686-unknown-uefi
rustup target add x86_64-unknown-uefi
```

To build a disk image, you'll also need mtools and mkgpt.
The latter one is automatically being downloaded and compiled,
you'll need git, automake, make and a C compiler for that.

To boot the disk image in a virtual machine, QEMU is recommended.
You'll need OVMF for that, too. You can either install it via your distribution's
package manager or (for `i686`) let the build script download it.

### building

`cargo build` creates a `towboot.efi` file inside the `target` folder.
By default, this is a debug build for `i686-unknown-uefi`.
You can change this by appending `--release`
or by setting `--target x86_64_unknown_uefi` (for example).

Running `./build.sh` will do that and also create a disk image
and boot that with QEMU, so just may just want to run this.

You can configure whether to create a `debug` or `release` build for
either `i686` or `x86_64`, whether to enable KVM or wait for a GDB to attach
by setting the environment variables `BUILD`, `ARCH`, `KVM` or `GDB`.
(The defaults are `debug`, `i686`, `no` and `no`.)

This script expects the kernel in `../../kernels/multiboot1.elf`,
you can override this by setting `KERNEL`.

## documentation

This README file is relatively short (as you can see).
More documentation is available by running `cargo doc --open`.

## Known bugs / workarounds

The `hacks` modules contains workarounds for bugs or missing features in
the compiler.

# Quirks

You can override some specifics of how the kernel is loaded at runtime by
adding quirks. They can be configured either in the `quirk` key of a kernel
entry (if the kernel is loaded via a configuration file) or via the `-quirk`
command line option (if the kernel is loaded via `-kernel`).

Available quirks are:

* `DontExitBootServices`: do not exit Boot Services
        This starts the kernel with more privileges and less available memory.
        In some cases this might also display more helpful error messages.
* `ForceElf`: always treat the kernel as an ELF file
* `ForceOverwrite`: ignore the memory map when loading the kernel
        (This might damage your hardware!)
* `KeepResolution`: ignore the kernel's preferred resolution
* `ModulesBelow200Mb`: keep allocations for modules below 200 MB
