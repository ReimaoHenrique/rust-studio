# Rust Studio

[![Crates.io](https://img.shields.io/crates/v/rust-studio.svg)](https://crates.io/crates/rust-studio)
[![Documentation](https://docs.rs/rust-studio/badge.svg)](https://docs.rs/rust-studio)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

> ⚠️ **Alpha Version** - This project is currently in alpha stage (v0.1.1). Features may change and breaking changes can occur between releases. Use with caution in production environments.

A modern Rust database client with a user-friendly web interface. Connect to any database via environment variables and manage your data through an intuitive web interface.

## 🚀 Features

- **Web-based Interface**: Modern, responsive web UI for database management
- **Environment-based Configuration**: Configure database connections via environment variables
- **Cargo Integration**: Works as a cargo subcommand for seamless integration
- **Cross-platform**: Works on Linux, macOS, and Windows

## 📦 Installation

Install via cargo:

```bash
cargo install rust-studio
```

## 🛠️ Usage

After installation, you can use Rust Studio as a cargo subcommand:

```bash
# Show help
cargo rust-studio --help

# Start the web server
cargo rust-studio run

# Show custom help
cargo rust-studio help
```

The web interface will be available at `http://localhost:5555` by default.

## ⚙️ Configuration

Configure your database connection using environment variables:

```bash
# Example for PostgreSQL
export DATABASE_URL="postgresql://username:password@localhost/database_name"

# Example for MySQL
export DATABASE_URL="mysql://username:password@localhost/database_name"

# Example for SQLite
export DATABASE_URL="sqlite://path/to/database.db"
```

## 🏗️ Development Status

This project is in **active development** and currently in alpha stage:

- ✅ Basic web server functionality
- ✅ Cargo subcommand integration
- ✅ Static file serving
- 🚧 Database connectivity (planned)
- 🚧 Query interface (planned)
- 🚧 Schema visualization (planned)
- 🚧 Data editing capabilities (planned)

## 🤝 Contributing

Contributions are welcome! Since this is an alpha project, expect frequent changes and please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📝 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Homepage](https://henriquereimao.dev/rust-studio)
- [Repository](https://github.com/ReimaoHenrique/rust-studio)
- [Documentation](https://docs.rs/rust-studio)
- [Crates.io](https://crates.io/crates/rust-studio)

## ⚠️ Alpha Disclaimer

This software is in alpha stage. This means:

- **Breaking changes** may occur between versions
- **Features may be incomplete** or change significantly
- **APIs are not stable** and may change without notice
- **Documentation may be incomplete** or outdated
- **Use in production is not recommended** without thorough testing

We appreciate your patience and feedback as we work towards a stable release!

---

Made with ❤️ by [Henrique Reimão](https://henriquereimao.dev)
