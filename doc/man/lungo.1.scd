lungo(1) ["Lungo 0.0.1" ["Lungo Manual"]]

# NAME

Lungo - A simple systray applet to prevent your system from going idle or sleeping on demand.

# DESCRIPTION

Prevent your machine from going idle or sleeping on demand, simply by cliking the systray applet to toggle it on and off.
The icon automatically changes to reflect the current state ("empty cup" icon if off, "hot cup" icon if on).

Lungo is:

- Simple and minimal by design: No superfluous options and features, no complex background operations.
- Distribution and desktop environment agnostic: Only requires either `systemd-logind` or `elogind`, not tied to any specific graphical / desktop environment.
- Crash and memory safe: Uses a `logind` file descriptor lock (which is automatically released by the kernel on crash or kill), written in a memory safe language.

Run `lungo` to start the systray applet.

# SYNOPSIS

*lungo* [OPTION]

# OPTIONS

*-V, --version*
	Display version information.

*-h, --help*
	Display the help message.

# BUGS

Please report bugs to the GitHub page: https://github.com/Antiz96/lungo

# AUTHOR

Robin Candau <robincandau@protonmail.com>
