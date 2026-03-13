# The C/C++ Project Manager (CPM) - MVP Version

![Version](https://img.shields.io/badge/version-0.1.0--mvp-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)

> **⚠️ MVP Version**: No pre-built binaries provided. Build from source. Features incomplete. Use at your own risk.

**CPM** is a package manager for C/C++ projects that streamlines project creation, building, and dependency management.

------

## ✨ Features

- 🚀 Quick project scaffolding with `cpm create`
- 📦 Git-based CMake dependency management via `cpm install`
- 🌐 Cross-platform (Windows, macOS, Linux)
- ⚙️ Multi-compiler support (G++,(Gcc, Clang, Clang++ have not yet been implemented.))

------

## 📦 Installation

```bash
git clone <repository-url>
cd cpm
cargo build --release
sudo cp target/release/cpm /usr/local/bin/  # Optional
```

**Prerequisites**: Rust, C/C++ compiler (G++), Git, CMake

------

## 🛠️ Usage

```bash
# Create new project
cpm create my_project
cd my_project

# Build project
cpm build

# Install dependency (Git + CMake only)
cpm install https://github.com/user/lib.git

# Run Projec
cpm run
```

Configure via `cpm.toml`:

```toml
[project]
name = "my_project"
version = "0.1.0"
licencse = "MIT"

[build]
compiler = "g++"
flags = ["-std=c++17", "-O2"]
system_libraries = ["z"]
```

------

## 🤝 Contributing

PRs welcome! Please follow [Conventional Commits](https://www.conventionalcommits.org/) and ensure:

```bash
cargo check && cargo clippy -- -D warnings && cargo test && cargo fmt --all -- --check
```

See [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for style.

------

# Warehouse Address

[github](https://github.com/Gmcihf/cpm)

[gitee](https://gitee.com/Gmcihf/cpm)



# 📄 License

[MIT License](LICENSE)

Dependencies may have different licenses—verify before distribution.

