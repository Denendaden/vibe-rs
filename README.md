# vibe-rs

Command-line tool to vibrate a toy while running a command, written in [Rust](https://www.rust-lang.org/) and the [Buttplug](https://buttplug.io/) framework.
Requires toys to be connected using [Intiface](https://intiface.com/).

## Installation

With Rust and Cargo installed, run `cargo install vibe-rs`.
Or download this repository and install from there.

## Usage

Run `vibe [command]`.
It's like `sudo`, but with `vibe` instead of `sudo`.

Any toys connected via Intiface will begin vibrating for however long the command takes to run.
If the command finishes successfully, the vibrations will gently fade out; otherwise, they will finish in an aggressive burst.

To adjust the speed of the vibrations, set the environment variable `VIBE_STRENGTH`.
The default is 0.25, so the toys will vibrate at a quarter of their maximum speed.
