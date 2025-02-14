## Caesium CommandLineTools
###### caesium-clt - v0.21.0

###### REQUIREMENTS
* [Rust](https://www.rust-lang.org/tools/install)
----------

###### COMPILATION
`cargo build --release`

----------

###### COMMAND LINE ARGUMENTS

```
USAGE:
    caesiumclt.exe [FLAGS] [OPTIONS] --quality <quality> [FILE]...

FLAGS:
    -d, --dry-run                 do not compress files but just show output paths
    -e, --exif                    keeps EXIF info during compression
    -h, --help                    Prints help information
        --keep-dates              keep original file date information
    -S, --keep-structure          keep the folder structure, can be used only with -R
    -l, --lossless                perform lossless compression
    -Q, --quiet                   suppress all output
    -R, --recursive               if input is a folder, scan subfolders too
        --same-folder-as-input    sets the output folder to be the same as the input folder. Overwrites original files
    -V, --version                 Prints version information
        --zopfli                  use zopfli when optimizing PNG files (it may take a very long time to complete)

OPTIONS:
        --height <height>                  height of the output image, if width is not set will preserve aspect ratio
        --long-edge <long-edge>            sets the size of the longest edge of the image
        --max-size <max-size>              set the expected maximum output size in bytes
    -o, --output <output>                  output folder
        --output-format <output-format>    convert the image to the selected format (jpg, png, webp, tiff) [default:
                                           none]
    -O, --overwrite <overwrite>            overwrite policy [default: all]
        --png-opt-level <png-opt-level>    select level for PNG optimization, between [0-6] [default: 3]
    -q, --quality <quality>                sets output file quality between [0-100], 0 for optimization
        --short-edge <short-edge>          sets the size of the shortest edge of the image
        --suffix <suffix>                  convert the image to the selected format (jpg, png, webp, tiff) [default:
                                           none]
        --threads <threads>                specify the number of parallel jobs (max is the number of processors
                                           available) [default: 0]
        --verbose <verbose>                select how much output you want to see, 0 is equal to -Q, --quiet [default:
                                           1]
        --width <width>                    width of the output image, if height is not set will preserve aspect ratio

ARGS:
    <FILE>...    Files to process
```

----------

###### USAGE EXAMPLES

Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output```
```
$ caesiumclt --lossless -o ~/output/ ~/image.jpg
```

Compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` with lossy compression and quality set to 80
```
$ caesiumclt -q 80 -o ~/output/ ~/image.jpg
```

Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` and keeps EXIF metadata
```
$ caesiumclt --lossless -e -o ~/output/ ~/image.jpg
```

Losslessly compress ```Pictures``` folder and subfolders, located in the ```home``` directory, into a folder called ```output```
```
$ caesiumclt --lossless -R -o ~/output/ ~/Pictures
```

Losslessly compress ```Pictures``` folder and subfolders, located in the ```home``` directory, into a folder called ```output``` retaining the input folder structure
```
$ caesiumclt --lossless -RS -o ~/output/ ~/Pictures
```
