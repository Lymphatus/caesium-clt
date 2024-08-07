## Caesium CommandLineTools
###### caesium-clt - v0.20.0

###### REQUIREMENTS
* [Rust](https://www.rust-lang.org/tools/install)
----------

###### TESTED PLATFORMS
* macOS Ventura
* Ubuntu 22.04
* Windows 11

----------

###### COMPILATION
`cargo build --release`

----------

###### COMMAND LINE ARGUMENTS
CaesiumCLT - Command Line Tools for image compression

```
USAGE:
    caesiumclt [FLAGS] [OPTIONS] --max-size <max-size> --output <output> --quality <quality> [FILE]...

FLAGS:
    -d, --dry-run           do not compress files but just show output paths
    -e, --exif              keeps EXIF info during compression
    -h, --help              Prints help information
        --keep-dates        keep original file date information
    -S, --keep-structure    keep the folder structure, can be used only with -R
    -Q, --quiet             suppress all output
    -R, --recursive         if input is a folder, scan subfolders too
    -V, --version           Prints version information
        --zopfli            use zopfli when optimizing PNG files (it may take a very long time to complete)

OPTIONS:
        --height <height>                  height of the output image, if width is not set will preserve aspect ratio
                                           [default: 0]
        --max-size <max-size>              set the expected maximum output size in bytes
    -o, --output <output>                  output folder
        --output-format <output-format>    convert the image to the selected format (jpg, png, webp, tiff) [default:
                                           none]
    -O, --overwrite <overwrite>            overwrite policy [default: all]
    -q, --quality <quality>                sets output file quality between [0-100], 0 for optimization
        --threads <threads>                specify the number of parallel jobs (max is the number of processors
                                           available) [default: 0]
        --verbose <verbose>                select how much output you want to see, 0 is equal to -Q, --quiet [default:
                                           1]
        --width <width>                    width of the output image, if height is not set will preserve aspect ratio
                                           [default: 0]

ARGS:
    <FILE>...    Files to process
```

----------

###### USAGE EXAMPLES

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
