## Caesium CommandLineTools
###### caesium-clt - v1.0.0-beta.0

###### REQUIREMENTS
* [Rust](https://www.rust-lang.org/tools/install)
----------

###### COMPILATION
`cargo build --release`

----------

###### USAGE

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

          [default: 1]

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

----------

###### EX AMPLES

Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output```
```
$ caesiumclt -q 0 -o ~/output/ ~/image.jpg
```

Compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` with lossy compression and quality set to 80
```
$ caesiumclt -q 80 -o ~/output/ ~/image.jpg
```

Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` and keeps EXIF metadata
```
$ caesiumclt -q 0 -e -o ~/output/ ~/image.jpg
```

Losslessly compress ```Pictures``` folder and subfolders, located in the ```home``` directory, into a folder called ```output```
```
$ caesiumclt -q 0 -R -o ~/output/ ~/Pictures
```

Losslessly compress ```Pictures``` folder and subfolders, located in the ```home``` directory, into a folder called ```output``` retaining the input folder structure
```
$ caesiumclt -q 0 -RS -o ~/output/ ~/Pictures
```
