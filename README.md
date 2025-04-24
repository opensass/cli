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
- [Features](#-features)
- [Usage](#-usage)
- [Components](#-components)
- [Benchmark](#-benchmark)
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

## 🧃 Components

Open SASS offers the following components:

| 🧩 Component | 📦 GitHub Repository | 📝 Description |
|------------------|--------------------------|---------------------|
| `accordion` | [![GitHub](https://img.shields.io/github/stars/opensass/accordion-rs)](https://github.com/opensass/accordion-rs) | ↕️ A highly customizable accordion component for WASM frameworks. |
| `alert`     | [![GitHub](https://img.shields.io/github/stars/opensass/alert-rs)](https://github.com/opensass/alert-rs)       | ⚠️ A highly customizable alert component for WASM frameworks. |
| `eld`       | [![GitHub](https://img.shields.io/github/stars/opensass/eld)](https://github.com/opensass/eld)                 | 🚛 ELD Toolkit for WASM frameworks. |
| `i18n`      | [![GitHub](https://img.shields.io/github/stars/opensass/i18n-rs)](https://github.com/opensass/i18n-rs)         | 🌐 Internationalization (i18n) component for WASM frameworks. |
| `input`     | [![GitHub](https://img.shields.io/github/stars/opensass/input-rs)](https://github.com/opensass/input-rs)       | 🔤 A highly customizable input component for WASM frameworks. |
| `radio`     | [![GitHub](https://img.shields.io/github/stars/opensass/radio-rs)](https://github.com/opensass/radio-rs)       | 🎛️ A highly customizable radio buttons component for WASM frameworks. |
| `scroll`    | [![GitHub](https://img.shields.io/github/stars/opensass/scroll-rs)](https://github.com/opensass/scroll-rs)     | 🖱️ A highly customizable scroll-to-anywhere component for WASM frameworks. |
| `select`    | [![GitHub](https://img.shields.io/github/stars/opensass/select-rs)](https://github.com/opensass/select-rs)     | 🔽 A highly customizable select group component for WASM frameworks. |

And much more coming over time...

## ⚡ Benchmark

```sh
❯ time npx shadcn@latest add accordion
5.93s user 2.22s system 93% cpu
```

```sh
❯ time os add accordion-rs yew
0.17s user 0.02s system 6% cpu
```

Open SASS CLI is **~50× faster** and uses **~15× less CPU** than `shadcn`. More optimizations on the way 🚀.

## 🤝 Contributions

Contributions are welcome! Whether it's bug fixes, feature requests, or examples, we would love your help to make Open SASS better.

1. Fork the repository.
1. Create a new branch for your feature/bugfix.
1. Submit a pull request for review.

## 📜 License

Open SASS is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this library in your projects.
