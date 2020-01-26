## Caesium CommandLineTools
##### caesium-clt - v0.15.1-beta (build 20200123) - Copyright &copy; Matteo Paonessa, 2020. All Rights Reserved.
[![Build Status](https://travis-ci.org/Lymphatus/caesium-clt.svg?branch=master)](https://travis-ci.org/Lymphatus/caesium-clt)
----------

###### REQUIREMENTS
* [libcaesium](https://github.com/Lymphatus/libcaesium) >= 0.5.0

###### Included libraries
* [optparse](https://github.com/skeeto/optparse)
* [tinydir](https://github.com/cxong/tinydir)

----------

###### TESTED PLATFORMS
* Mac OS X Catalina (v10.15)
* Ubuntu 19.04
* Windows 10

----------

###### COMPILATION
See INSTALL.md for more details.

----------

###### COMMAND LINE ARGUMENTS
- `-q, --quality [value]` {Required}  
    Sets the quality of the image. The higher the value is, better the result will be. Note that a value of 0 will mean
    _lossless_ compression, which will not modify the original image, but will compress less. Allowed range is [0. 100].  
    A common value for lossy compression is 80.
- `-e, --exif`  
    Keeps the JPEG metadata information during compression. File size will be slightly higher.
- `-o, --output [value]` {Required}  
    Path to the output folder where the compressed files will be stored. Can be the same input folder, which will overwrite the original files.
- `-s, --scale [value]`  
    Will scale the image to a factor of _[value]_. Allowed formats are decimal values, fractions and percentages.  
    The factor must be a number > 0 and <= 1.0, as caesiumclt won't upscale any image.  
    _This is an experimental feature and might not work as expected._
- `-R, --recursive`  
    If the input is a folder, caesiumclt will also scan every subfolder searching for images.  
    Note that this may end up building a large set of files to be compressed and should be used carefully.
- `-S, --keep-structure`  
    If the input is a folder, and the `-R` option is set, caesiumclt will compress all the files keeping the original folder structure.
- `-O, --overwrite`  
    Sets the overwrite policy: `all` will overwrite any existing file, `prompt` will ask each time before overwriting, `bigger` will overwrite bigger files only, and `none` will silently skip existing files.
- `-d, --dry-run`  
    If this option is set, no files will be compressed, but the entire process will just be simulated.  
    Useful for checking if all the files will be correctly handled.
- `-Q, --quiet`  
    Suppress all output. Output from the libcaesium library will still be outputted.
- `-h, --help`  
    Displays a summary of the command line arguments, much like this one you're reading.
- `-v, --version`  
    Prints the current caesiumclt version.
    

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

Compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` with lossy compression, quality set to 80 and resize it to half
```
$ caesiumclt -q 80 -s 0.5 -o ~/output/ ~/image.jpg
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

----------

###### CHANGELOG
* 0.15.1-beta - Fixed rename bug on Windows + "Compressing..." message
* 0.15.0-beta - Support for libcaesium 0.5.0
* 0.14.0-beta - Added --quiet option
* 0.13.1-beta - Bugfix
* 0.13.0-beta - Bugfix
* 0.12.1-beta - Bugfix
* 0.12.0-beta - Resizing (experimental)
* 0.11.0-beta - Fixing paths issues and dry-run option
* 0.10.2-beta - Bugfixes & full Windows support
* 0.10.1-beta - All features are available
* 0.10.0-beta - Switched to cmake build system and libcaesium
* 0.9.1-beta - Initial development stage
