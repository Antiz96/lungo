# Lungo

## Table of contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Description

A simple systray applet to prevent the system from going idle or suspending on demand (including on laptop lid closure).

Run `lungo` to start the systray applet.

Simply click on the systray applet to toggle it on and off.  
The systray icon automatically changes to reflect the current state ("empty cup" icon if off, "hot cup" icon if on).  
The state can also be consulted and toggled from the systray applet menu.

Lungo is:

- Simple and minimal by design: No superfluous options and features, no complex background operations.
- Distribution and desktop environment agnostic: Only requires either `systemd-logind` or `elogind`, not tied to any specific graphical / desktop environment.
- Crash and memory safe: Uses a `logind` file descriptor lock (which is automatically released by the kernel on crash or kill), written in a memory safe language.

**Motivation:**

This project is generally inspired by [caffeine](https://launchpad.net/caffeine) / [caffeine-ng](https://codeberg.org/WhyNotHugo/caffeine-ng) (hence the [lungo](https://en.wikipedia.org/wiki/Lungo) name, as a reference) with some parts of the implementation inspired by [caffeine-applet](https://github.com/codevardhan/caffeine-applet).

The main motivation was to make a simple and minimal implementation of a "caffeine" systray applet, with no arguably complex or "superfluous" mechanism / features coupled to numerous runtime dependencies (unlike `caffeine` / `caffeine-ng`), **and** that isn't tied to a specific desktop / graphical environment (unlike `caffeine-applet` or the [caffeine gnome shell extension](https://github.com/eonpatapon/gnome-shell-extension-caffeine)).

## Installation

### Runtime dependencies

`systemd-logind` or `elogind` up and running.

### Packages

[![Packaging status](https://repology.org/badge/vertical-allrepos/lungo.svg)](https://repology.org/project/lungo/versions)

### Pre-compiled binary

A (statically linked) pre-compiled binary for the `x86_64 (amd64)` architecture is distributed as a [release asset](https://github.com/Antiz96/lungo/releases/latest) (`lungo-<release_version>-x86_64`).

The pre-compiled binary can be reproduced from source (in the sense of [reproducible builds](https://reproducible-builds.org)).  
The build environment is created and fully documented via [repro-env](https://github.com/kpcyrd/repro-env), and is tracked in this repository.

To reproduce the pre-compiled binary for a given release, [install repro-env](https://github.com/kpcyrd/repro-env#download) and run the following:

```bash
git clone https://github.com/Antiz96/lungo.git
cd lungo
git checkout <tag> # Where <tag> is the git tag for the targeted release, e.g. "v1.0.0"
repro-env build -- cargo build --release --target x86_64-unknown-linux-musl
sha256sum target/x86_64-unknown-linux-musl/release/lungo
```

Then, compare the `sha256` hash of the built binary to the one of the pre-compiled release binary (which is also recorded in the `lungo-<release_version>-x86_64.sha256` file in the release assets). Both hashes should be equal, indicating that the binary has been successfully reproduced.

Each release assets are also cryptographically signed, with the detached signature for each asset distributed as `<asset_name>.asc` (see the [MAINTAINERS.md file](https://github.com/Antiz96/lungo/blob/main/MAINTAINERS.md) for a list of keys expected to emit signatures).

### Build from source

```bash
git clone https://github.com/Antiz96/lungo.git
cd lungo
cargo build --release
```

The built binary will be located at `./target/release/lungo`, place it somewhere in your `$PATH`.

Place the icons from the [`res/icons/`](https://github.com/Antiz96/lungo/tree/main/res/icons) directory in an "icon compliant" folder on the system (such as `~/.icons/lungo/`, `~/.local/share/icons/lungo` or `/usr/share/icons/lungo/`, create it if needed). For packaging, using the `hicolor-icon-theme` folder (e.g. `/usr/share/icons/hicolor/scalable/apps/`) is an option.

A `.desktop` file is available in the [`res/desktop/`](https://github.com/Antiz96/lungo/tree/main/res/desktop) directory, allowing to automatically start the systray applet at boot via [XDG Autostart](https://wiki.archlinux.org/title/XDG_Autostart) (by placing it either in `~/.config/autostart/` or `/etc/xdg/autostart/`).

The [man page](https://github.com/Antiz96/lungo/tree/main/doc/man) can be generated with `scdoc`:

```bash
scdoc < doc/man/lungo.1.scd > doc/man/lungo.1
```

There are also shell completions available in the [`res/completions/`](https://github.com/Antiz96/lungo/tree/main/res/completions) directory.

## Usage

Run `lungo` to start the systray applet.

To start it automatically at boot, either place the [`.desktop file`](https://github.com/Antiz96/lungo/blob/main/res/desktop/lungo.desktop) in `~/.config/autostart/` or `/etc/xdg/autostart/` (relies on [XDG Autostart](https://wiki.archlinux.org/title/XDG_Autostart)). Alternatively (if your graphical environment doesn't support XDG Autostart for instance), add the `lungo` command to your environment's auto-start method.

Simply click on the systray applet to toggle it on and off.  
The icon automatically changes to reflect the current state ("empty cup" icon if off, "hot cup" icon if on).  
The state can also be consulted and toggled from the systray applet menu.

## Documentation

See `lungo --help` and the [lungo(1) man page](https://raw.githubusercontent.com/Antiz96/lungo/refs/heads/main/doc/man/lungo.1.scd).

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/lungo/blob/main/CONTRIBUTING.md).

## License

Lungo is licensed under the [GPL-3.0 license](https://github.com/Antiz96/lungo/blob/main/LICENSE) (or any later version of that license).
