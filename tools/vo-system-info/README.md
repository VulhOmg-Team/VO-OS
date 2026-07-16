# VO System Info

VO System Info is a system diagnostic tool for VO-OS.

It provides hardware and system information required for future VO-OS components such as:

- VO Hardware Advisor;
- VO Guard;
- VO Center.

## Current Features

Currently supported:

- operating system detection;
- architecture detection.

## Planned Features

Future versions will include:

- CPU detection;
- GPU detection;
- RAM information;
- storage information;
- kernel information;
- driver information;
- hardware compatibility analysis.

## Build

Requirements:

- Rust;
- Cargo.

Build:

```bash
cargo build --release
