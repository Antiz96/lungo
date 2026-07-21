//! Print help message

pub fn show_help() {
    println!(
        "Lungo - A simple systray applet to prevent your system from going idle or sleeping."
    );
    println!();
    println!("Run the `lungo` command to start the systray applet.");
    println!();
    println!("Options:");
    println!("  -h, --help     Display this message");
    println!("  -V, --version  Display version information");
    println!();
    println!("For more information, see the lungo(1) man page.");
}
