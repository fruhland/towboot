use std::process;

use cli_xtask::clap;
use cli_xtask::config::Config;
use cli_xtask::tracing::info;
use cli_xtask::{Result, Run, Xtask};

fn main() -> Result<()> {
    Xtask::<Command>::main()
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Build a bootable image, containing towboot, a kernel and modules
    Build {
        #[arg( long )]
        release: bool,
        #[arg( long )]
        no_i686: bool,
        #[arg( long )]
        no_x86_64: bool,
        #[arg( long, default_value = "towboot.toml" )]
        config: String,
        #[arg( long, default_value = "disk.img" )]
        target: String,
    },
    Run,
}
impl Command {
    fn build(&self, release: &bool, no_i686: &bool, no_x86_64: &bool, config: &str, target: &str) -> Result<()> {
        let mut cargo_command = process::Command::new("cargo");
        let mut build_command = cargo_command.arg("build");
        if *release {
            build_command = cargo_command.arg("--release");
        }
        if !no_i686 {
            info!("building for i686, pass --no-i686 to skip this");
            build_command
                .arg("--target")
                .arg("i686-unknown-uefi")
                .spawn()?.wait()?;
        }
        if !no_x86_64 {
            info!("building for x86_64, pass --no-x86-64 to skip this");
            build_command
                .arg("--target")
                .arg("x86_64-unknown-uefi")
                .spawn()?.wait()?;
        }
        Ok(())
    }
}

impl Run for Command {
    fn run(&self, _config: &Config) -> Result<()> {
        match self {
            Self::Build {
                release, no_i686, no_x86_64, config, target,
            } => self.build(release, no_i686, no_x86_64, config, target),
            Self::Run => { println!("run!"); Ok(()) },
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
