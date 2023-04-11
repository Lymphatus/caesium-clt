## Caesium CommandLineTools
###### caesium-clt - v0.19.1

###### REQUIREMENTS
* [Rust](https://www.rust-lang.org/tools/install)
----------

###### TESTED PLATFORMS
* macOS Ventura (v13.0)
* Ubuntu 22.04
* Windows 10

----------

###### COMPILATION
`cargo build --release`

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
- `-R, --recursive`  
  If the input is a folder, caesiumclt will also scan every subfolder searching for images.  
  Note that this may end up building a large set of files to be compressed and should be used carefully.
- `-S, --keep-structure`  
  If the input is a folder, and the `-R` option is set, caesiumclt will compress all the files keeping the original folder structure.
- `-O, --overwrite`  
  Sets overwrite policy: `all` will overwrite any existing file, `prompt` will ask each time before overwriting, `bigger` will overwrite bigger files only, and `none` will silently skip existing files.
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
