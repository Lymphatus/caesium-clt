## Caesium CommandLineTools
##### caesium-clt - v0.10.0-beta (build 20161219) - Copyright &copy; Matteo Paonessa, 2016. All Rights Reserved.

----------

###### REQUIREMENTS
* [libcaesium](https://github.com/mozilla/mozjpeg)

###### Included libraries
* [optparse](https://github.com/skeeto/optparse)
* [tinydir](https://github.com/cxong/tinydir)

----------

###### TESTED PLATFORMS
* Mac OS X Sierra (v10.12.1)
* Arch Linux

----------

###### INSTALLATION
See INSTALL for more details.

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

----------

###### TODO
* Code cleaning
* Keep folder structure

----------

###### KNOWN ISSUES
* It does not work on Windows unless you port getopt to it

----------

###### CHANGELOG
* 0.10.0-beta - Switched to cmake and libcaesium
* 0.9.1-beta - Initial development stage

Check the [Commits](https://github.com/Lymphatus/caesium-clt/commits/master) for a detailed list of changes.

----------

###### RESOURCES
* caesium-clt website - [http://saerasoft.com/caesium/clt](http://saerasoft.com/caesium/clt)
* Caesium website - [http://saerasoft.com/caesium](http://saerasoft.com/caesium)
* caesium-clt Git Repository - [https://github.com/Lymphatus/CaesiumCLT](https://github.com/Lymphatus/caesium-clt)
* Author website - SaeraSoft - [http://saerasoft.com](http://saerasoft.com)
* Twitter - [Matteo Paonessa](https://twitter.com/MatteoPaonessa)
