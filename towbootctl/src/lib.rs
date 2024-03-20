//! This crate offers functionality to use towboot for your own operating system.
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use log::info;
use tempfile::NamedTempFile;

use towboot_config::Config;

mod bochs;
pub mod config;
pub mod firmware;
mod image;
pub use bochs::bochsrc;
pub use image::Image;

/// How big the image should be
pub const DEFAULT_IMAGE_SIZE: u64 = 50*1024*1024;

/// Where to place the 32-bit EFI file
pub const IA32_BOOT_PATH: &str = "EFI/Boot/bootia32.efi";

/// Where to place the 64-bit EFI file
pub const X64_BOOT_PATH: &str = "EFI/Boot/bootx64.efi";

/// Write the given configuration file to image.
/// This also copies all files that are referenced in it.
fn add_config_to_image(image: &mut Image, config: &mut Config) -> Result<()> {
    let mut config_path = PathBuf::from(config.src.clone());
    config_path.pop();
    // go through all needed files; including them (but without the original path)
    for src_file in config.needed_files() {
        let src_path = config_path.join(PathBuf::from(&src_file));
        let dst_file = src_path.file_name().unwrap();
        let dst_path = PathBuf::from(&dst_file);
        src_file.clear();
        src_file.push_str(dst_file.to_str().unwrap());
        image.add_file(&src_path, &dst_path)?;
    }

    // write the configuration itself to the image
    let mut config_file = NamedTempFile::new()?;
    config_file.as_file_mut().write_all(
        toml::to_string(&config)?.as_bytes()
    )?;
    image.add_file(&config_file.into_temp_path(), &PathBuf::from("towboot.toml"))?;
    Ok(())
}

/// Joins a slice of strings.
pub fn runtime_args_to_load_options(runtime_args: &[String]) -> String {
    let mut load_options = "towboot.efi".to_owned();
    for string in runtime_args.iter() {
        load_options.push(' ');
        if string.contains(' ') {
            load_options.push('"');
        }
        load_options.push_str(string);
        if string.contains(' ') {
            load_options.push('"');
        }
    }
    load_options
}

/// Create an image, containing a configuration file, kernels, modules and towboot.
pub fn create_image(
    target: &Path, runtime_args: &[String], i686: Option<&Path>, x86_64: Option<&Path>,
) -> Result<Image> {
    info!("creating image at {}", target.display());
    let mut image = Image::new(target, DEFAULT_IMAGE_SIZE)?;

    // generate a configuration file from the load options
    let load_options = runtime_args_to_load_options(runtime_args);
    if let Some(mut config) = config::get(&load_options)? {
        add_config_to_image(&mut image, &mut config)?;
    }

    // add towboot itself
    if let Some(src) = i686 { 
        image.add_file(src, &PathBuf::from(IA32_BOOT_PATH))?;
    }
    if let Some(src) = x86_64 { 
        image.add_file(src, &PathBuf::from(X64_BOOT_PATH))?;
    }

    Ok(image)
}

