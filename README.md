# Caesium Command Line Tools [![](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/Lymphatus)

[![Test](https://github.com/Lymphatus/caesium-clt/workflows/Test/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions?query=workflow%3ATest)
[![Clippy](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml)
[![Code formatting](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml)

**caesiumclt** is a powerful command-line tool for image compression written in Rust that delivers exceptional results
with minimal effort. Whether you need pixel-perfect lossless compression or aggressive size reduction, caesiumclt has
you covered.

Built on the robust [libcaesium](https://github.com/Lymphatus/libcaesium) engine, it combines high performance with
flexibility to fit seamlessly into your workflow.

<p align="center">
    <img alt="demo" src="https://github.com/user-attachments/assets/dcf0b52c-6fad-4c7c-8b60-27e40d393264">
</p>

## ✨ Features

- **Multiple Compression Modes**
    - 🔍 Quality-based lossy compression
    - 💎 Lossless compression—preserve image quality while reducing size
    - 📏 Maximum size compression - target specific file sizes

- **Advanced Image Processing**
    - 📐 Resize images with multiple options (width, height, long/short edge)
    - 🔄 Convert between formats (JPEG, PNG, WebP)
    - 🏷️ EXIF metadata preservation

- **Workflow Optimization**
    - *️⃣ Recursive directory compression
    - 📁 Folder structure preservation
    - 🕒 File timestamp preservation
    - 📝 Custom suffix for output files
    - ⚡ Fast and customizable multithreaded processing

### Supported Formats

| Format   | Lossy | Lossless |
|----------|:-----:|:--------:|
| **JPEG** |   ✅   |    ✅     |
| **PNG**  |   ✅   |    ✅     |
| **WebP** |   ✅   |    ✅     |
| **GIF**  |   ✅   |    ❌     |

## 🚀 Getting Started

### Binaries

Download the latest binaries from the [release page](https://github.com/Lymphatus/caesium-clt/releases).

**Supported platforms:**

- Windows (x86_64)
- macOS (x86_64/aarch64)
- Linux (x86_64/aarch64)

### Installation

#### Cargo

```bash
cargo install caesiumclt
```

#### Homebrew

```bash
brew install caesiumclt
```

#### Winget

```bash
winget install SaeraSoft.CaesiumCLT
```

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
$ caesiumclt --lossless -o output/ image.jpg

# Compress with metadata preservation
$ caesiumclt --lossless -e --keep-dates -o output/ image.jpg

# Compress a directory recursively
$ caesiumclt --lossless -R -o output/ Pictures

# Preserve folder structure
$ caesiumclt --lossless -RS -o output/ Pictures
```

**Lossy compression:**

```bash
# Compress with quality setting
$ caesiumclt -q 80 -o output/ image.jpg

# Compress multiple images with a specific quality
$ caesiumclt -q 75 -o output/ image1.jpg image2.png image3.webp

# Compress with suffix to avoid overwriting originals
$ caesiumclt -q 85 --suffix _compressed --same-folder-as-input image.jpg
```

**Format conversion:**

```bash
# Convert images to WebP format with quality setting
$ caesiumclt -q 85 --format webp -o output/ Pictures/*.jpg

# Convert PNG to JPEG with maximum quality
$ caesiumclt -q 100 --format jpeg -o output/ image.png
```

**Resizing options:**

```bash
# Resize to specific width (maintaining aspect ratio)
$ caesiumclt --lossless --width 1920 -o output/ image.jpg

# Resize to specific height (maintaining aspect ratio)
$ caesiumclt -q 90 --height 1080 -o output/ image.jpg

# Resize by longest edge (useful for mixed portrait/landscape photos)
$ caesiumclt -q 85 --long-edge 1500 -o output/ Pictures/*.jpg

# Resize by shortest edge
$ caesiumclt -q 85 --short-edge 800 -o output/ Pictures/*.jpg
```

**Advanced options:**

```bash
# Target a specific maximum file size (500KB)
$ caesiumclt --max-size 512000 -o output/ large-image.jpg

# Parallel processing with specific thread count
$ caesiumclt -q 80 --threads 4 -R -o output/ Pictures/

# Dry run to test compression without writing files
$ caesiumclt -q 80 --dry-run -o output/ Pictures/

# PNG optimization with highest compression level
$ caesiumclt --lossless --png-opt-level 6 -o output/ image.png

# JPEG advanced options with specific chroma subsampling
$ caesiumclt -q 85 --jpeg-chroma-subsampling "4:2:0" --jpeg-baseline -o output/ image.jpg
```

**Overwrite policies:**

```bash
# Never overwrite existing files
$ caesiumclt -q 85 -O never -o output/ Pictures/*.jpg

# Overwrite only if the existing file is bigger
$ caesiumclt -q 85 -O bigger -o output/ Pictures/*.jpg
```
