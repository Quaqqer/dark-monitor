# dark-monitor

A desktop-agnostic monitor of the theme for Linux.

## Features

- Desktop agnostic
- Minimalistic
- Run scripts when the theme changes
- Run multiple instances parallel
- Read dark mode
- Toggle dark mode

## Usage

Listen to changes

```sh
# Monitor color-scheme
dark-monitor --default-as light \
  --on-dark 'echo Switching to dark mode...' \
  --on-light 'echo Switching to light mode...'

# Listen to changes and print to stdout
dark-monitor listen

# Get color-scheme
dark-monitor get-color-scheme

# Set color-scheme (currently only works for GNOME/GTK portals)
dark-monitor set-color-scheme dark
dark-monitor toggle-dark-mode
```

## Examples

I've provided some examples in
[`recipes/`](https://github.com/Quaqqer/dark-monitor/tree/trunk/recipes), if you
find this tool useful and would like to share how you are using it, please make
a pull request!

## Installation

dark-monitor depends on dbus and pkg-config, the desktop should have a portal
that implements the color-scheme property (GNOME, KDE, GTK[^1], darkman[^2]).

Using cargo:

```
cargo install --git https://github.com/Quaqqer/dark-monitor.git
```

## Background

On Linux desktops the XDG Desktop Portal[^3] provides an interface for common
things between desktop environments, such as settings. One thing it provides
access to is the color-scheme[^4] property, which is either 'default',
'prefer-dark', or 'prefer-light'. We can use D-Bus to monitor changes to this
property and act accordingly.

## Related projects and inspiration

- darkman[^2] is a service for dark-mode and light-mode, allowing the user to
  specify scripts that run when switching between the two. A difference is that
  this not only monitors the dark-mode, it also provides the property. The
  scope of this project is to only monitor and react to modes changing, instead
  of providing the property, to be desktop agnostic.
- Yin-Yang[^5] is a desktop agnostic graphical application that similarly to
  this project reacts to theme changes. It has built-in support for a list of
  applications, scripting, and more.
- swayidle[^6] is a service that inspired this project. It runs scripts when
  the user has been idle for a certain amount of time. swayidle has a very
  good, minimalistic command line interface.

[^1]: https://github.com/flatpak/xdg-desktop-portal-gtk
[^2]: https://gitlab.com/WhyNotHugo/darkman
[^3]: https://flatpak.github.io/xdg-desktop-portal/docs/index.html
[^4]: https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html
[^5]: https://github.com/oskarsh/Yin-Yang
[^6]: https://github.com/swaywm/swayidle
