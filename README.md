## Caesium CommandLineTools
##### caesium-clt - v0.12.0-beta (build 20180101) - Copyright &copy; Matteo Paonessa, 2018. All Rights Reserved.
[![Build Status](https://travis-ci.org/Lymphatus/caesium-clt.svg?branch=master)](https://travis-ci.org/Lymphatus/caesium-clt)
----------

###### REQUIREMENTS
* [libcaesium](https://github.com/Lymphatus/libcaesium)

###### Included libraries
* [optparse](https://github.com/skeeto/optparse)
* [tinydir](https://github.com/cxong/tinydir)

----------

###### TESTED PLATFORMS
* Mac OS X High Sierra (v10.13.1)
* Ubuntu 16.04
* Windows 10

----------

###### COMPILATION
See INSTALL.md for more details.

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

###### TODO
* Code cleaning
* Deeper error handling

----------

###### CHANGELOG
* 0.12.0-beta - Resizing (experimental)
* 0.11.0-beta - Fixing paths issues and dry-run option
* 0.10.2-beta - Bugfixes & full Windows support
* 0.10.1-beta - All features are available
* 0.10.0-beta - Switched to cmake build system and libcaesium
* 0.9.1-beta - Initial development stage

----------

###### RESOURCES
* caesium-clt website - [http://saerasoft.com/caesium/clt](http://saerasoft.com/caesium/clt)
* Caesium website - [http://saerasoft.com/caesium](http://saerasoft.com/caesium)
* caesium-clt Git Repository - [https://github.com/Lymphatus/caesium-clt](https://github.com/Lymphatus/caesium-clt)
* Author website - SaeraSoft - [http://saerasoft.com](http://saerasoft.com)