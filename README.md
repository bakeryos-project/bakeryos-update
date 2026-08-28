# BakeryOS Update

A GTK4 graphical application for BakeryOS that checks for and installs system updates. It lists packages with newer versions, lets users deselect individual packages, and requests administrator authentication when an update starts.

## Features

- Check for available package updates using `checkupdates`; when necessary, synchronize the package database with `pacman`.
- Display the update list in a GNOME/libadwaita interface.
- Select or deselect packages before updating.
- Install updates with `pkexec pacman -Syu --noconfirm`.
- Support translations through gettext.

> This application directly modifies system packages. Review the update list carefully and do not shut down the computer while an update is in progress.

## Requirements

The project is designed for Pacman-based systems such as BakeryOS or Arch Linux. To build from source, you need:

- Rust and Cargo
- Meson (>= 1.0) and Ninja
- GTK4, libadwaita, GLib/GIO, and gettext
- `blueprint-compiler`

To use the application, the system must provide `pacman` and `pkexec` (Polkit). Installing `pacman-contrib` is recommended because it provides `checkupdates`.

Example for Arch Linux:

```bash
sudo pacman -S --needed base-devel meson ninja rust cargo gtk4 libadwaita \
  gettext blueprint-compiler pacman-contrib polkit
```

## Build and run

```bash
meson setup build
meson compile -C build
./build/src/bakeryos-update
```

Meson generates the UI resources, Rust configuration, and binary in the `build` directory. Do not run the application with `sudo`; when administrator privileges are needed, it opens a Polkit prompt through `pkexec`.

To install it on the system:

```bash
meson install -C build
```

## Packaging

The project includes a `PKGBUILD` for Arch Linux. Build and install the package with:

```bash
makepkg -si
```

## License

This project is released under the GNU GPL v3.0 or later
