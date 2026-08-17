//! Lungo - A simple systray applet to prevent the system from going idle or suspending on demand (including on laptop lid closure).

use clap::Parser;
use log::error;
use std::process;
use tokio::runtime;

mod help;
mod inhibit;
mod lockfile;
mod tray;
mod version;

// Arguments definition
#[derive(Parser)]
#[command(disable_help_flag = true, disable_version_flag = true)]
struct Args {
    #[arg(short = 'h', long)]
    help: bool,

    #[arg(short = 'V', long)]
    version: bool,
}

fn main() {
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

    // Initialize logger
    env_logger::init();

    // Create (if needed) and acquire lockfile
    // Exit if there's already an instance running
    // or if there was an issue creating or acquiring the lockfile (e.g. permission issue)
    let _lockfile = lockfile::acquire_lockfile().unwrap_or_else(|error| {
        error!("{error:?}");
        process::exit(1);
    });

    // Create multi-threaded tokio runtime
    let tokio_runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|error| {
            error!("Failed to create Tokio runtime: {error}");
            process::exit(2);
        });

    // Start the systray applet
    tokio_runtime.block_on(tray::run()).unwrap_or_else(|error| {
        error!("{error:?}");
        process::exit(3);
    });
}
