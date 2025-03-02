# Caesium Command Line Tools [![](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/Lymphatus)

-----------------------

[![Test](https://github.com/Lymphatus/caesium-clt/workflows/Test/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions?query=workflow%3ATest)
[![Clippy](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml)
[![Code formatting](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml)

**caesiumclt** is a command line tool for image compression written in Rust, supporting lossy, lossless, and maximum
size compression.
Based on [libcaesium](https://github.com/Lymphatus/libcaesium).
<p align="center">
    <img alt="demo" src="https://github.com/user-attachments/assets/675d9a49-55cb-42d7-b435-de39b6917315">
</p>

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
    - [Binaries](#binaries)
    - [Installation from Source](#installation-from-source)
- [Usage](#usage)
    - [Examples](#examples)
- [Development](#development)
    - [Requirements](#requirements)
    - [Build](#build)

## Features

- Quality-based lossy compression
- Lossless compression - the very same image is returned
- Maximum size compression - the image is compressed to a specific size
- Resize images
- Convert to different formats
- EXIF metadata preservation
- Recursive directory compression
- Folder structure preservation
- File timestamp preservation
- Custom suffix for output files
- Fast and customizable multithreaded processing

### Supported formats

| Format   | Lossy | Lossless |
|----------|-------|----------|
| **JPEG** | ✅     | ✅        |
| **PNG**  | ✅     | ✅        |
| **WebP** | ✅     | ✅        |
| **TIFF** | ❌     | ✅        |

## Getting started

### Binaries

You can download the latest binaries from the [releases page](https://github.com/Lymphatus/caesium-clt/releases).
Supported platforms are:

- Windows (x86_64)
- macOS (x86_64/aarch64)
- Linux (x86_64/aarch64)

### Installation from source

If you have Rust installed, you can install the tool using Cargo:

```bash
cargo install --git https://github.com/Lymphatus/caesium-clt caesiumclt
```

If you instead want to build the tool refer to the [Development](#development) section.

## Usage

For a full list of all flags and options refer to the [full docs](docs/USAGE.md).

### Examples

Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output```

```
$ caesiumclt --lossless -o ~/output/ ~/image.jpg
```

Compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` with lossy compression
and quality set to 80

```
$ caesiumclt -q 80 -o ~/output/ ~/image.jpg
```

Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` and keeps
EXIF metadata and original file date information

```
$ caesiumclt --lossless -e --keep-dates -o ~/output/ ~/image.jpg
```

Losslessly compress ```Pictures``` folder and subfolders, located in the ```home``` directory, into a folder called
```output```

```
$ caesiumclt --lossless -R -o ~/output/ ~/Pictures
```

Losslessly compress ```Pictures``` folder and subfolders, located in the ```home``` directory, into a folder called
```output``` retaining the input folder structure

```
$ caesiumclt --lossless -RS -o ~/output/ ~/Pictures
```

## Development

### Requirements

* [Rust](https://www.rust-lang.org/tools/install) >= 1.79.0

### Build

`cargo build`

