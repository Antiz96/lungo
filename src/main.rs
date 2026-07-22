//! Lungo - A simple systray applet to prevent the system from going idle or suspending on demand (including on laptop lid closure).

use clap::Parser;
use log::error;
use std::io::ErrorKind;
use std::process;

mod help;
mod inhibit;
mod lockfile;
mod tray;
mod version;

// Argument parser
#[derive(Parser)]
#[command(disable_help_flag = true, disable_version_flag = true)]
struct Args {
    // Options / flags
    #[arg(short = 'h', long)]
    help: bool,

    #[arg(short = 'V', long)]
    version: bool,
}

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // Parse arguments
    let args = Args::parse();

    // Show help message if the -h / --help arg is passed
    if args.help {
        help::show_help();
        return;
    }

    // Show name and version if the -V / --version arg is passed
    if args.version {
        version::show_version();
        return;
    }

    // Create (if needed) and acquire lockfile
    // Exit if there's already an instance running
    // or if there was an issue creating or acquiring the lockfile (e.g. permission issue)
    let _lock = lockfile::acquire_lockfile().unwrap_or_else(|error| {
        if error.kind() == ErrorKind::AlreadyExists {
            error!("Another instance of lungo is already running");
        } else {
            error!("Failed to acquire lockfile: {error}");
        }

        process::exit(1);
    });

    // Start systray applet
    tray::run().await;
}
