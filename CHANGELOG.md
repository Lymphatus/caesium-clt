# Changelog

## v1.2.0

### Added

- **ICC Profile Stripping**: New `--strip-icc` flag to remove ICC profile information from JPG files, regardless of the
  `-e` flag
- **No Upscale Option**: New `--no-upscale` flag to prevent upscaling images when resizing

### Changed

- Updated libcaesium dependency from 0.19 to 0.20
- Enhanced ICC profile handling for JPEG compression

### Fixed

- Fixed base path computation for files on different drives (Windows)
- Fixed the output path computation when the base directory is empty

### Documentation

- Updated USAGE.md with new command-line options

---

## v1.1.1

### Fixed

- Fixed resize parameters building when both width and height are specified
- Improved resize parameter logic to properly handle width and height combinations

### CI/CD

- GitHub Actions workflow for automated crates.io publishing

---

## v1.1.0

### Added

- **GIF Support**: Added GIF compression support (no format conversion to GIF)
- **No Upscale Feature**: Prevent upscaling images when resizing with `--no-upscale` flag
- **Human-readable File Sizes**: `--max-size` now accepts human-readable formats (e.g., `100KB`, `0.5MB`, `1GiB`)

### Changed

- Updated libcaesium from 0.18 to 0.19

### Fixed

- Fixed resize logic to handle both width and height parameters simultaneously
- Fixed GIF format mapping in compression parameters
- Added proper GIF quality handling

### Documentation

- Updated README with GIF support information
- Added installation instructions for Cargo and Homebrew

---

## v1.0.2

### Changed

- Enhanced file type detection using first bytes reading instead of full file inference (performance improvement)
- Improved `get_file_mime_type` function efficiency

### Fixed

- **Critical Fix**: Fixed output path handling in dry-run mode
- Fixed `same_folder_as_input` logic to properly place files in the same directory as input
- Improved output directory creation to skip in dry-run mode

---

## v1.0.1

### Changed

- Updated libcaesium from 0.17.3 to 0.18.0
- Enhanced lossless compression handling for different formats (JPEG, PNG, WebP)

### Fixed

- Fixed lossless flag application – now properly sets format-specific optimize flags instead of a global flag

### CI/CD

- Added Homebrew automated release workflow

---

## v1.0.0

### Features

- **Multi-format support**: JPEG, PNG, WebP
- **Compression options**:
    - Quality-based compression
    - Lossless compression mode
    - Target maximum file size
- **Resize options**:
    - Width and height
    - Long-edge and short-edge
- **Metadata handling**:
    - EXIF metadata preservation with `--exif` flag
    - Keep original file dates with `--keep-dates` flag
- **Format conversion**: Convert between supported image formats
- **Batch processing**:
    - Recursive directory scanning with `-R` flag
    - Progress bar with different verbosity levels
- **Output options**:
    - Dry-run mode
    - Custom output directory
    - Suffix support for output files
    - Keep folder structure option
    - Same folder as input option
- **Overwrite policies**: all, prompt, bigger, none
- **Format-specific options**:
    - JPEG: chroma subsampling, baseline mode
    - PNG: optimization levels (0-6), Zopfli compression

---

## v1.0.0-beta.2

### Changed

- Progress bar refactoring and code cleanup
- Improved user interface and output formatting

---

## v1.0.0-beta.1

### Changed

- Verbose level as numbers instead of strings
- Improved CLI argument handling and validation

---

## v1.0.0-beta.0

### Added

- Same folder as input option
- Suffix option for output files
- Lossless compression flag

### Changed

- Major code refactoring for stability and maintainability

---

## v0.21.0

### Added

- Compress by quality option
- Long edge/Short edge resize option

---

## v0.20.0

### Added

- Keep the original dates option
- Format conversion support

---

## v0.19.3

### Fixed

- Various bug fixes and stability improvements

---

## v0.19.0

### Changed

- **Major**: Complete migration to Rust
- Improved performance and reliability
- Better cross-platform support

---

## v0.18.0

### Fixed

- Fixed Windows build issues

### Changed

- Updated libcaesium to 0.9.3

---

## v0.17.0

### Changed

- Updated libcaesium to 0.9.2

---

## v0.16.0

### Changed

- Now using libcaesium Rust library

---

## v0.15.2

### Fixed

- Fixed the Windows -RS flag bug

---

## v0.15.1

### Fixed

- Fixed the rename bug on Windows

### Added

- "Compressing..." message for better user feedback

---

## v0.15.0

### Changed

- Support for libcaesium 0.5.0

---

## v0.14.0

### Added

- `--quiet` option for silent operation

---

## v0.13.1

### Fixed

- General bug fixes

---

## v0.13.0

### Fixed

- General bug fixes

---

## v0.12.1-beta

### Added

- Windows binaries available

### Note

- Beta release – backup recommended before use
- macOS and Linux users must compile from source

---

## v0.12.0

### Added

- Resizing support (experimental)

---

## v0.11.0

### Added

- Dry-run option

### Fixed

- Path handling issues

---

## v0.10.2-beta

### Added

- Windows binaries available
- Basic compression functionality

### Note

- Beta release – backup recommended before use
- macOS and Linux users must compile from source

---

## v0.10.1

### Added

- All core features now available

---

## v0.10.0

### Changed

- Switched to CMake build system
- Now using libcaesium library

---

## v0.9.1

### Added

- Initial development stage

---

[v1.2.0]:  https://github.com/Lymphatus/caesium-clt/releases/tag/v1.2.0

[v1.1.1]:  https://github.com/Lymphatus/caesium-clt/releases/tag/v1.1.1

[v1.1.0]: https://github.com/Lymphatus/caesium-clt/releases/tag/v1.1.0

[v1.0.2]: https://github.com/Lymphatus/caesium-clt/releases/tag/v1.0.2

[v1.0.1]: https://github.com/Lymphatus/caesium-clt/releases/tag/v1.0.1

[v1.0.0]: https://github.com/Lymphatus/caesium-clt/releases/tag/v1.0.0
[v1.0.0-beta.2]: https://github.com/Lymphatus/caesium-clt/releases/tag/v1.0.0-beta. 2
[v1.0.0-beta.1]: https://github.com/Lymphatus/caesium-clt/releases/tag/v1.0.0-beta.1
[v1.0.0-beta.0]: https://github.com/Lymphatus/caesium-clt/releases/tag/v1.0.0-beta.0
[0.21.0]: https://github.com/Lymphatus/caesium-clt/releases/tag/0.21.0
[0.20.0]: https://github.com/Lymphatus/caesium-clt/releases/tag/0.20.0
[0.19.3]: https://github.com/Lymphatus/caesium-clt/releases/tag/0.19.3
[0.12.1-beta]: https://github.com/Lymphatus/caesium-clt/releases/tag/0.12.1-beta
[0.10.2-beta]: https://github.com/Lymphatus/caesium-clt/releases/tag/0.10.2-beta

CHANGELOG
