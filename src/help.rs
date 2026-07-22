//! Print help message

pub fn show_help() {
    println!(
        "Lungo - A simple systray applet to prevent the system from going idle or suspending on demand (including on laptop lid closure)."
    );
    println!();
    println!("Run `lungo` to start the systray applet.");
    println!();
    println!("Simply click on the systray applet to toggle it on and off.");
    println!(
        "The icon automatically changes to reflect the current state (\"empty cup\" icon if off, \"hot cup\" icon if on)."
    );
    println!("The state can also be consulted and toggled from the systray applet menu.");
    println!();
    println!("Options:");
    println!("  -h, --help     Display this message");
    println!("  -V, --version  Display version information");
    println!();
    println!("For more information, see the lungo(1) man page.");
}
