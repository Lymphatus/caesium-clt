# 🖼️ Caesium Command Line Tools [![](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/Lymphatus)

[![Test](https://github.com/Lymphatus/caesium-clt/workflows/Test/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions?query=workflow%3ATest)
[![Clippy](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml)
[![Code formatting](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml)

> **Supercharge your image optimization workflow with lightning-fast compression**

**caesiumclt** is a powerful command-line tool for image compression written in Rust that delivers exceptional results with minimal effort. Whether you need pixel-perfect lossless compression or aggressive size reduction, caesiumclt has you covered.

Built on the robust [libcaesium](https://github.com/Lymphatus/libcaesium) engine, it combines high performance with flexibility to fit seamlessly into your workflow.

<p align="center">
    <img alt="demo" src="https://github.com/user-attachments/assets/675d9a49-55cb-42d7-b435-de39b6917315">
</p>

## ✨ Features

- **Multiple Compression Modes**
  - 🔍 Quality-based lossy compression
  - 💎 Lossless compression - preserve image quality while reducing size
  - 📏 Maximum size compression - target specific file sizes

- **Advanced Image Processing**
  - 📐 Resize images with multiple options (width, height, long/short edge)
  - 🔄 Convert between formats (JPEG, PNG, WebP)
  - 🏷️ EXIF metadata preservation

- **Workflow Optimization**
  - 📁 Recursive directory compression
  - 🌳 Folder structure preservation
  - 🕒 File timestamp preservation
  - 📝 Custom suffix for output files
  - ⚡ Fast and customizable multithreaded processing

### Supported Formats

| Format   | Lossy | Lossless |
|----------|:-----:|:--------:|
| **JPEG** |   ✅   |    ✅     |
| **PNG**  |   ✅   |    ✅     |
| **WebP** |   ✅   |    ✅     |

## 🚀 Getting Started

### Binaries

Download the latest binaries from the [releases page](https://github.com/Lymphatus/caesium-clt/releases).

**Supported platforms:**
- Windows (x86_64)
- macOS (x86_64/aarch64)
- Linux (x86_64/aarch64)

## 🛠️ Development

### Requirements

* [Rust](https://www.rust-lang.org/tools/install) >= 1.79.0

### Quick Start

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Lymphatus/caesium-clt.git
   cd caesium-clt
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Build for release (optimized):**
   ```bash
   cargo build --release
   ```

4. **Run the tool directly:**
   ```bash
   cargo run -- [your-arguments-here]
   ```

5. **Run tests:**
   ```bash
   cargo test
   ```

## 📖 Usage

For a complete list of all flags and options, refer to the [full documentation](docs/USAGE.md).

### Examples

**Lossless compression:**
```bash
# Compress a single image
$ caesiumclt --lossless -o ~/output/ ~/image.jpg

# Compress with metadata preservation
$ caesiumclt --lossless -e --keep-dates -o ~/output/ ~/image.jpg

# Compress a directory recursively
$ caesiumclt --lossless -R -o ~/output/ ~/Pictures

# Preserve folder structure
$ caesiumclt --lossless -RS -o ~/output/ ~/Pictures
```

**Lossy compression:**
```bash
# Compress with quality setting
$ caesiumclt -q 80 -o ~/output/ ~/image.jpg
```
