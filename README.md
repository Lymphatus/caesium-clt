## Caesium Command Line Tools
##### CCLT - v0.9.1-beta (build 20150921) - Copyright &copy; Matteo Paonessa, 2015. All Rights Reserved.

----------

###### REQUIREMENTS
* [mozjpeg](https://github.com/mozilla/mozjpeg)
* [zopfli](https://github.com/google/zopfli)
* [lodepng](https://github.com/lvandeve/lodepng)

----------

###### TESTED PLATFORMS
* MacOSX Yosemite (v. 10.10.5)
* Arch Linux
* Ubuntu 14.04.2

----------

###### INSTALLATION
See INSTALL for more details.

----------

###### USAGE EXAMPLES
```
$ caesiumclt -l -o ~/output/ ~/image.jpg
```
Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output```

```
$ caesiumclt -q 80 -o ~/output/ ~/image.jpg
```
Compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` with lossy compression and quality set to 80

```
$ caesiumclt -l -e -o ~/output/ ~/image.jpg
```
Losslessly compress ```image1.jpg```, located in the ```home``` directory, into a folder called ```output``` and keeps EXIF metadata

```
$ caesiumclt -l -R -o ~/output/ ~/Pictures
```
Losslessly compress ```Pictures``` folder and subfolders, located in the ```home``` directory, into a folder called ```output```

```
$ caesiumclt -q 80 -s 50% -o ~/output/ ~/image1.jpg
```
Compress with quality 80 and resize at 50% ```image1.jpg```, located in the ```home``` directory, into a folder called ```output```

----------

###### TODO
* Code cleaning
* Folder structure support

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
