lungo(1) ["Lungo 1.0.3" ["Lungo Manual"]]

# NAME

Lungo - A simple systray applet to prevent the system from going idle or suspending on demand (including on laptop lid closure).

# DESCRIPTION

Run `lungo` to start the systray applet.

Simply click on the systray applet to toggle it on and off.

The systray icon automatically changes to reflect the current state ("moon cup" when disabled, "power cup" when enabled).

The state can also be consulted and toggled from the systray applet menu.

# SYNOPSIS

*lungo* [OPTION]

# OPTIONS

*-V, --version*
	Display version information.

*-h, --help*
	Display the help message.

# EXIT STATUS

*0*
	Ok.

*1*
	Failed to create or acquire the lockfile.

*2*
	Failed to create the tokio runtime.

*3*
	Failed to start the systray applet.

# BUGS

Please report bugs to the GitHub page: https://github.com/Antiz96/lungo

# AUTHOR

Robin Candau <robincandau@protonmail.com>
