## Caesium Command Line Tools
##### CCLT - v0.9.1-beta (build 20160121) - Copyright &copy; Matteo Paonessa, 2016. All Rights Reserved.

----------

###### REQUIREMENTS
* [mozjpeg](https://github.com/mozilla/mozjpeg)
* [zopfli](https://github.com/google/zopfli)
* [lodepng](https://github.com/lvandeve/lodepng)

----------

###### TESTED PLATFORMS
* Mac OS X El Capitan (v10.11.1)
* Arch Linux
* Ubuntu 14.04.2

----------

###### INSTALLATION
See INSTALL for more details.

----------

###### USAGE EXAMPLES

Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output```
```
$ caesiumclt -l -o ~/output/ ~/image.jpg
```

Compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` with lossy compression and quality set to 80
```
$ caesiumclt -q 80 -o ~/output/ ~/image.jpg
```

Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` and keeps EXIF metadata
```
$ caesiumclt -l -e -o ~/output/ ~/image.jpg
```

Losslessly compress ```Pictures``` folder and subfolders, located in the ```home``` directory, into a folder called ```output```
```
$ caesiumclt -l -R -o ~/output/ ~/Pictures
```

----------

###### TODO
* Code cleaning
* Folder structure support

----------

###### KNOWN ISSUES
* Strange behaviour with network files: the output base_name is the name of the output subfolder (e.g. -o /path/to/images/compressed /path/to/images -> results in all the output images named as "compressed")
* Resizing works only for powers of two (e.g. 50%, 25%, 16%...) and for JPEGs only
* It does not work on Windows unless you port getopt to it

----------

###### CHANGELOG
* 0.9.1-beta - Initial development stage

Check the [Commits](https://github.com/Lymphatus/CaesiumCLT/commits/master) for a detailed list of changes.

----------

###### RESOURCES
* Caesium website - [http://saerasoft.com/caesium](http://saerasoft.com/caesium)
* CCLT Git Repository - [https://github.com/Lymphatus/CaesiumCLT](https://github.com/Lymphatus/CaesiumCLT)
* Author website - SaeraSoft - [http://saerasoft.com](http://saerasoft.com)
* Twitter - [Matteo Paonessa](https://twitter.com/MatteoPaonessa)
