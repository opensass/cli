<div align="center">

# 🧩 OpenSASS

[![Crates.io](https://img.shields.io/crates/v/opensass)](https://crates.io/crates/opensass)
[![Crates.io Downloads](https://img.shields.io/crates/d/opensass)](https://crates.io/crates/opensass)
![Crates.io License](https://img.shields.io/crates/l/opensass)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)

[![Join our Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

<!-- absolute url for docs.rs cause assets is excluded from crate -->
![logo](https://raw.githubusercontent.com/opensass/cli/refs/heads/main/assets/logo.webp)

| 🐧 Linux `(Recommended)` |        🪟 Windows        |
| :----------------------: | :----------------------: |
| `cargo install opensass` | `cargo install opensass` |
| [Download Executable File](https://github.com/opensass/cli/releases/download/v0.0.5/os) | [Download `.exe` File](https://github.com/opensass/cli/releases/download/v0.0.5/os.exe) |
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

<video src="https://github.com/user-attachments/assets/add61376-cdf6-4a46-9d3b-067bc4d0c4bd"></video>

### Yew Usage

```sh
os add i18nrs yew
```

This will:

- Download the `i18nrs` crate.
- Extract files only related to the `yew` feature.
- Copy `src/` files into your project under a new directory `crate_name`, in this case `i18nrs`.
- Update your `Cargo.toml` dependencies and `lib.rs` file.

### Dioxus Usage

```sh
os add i18nrs dio
```

### Leptos Usage

```sh
os add i18nrs lep
```

## 🧃 Components

Open SASS offers the following components:

| 🧩 Component | 📦 Repository | 📝 Description |
|------------------|--------------------------|---------------------|
| `accordion-rs` | [![GitHub](https://img.shields.io/github/stars/opensass/accordion-rs)](https://github.com/opensass/accordion-rs) | ↕️ A highly customizable accordion component for WASM frameworks. |
| `alert-rs`     | [![GitHub](https://img.shields.io/github/stars/opensass/alert-rs)](https://github.com/opensass/alert-rs)       | ⚠️ A highly customizable alert component for WASM frameworks. |
| `eld`       | [![GitHub](https://img.shields.io/github/stars/opensass/eld)](https://github.com/opensass/eld)                 | 🚛 ELD Toolkit for WASM frameworks. |
| `i18nrs`      | [![GitHub](https://img.shields.io/github/stars/opensass/i18n-rs)](https://github.com/opensass/i18n-rs)         | 🌐 Internationalization (i18n) component for WASM frameworks. |
| `image-rs`    | [![GitHub](https://img.shields.io/github/stars/opensass/image-rs)](https://github.com/opensass/image-rs)        | 🖼️ Image Component for WASM frameworks. |
| `input-rs`     | [![GitHub](https://img.shields.io/github/stars/opensass/input-rs)](https://github.com/opensass/input-rs)       | 🔤 A highly customizable input component for WASM frameworks. |
| `navbar`    | [![GitHub](https://img.shields.io/github/stars/opensass/navbar)](https://github.com/opensass/navbar)     | 🍔 A highly customizable navbar component for WASM frameworks. |
| `radiors`     | [![GitHub](https://img.shields.io/github/stars/opensass/radio-rs)](https://github.com/opensass/radio-rs)       | 🎛️ A highly customizable radio buttons component for WASM frameworks. |
| `scroll-rs`    | [![GitHub](https://img.shields.io/github/stars/opensass/scroll-rs)](https://github.com/opensass/scroll-rs)     | 🖱️ A highly customizable scroll-to-anywhere component for WASM frameworks. |
| `select-rs`    | [![GitHub](https://img.shields.io/github/stars/opensass/select-rs)](https://github.com/opensass/select-rs)     | 🔽 A highly customizable select group component for WASM frameworks. |
| `sidebar`    | [![GitHub](https://img.shields.io/github/stars/opensass/sidebar)](https://github.com/opensass/sidebar)     | 🗃️ A sidebar component for WASM frameworks. |
| `skeleton-rs`    | [![GitHub](https://img.shields.io/github/stars/opensass/skeleton-rs)](https://github.com/opensass/skeleton-rs)     | 🦴 A skeleton component for WASM frameworks. |
| `table-rs`    | [![GitHub](https://img.shields.io/github/stars/opensass/table-rs)](https://github.com/opensass/table-rs)     | 📋 Table component for WASM frameworks. |

And much more coming over time...

## ⚡ Benchmark

```sh
❯ time yes | npx shadcn@latest add accordion
npx shadcn@latest add accordion  8.98s user 4.85s system 90% cpu 15.279 total
```

```sh
❯ time os add accordion-rs yew
os add accordion-rs yew  0.16s user 0.02s system 5% cpu 0.2 total
```

Open SASS CLI is **~56× faster** in user time and uses **~18× less CPU** than `shadcn`. More optimizations on the way 🚀.

## 🤝 Contributions

Contributions are welcome! Whether it's bug fixes, feature requests, or examples, we would love your help to make Open SASS better.

1. Fork the repository.
1. Create a new branch for your feature/bugfix.
1. Submit a pull request for review.

## 📜 License

Open SASS is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this library in your projects.
