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
- `--format <FORMAT>`  
  Converts the original image to the selected format. Possible values are:
    - `jpeg`
    - `png`
    - `webp`
    - `tiff`
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
    - `quiet`: Suppress all output
    - `progress`: Show only progress and final results
    - `warnings-and-errors`: Show also skipped and error messages
    - `all`: Print all
- `-h, --help`  
  Print help. A summary can be seen with `-h`.
- `-V, --version`  
  Print version.

### Full help

```
Usage: caesiumclt [OPTIONS] <--quality <QUALITY>|--lossless|--max-size <MAX_SIZE>> <--output <OUTPUT>|--same-folder-as-input> [FILES]...

Arguments:
  [FILES]...


Options:
  -q, --quality <QUALITY>
          sets output file quality between [0-100]

      --lossless
          perform lossless compression

      --max-size <MAX_SIZE>
          set the expected maximum output size in bytes

      --width <WIDTH>
          width of the output image, if height is not set will preserve aspect ratio

      --height <HEIGHT>
          height of the output image, if width is not set will preserve aspect ratio

      --long-edge <LONG_EDGE>
          sets the size of the longest edge of the image

      --short-edge <SHORT_EDGE>
          sets the size of the shortest edge of the image

  -o, --output <OUTPUT>
          output folder

      --same-folder-as-input
          sets the output folder to be the same as the input folder, overwrites original files

      --format <FORMAT>
          convert to the selected output format, or keep the original

          [default: original]
          [possible values: jpeg, png, webp, tiff, original]

      --png-opt-level <PNG_OPT_LEVEL>
          select level for PNG optimization, between [0-6]

          [default: 3]

      --zopfli
          use zopfli when optimizing PNG files (it may take a very long time to complete)

  -e, --exif
          keeps EXIF info during compression

      --keep-dates
          keep original file date information

      --suffix <SUFFIX>
          add a suffix to the output filename

  -R, --recursive
          if input is a folder, scan subfolders too

  -S, --keep-structure
          keep the folder structure, can be used only with -R

  -d, --dry-run
          do not write output files

      --threads <THREADS>
          specify the number of parallel jobs (max is the number of processors available)

          [default: 0]

  -O, --overwrite <OVERWRITE>
          overwrite policy

          [default: all]

          Possible values:
          - all:    Always overwrite
          - never:  Never overwrite
          - bigger: Overwrite only if the file to be overwritten is bigger

  -Q, --quiet
          suppress all output

      --verbose <VERBOSE>
          select how much output you want to see

          [default: progress]

          Possible values:
          - quiet:               Suppress all output
          - progress:            Show only progress and final results
          - warnings-and-errors: Show also skipped and error messages
          - all:                 Print all

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```