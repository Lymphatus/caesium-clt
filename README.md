# Caesium Command Line Tools [![](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/Lymphatus)

[![Test](https://github.com/Lymphatus/caesium-clt/workflows/Test/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions?query=workflow%3ATest)
[![Clippy](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/clippy.yml)
[![Code formatting](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml/badge.svg)](https://github.com/Lymphatus/caesium-clt/actions/workflows/fmt.yml)

v1.0.0-beta.0

## Development

### Requirements

* [Rust](https://www.rust-lang.org/tools/install) >= 1.79.0

### Build

`cargo build --release`

## Usage

See full usage docs [here](docs/USAGE.md).

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
EXIF metadata

```
$ caesiumclt --lossless -e -o ~/output/ ~/image.jpg
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
