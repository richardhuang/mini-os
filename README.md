# Mini OS

A minimal x86_64 operating system written in Rust. This is a learning project.

## Features

- ✅ x86_64 architecture support
- ✅ VGA text mode output (80x25)
- ✅ GDT (Global Descriptor Table)
- ✅ Basic interrupt handling
- ✅ Memory management (Bump Allocator)
- ✅ Simple in-memory file system
- ✅ Hello World program

## Requirements

- Rust Nightly toolchain
- QEMU (x86_64)
- bootimage tool

## Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install QEMU
# macOS
brew install qemu

# Ubuntu/Debian
sudo apt-get install qemu-system-x86

# Install bootimage tool
cargo install bootimage
```

## Build and Run

```bash
# Build
cargo bootimage --target x86_64-unknown-none

# Run with QEMU
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-mini-os.bin \
    -m 512M \
    -serial stdio
```

## Project Structure

```
mini-os/
├── src/
│   ├── main.rs        # Kernel entry point
│   ├── vga.rs         # VGA text output driver
│   ├── gdt.rs         # Global Descriptor Table
│   ├── interrupts.rs  # Interrupt handling
│   ├── memory.rs      # Memory management
│   └── fs.rs          # Simple file system
├── .cargo/config.toml
├── Cargo.toml
└── README.md
```

## License

MIT License
