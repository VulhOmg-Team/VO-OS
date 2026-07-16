# VO System Info

VO System Info is a system diagnostic tool for VO-OS.

Its purpose is to collect hardware and system information for both users and other VO-OS components.

It serves as the foundation for:

- VO Hardware Advisor;
- VO Guard;
- VO Center.

---

## Current Features

Currently supported:

- operating system detection;
- architecture detection.

---

## Planned Features

Future versions will include:

- CPU detection;
- GPU detection;
- RAM information;
- storage information;
- kernel information;
- driver information;
- hardware compatibility analysis;
- compatibility recommendations.

---

## Build

### Requirements

- Rust
- Cargo

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

---

## Project Goals

VO System Info should be:

- lightweight;
- fast;
- reliable;
- cross-platform where possible;
- easy to maintain.

---

## Future Integration

VO System Info will become the hardware information provider for:

- VO Center;
- VO Guard;
- VO Hardware Advisor;
- VO Installer.

---

## License

VO System Info follows the VO-OS project license.
