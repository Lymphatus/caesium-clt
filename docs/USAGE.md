## CaesiumCLT Usage

### Options

##### Compression (at least one is required)

- `-q, --quality <QUALITY>`  
  Sets compression quality between 0 and 100. Higher the value, better the quality and bigger the file size.
- `--lossless`  
  Perform lossless compression. Some file formats may not support this or result in bigger file sizes.
- `--max-size <MAX_SIZE>`  
  Attempts to compress the image to the nearest size in bytes without exceeding it. If the requested size is too small,
  it will output the smallest possible result.

##### Advanced compression

- `--jpeg-chroma-subsampling <JPEG_CHROMA_SUBSAMPLING>`  
  Sets the chroma subsampling for JPEG files. Possible values are:
    - `4:4:4`
    - `4:2:2`
    - `4:2:0`
    - `4:1:1`
    - `auto`

- `--jpeg-baseline`  
  Forces the output to be in baseline JPEG format instead of progressive.

- `--png-opt-level <PNG_OPT_LEVEL>`  
  Sets the optimization level for PNG files. Higher values result in better compression but take longer to complete.
  Possible values are between 0 and 6. Default is 3.
- `--zopfli`  
  Use zopfli when optimizing PNG files. It may take a very long time to complete, especially if the application is not
  build in release mode.

##### Metadata

- `-e, --exif`  
  Keeps EXIF metadata info during compression, if present. This can slightly increase the output file size.
- `--keep-dates`  
  Keeps the original last modified and last accessed date information, if possible.
- `--strip-icc`  
  Strips all ICC profile information on JPG, regardless of `-e` flag.

##### Resizing

- `--width <WIDTH>`  
  Sets the width of the output image. If height is not set, it will preserve the aspect ratio. Can't be used with
  `--long-edge` or `--short-edge`.
- `--height <HEIGHT>`  
  Sets the height of the output image. If width is not set, it will preserve the aspect ratio. Can't be used with
  `--long-edge` or `--short-edge`.
- `--long-edge <LONG_EDGE>`  
  Sets the size of the longest edge of the image. It is aware of rotation metadata. Can't be used with `--width` or
  `--height` or `--short-edge`.
- `--short-edge <SHORT_EDGE>`  
  Sets the size of the shortest edge of the image. It is aware of rotation metadata. Can't be used with `--width` or
  `--height` or `--long-edge`.
- `--no-upscale
  Prevent upscaling the image when resizing

##### Destination

- `-o, --output <OUTPUT>`  
  Sets the output folder. If the folder does not exist, it will be created. Can't be used with `--same-folder-as-input`.
- `--same-folder-as-input`  
  Sets the output folder to be the same as the input folder. WARNING: this can potentially overwrite the original files
  if a suffix is not set. Overwritten files CANNOT be recovered.
- `--suffix <SUFFIX>`  
  Adds a suffix to the output filename, before the file extension.
- `-S, --keep-structure`  
  Preserves the folder tree structure of the input files. Can be used only with `-R`.
- `-o, --overwrite <OVERWRITE>`  
  Sets the overwrite policy if the output file already exists. Possible values are:
    - `all`: Always overwrite
    - `never`: Never overwrite
    - `bigger`: Overwrite only if the file to be overwritten is bigger
- `--min-savings <MIN_SAVINGS>`  
  Skip writing the output file if the compression savings are below the specified threshold.
  This is useful to prevent repeated re-compression from degrading image quality.  
  Supported formats:
    - Percentage (supports decimals): `10%`, `1.5%`, `0.1%`
    - Size with unit: `100KB`, `1MB`, `500KiB`
    - Plain number (interpreted as bytes): `1000`, `50000`
- `--format <FORMAT>`  
  Converts the original image to the selected format. Possible values are:
    - `jpeg`
    - `png`
    - `webp`
    - `original` (default, no changes)

##### Other

- `-R, --recursive`  
  If the input is a folder, scan its subfolders too.
- `-d, --dry-run`  
  Do not write output files, only simulate the process.
- `--threads <THREADS>`  
  Specify the number of parallel jobs. The maximum is the number of processors available. `0` means that the application
  will try to guess the maximum number of parallel jobs available. Default is `0`.
- `-Q, --quiet`  
  Suppress all output. Overrides `--verbose`.
- `--verbose <VERBOSE>`  
  Select how much output you want to see. Possible values are:
    - `0`: Suppress all output
    - `1`: Show only progress and final results
    - `2`: Show also skipped and error messages
    - `3`: Print all
- `-h, --help`  
  Print help. A summary can be seen with `-h`.
- `-V, --version`  
  Print version.

### Full help

Use `--help` to see the full list of options.
