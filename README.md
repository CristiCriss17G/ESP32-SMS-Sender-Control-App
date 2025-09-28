# ESP32 SMS Sender Control App

A cross-platform desktop application for controlling ESP32-based SMS sender devices via Bluetooth Low Energy (BLE). Built with SvelteKit, Tauri, and Rust for optimal performance and native system integration.

## Features

- 📱 Control ESP32 SMS sender devices wirelessly
- 🔗 Bluetooth Low Energy (BLE) communication
- 🖥️ Cross-platform desktop app (Windows, macOS, Linux)
- ⚡ Fast and lightweight with Tauri + Rust backend
- 🎨 Modern UI with SvelteKit, TailwindCSS, and Skeleton UI
- 📊 Real-time device monitoring and control

## Tech Stack

- **Frontend**: SvelteKit 2.x with TypeScript
- **Backend**: Tauri 2.x with Rust
- **UI Framework**: Skeleton UI with TailwindCSS
- **Package Manager**: Bun
- **Communication**: Bluetooth Low Energy (BLE) via tauri-plugin-blec

## Prerequisites

Before you begin, ensure you have the following installed:

### Bun

Install Bun (JavaScript runtime and package manager):

```sh
# Windows (PowerShell)
irm bun.sh/install.ps1 | iex

# macOS/Linux
curl -fsSL https://bun.sh/install | bash

# Or via npm
npm install -g bun
```

### Rust

Install Rust and Cargo:

```sh
# Install via rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows, download and run rustup-init.exe from https://rustup.rs/
```

### Tauri CLI

Install Tauri CLI for development:

```sh
# Via Cargo
cargo install @tauri-apps/cli@^2.0.0

# Or via Bun
bun add -D @tauri-apps/cli@^2.0.0
```

### System Dependencies

**Windows:**

- Visual Studio Build Tools or Visual Studio with C++ development tools
- WebView2 (usually pre-installed on Windows 11)

**macOS:**

- Xcode Command Line Tools: `xcode-select --install`

**Linux (Ubuntu/Debian):**

```sh
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libbluetooth-dev
```

## Getting Started

1. **Clone the repository:**

```sh
git clone <repository-url>
cd esp32-sms-sender-control-app
```

2. **Install dependencies:**

```sh
bun install
```

3. **Start development server:**

```sh
# Start the Tauri development environment
bun tauri dev

# Or run frontend and backend separately:
bun dev  # Frontend only
```

## Available Scripts

```sh
# Development
bun dev                 # Start Vite dev server
bun tauri dev          # Start Tauri development with hot reload

# Building
bun run build          # Build frontend for production
bun tauri build        # Build complete Tauri application

# Code Quality
bun run check          # Run Svelte type checking
bun run format         # Format code with Prettier
bun run lint           # Lint code with ESLint

# Preview
bun preview           # Preview production build
```

## Project Structure

```text
esp32-sms-sender-control-app/
├── src/                    # SvelteKit frontend source
├── src-tauri/             # Tauri Rust backend
│   ├── src/               # Rust source code
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── static/                # Static assets
├── package.json           # Node.js dependencies and scripts
├── bun.lock              # Bun lockfile
└── README.md
```

## Building for Production

### Desktop Application

```sh
# Build for current platform
bun tauri build

# Build for specific target (requires target toolchain)
bun tauri build --target x86_64-pc-windows-msvc  # Windows
bun tauri build --target x86_64-apple-darwin     # macOS Intel
bun tauri build --target aarch64-apple-darwin    # macOS Apple Silicon
bun tauri build --target x86_64-unknown-linux-gnu # Linux
```

The built application will be available in `src-tauri/target/release/`.

## Development Notes

- **Rust Version**: This project requires Rust 1.77.2 or later
- **Tauri Plugins**: Uses Bluetooth Low Energy plugin for ESP32 communication
- **Frontend Framework**: Built with SvelteKit 5.x and the latest Skeleton UI components
- **Package Manager**: Optimized for Bun, but npm/pnpm/yarn can also be used

## Troubleshooting

### BLE Permission Issues

On macOS and Linux, ensure your application has the necessary Bluetooth permissions.

### Build Errors

If you encounter build errors:

1. Ensure all prerequisites are properly installed
2. Update Rust: `rustup update`
3. Clear build cache: `bun tauri build --clean`

### Dependencies Issues

If dependency installation fails:

```sh
# Clear node_modules and reinstall
rm -rf node_modules bun.lock
bun install
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
