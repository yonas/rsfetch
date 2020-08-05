## rsfetch

This is a WIP rewrite of [rsfetch](https://github.com/rsfetch/rsfetch) from scratch.<br>
I've been really unhappy with the codebase now for various reasons.<br>
It's made development for rsfetch downright unpleasant at times.

----

A blazing fast (<5 ms) information fetching utility.<br>
As of writing this, plain output takes a total of 1.9 ms to display all available fields.<br>
While pretty output takes a total of 3 ms to display all fields.<br>
According to [`hyperfine`](https://github.com/sharkdp/hyperfine) anyways.

Table of Contents:

- [Example Output](#example-output)
- [Features](#features)
- [Help](#help)
- [TODO](#todo)

## Example Output

`$ cargo run --features=music,plain_output -- -cDdEehkmMsuUp portage`

```
CPU:          Intel(R) Core(TM) i5-3470 CPU @ 3.20GHz
Device:       OptiPlex 7010
Environment:  bspwm
Distro:       Gentoo
Editor:       /usr/bin/emacsclient
Hostname:     gentoo
Kernel:       5.4.48-ck-valley
Memory:       15971 MB
Packages:     88 (explicit), 576 (total)
Shell:        /bin/bash
Uptime:       6d 23h 35m
User:         valley
Music:        The Prodigy - The Fat of the Land (1997) - Breathe
```

`$ cargo run --features=music,pretty_output -- -cDdEehkmMsuUp portage`

```
+─────────────+─────────────────────────────────────────────────+
 CPU          │Intel(R) Core(TM) i5-3470 CPU @ 3.20GHz
 Device       │OptiPlex 7010
 Environment  │bspwm
 Distro       │Gentoo
 Editor       │/usr/bin/emacsclient
 Hostname     │gentoo
 Kernel       │5.4.48-ck-valley
 Memory       │15971 MB
 Packages     │88 (explicit), 576 (total)
 Shell        │/bin/bash
 Uptime       │6d 23h 47m
 User         │valley
 Music        │The Prodigy - The Fat of the Land (1997) - Serial Thrilla
+─────────────+─────────────────────────────────────────────────────────────────+
```

## Features

Currently there are 2 features. One or the other must be chosen for each section.<br>

Music:

- `nomusic`, this will cause rsfetch to not pull in the `mpd` crate, and will cause the music function to output `N/A`.
- `music`, this will cause rsfetch to pull in the `mpd` crate, and display the music info as: 

`artist - album (date) - title`

Output:

- `plain_output`, see example above for how it looks.
- `pretty_output`, this will pull in the `prettytable-rs` crate, see example above for how it looks.

## Help

`$ cargo run -- --help`

```
rsfetch 0.1.0
Phate6660 <https://pages.codeberg.org/Phate6660>

An info fetch tool written in Rust. Everything is off by default, enable what you want.

USAGE:
    rsfetch [FLAGS] [OPTIONS]

FLAGS:
    -c               Display the model of the CPU.
    -D               Display the name of the device.
    -d               Display the name of the distro.
    -E               Display the name of the user's editor. Must have the $EDITOR environmental variable set.
    -e               Display the user's environment. First checks for a DE, before resorting to parsing your
                     $HOME/.xinitrc for your WM.
        --help       Prints help information
    -h               Display the hostname of the device.
    -k               Display the name of the kernel.
    -m               Display free/total memory.
    -M               Display currently playing music. Only mpd is supported. Must be built with the music feature.
    -s               Display the name of the user's shell.
    -u               Display the uptime.
    -U               Display the name of the user.
    -V, --version    Prints version information

OPTIONS:
    -p, --packages <manager>    Display package count.
```

## TODO

- Better output (implemented, might expand in future)
- CPU info (implemented, only tested on Gentoo)
- DE/WM info (implemented, only tested on my setup)
- memory info (partially implemented, shows total in MBs)
- properly implement async
- terminal info
