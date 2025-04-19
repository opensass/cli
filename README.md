<div align="center">

# 🧩 OpenSASS

[![Crates.io](https://img.shields.io/crates/v/opensass.svg)](https://crates.io/crates/opensass)
[![docs](https://docs.rs/opensass/badge.svg)](https://docs.rs/opensass/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

| 🐧 Linux `(Recommended)` |        🪟 Windows        |
| :----------------------: | :----------------------: |
| `cargo install opensass` | `cargo install opensass` |
| [Download Executable File](https://github.com/opensass/cli/releases/download/v0.0.1/os) | [Download `.exe` File](https://github.com/opensass/cli/releases/download/v0.0.1/os.exe) |
|         `os -h`          |         `os -h`          |

</div>

> 🧩 **OpenSASS**: A CLI tool for downloading reusable OpenSASS components from crates.io and integrating them into your WASM frontend projects.

## 📖 Table of Contents

- [Installation](#-installation)
- [What is OpenSASS?](#-what-is-opensass)
- [Features](#-features)
- [Usage](#-usage)
- [Examples](#-examples)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Installation

To install the CLI:

```sh
cargo install opensass
```

Or build from source:

```sh
git clone https://github.com/opensass/cli.git
cd cli
cargo build --release
```

## ❓ What is OpenSASS?

**OpenSASS** is a modular CLI tool that enables you to:

- Download reusable Rust/WASM components published to `crates.io`.
- Automatically resolve and import component-specific dependencies and features.
- Copy relevant source files directly into your `src/` directory.
- Seamlessly integrate with frontend frameworks like `Yew`, `Leptos`, and `Dioxus`.

It simplifies the process of reusing frontend Rust code across projects.

## ✨ Features

- 🧩 Add OpenSASS component-based crates with a single command.
- ⚙ Automatically updates `Cargo.toml` with proper features.
- 🔁 Copies only the `src/` files related to the specified feature.

## 💡 Usage

### Add an OpenSASS component to your project

```sh
os add i18nrs yew
```

This will:

- Download the `i18nrs` crate.
- Extract files only related to the `yew` feature.
- Copy `src/` files into your project.
- Update your `Cargo.toml` dependencies and feature flags.

## 🧪 Examples

### Using with Yew

```sh
os add radiors yew
```

### Using with Leptos

```sh
os add radiors lep
```

### Using with Dioxus

```sh
os add radiors dio
```

## 📦 Roadmap

- [x] Add components by feature.
- [x] Auto-update `lib.rs` & `Cargo.toml`.
- [ ] Initialize custom WASM templates.
- [ ] TUI support.
- [ ] VS Code extension?

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork this repo
1. Create your branch (`git checkout -b feature/my-feature`)
1. Commit your changes (`git commit -am 'Add new feature'`)
1. Push and open a PR

## 📄 License

Licensed under the [MIT License](LICENSE).
