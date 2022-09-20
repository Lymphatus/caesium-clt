## Caesium 命令行工具
###### caesium-clt - v0.16.0-beta (build 20211023)

###### 依赖
* [libcaesium](https://github.com/Lymphatus/libcaesium) >= 0.6.0

###### 已包含的库
* [optparse](https://github.com/skeeto/optparse)
* [tinydir](https://github.com/cxong/tinydir)

----------

###### 已通过测试的平台
* Mac OS X Big Sur (v10.15)
* Ubuntu 20.04
* Windows 10

----------

###### 编译
详情见 INSTALL.md 。

----------

###### 命令行参数
- `-q, --quality [value]` {Required}  
  设置图像质量，值越高，图像越好。当值为 0 将 _无损_ 压缩图片，不会修改原图，但压缩得很少。  
  可选范围为 [0, 100] ，有损压缩的常见值为 `80` 。
- `-e, --exif`  
  压缩过程中保留 JPEG 元数据信息，文件大小将会略高。
- `-o, --output [value]` {Required}  
  压缩文件的输出文件夹的路径，如果和输入文件夹相同将覆盖原文件。
- `-R, --recursive`  
  如果输入是文件夹，caesiumclt 将会递归扫描每个子文件夹以搜索图像。  
  请注意，这最终可能会有大量要压缩的文件，应谨慎使用。
- `-S, --keep-structure`  
  如果输入是文件夹，并且设置了 `-R` 选项，caesiumclt 将保持原始文件夹结构地压缩所有文件。
- `-O, --overwrite`  
  设置覆盖策略：`all` 将覆盖任何现有文件，`prompt` 将在每次覆盖前询问，`bigger` 将仅覆盖更大的文件，而 `none` 将静默跳过现有文件。
- `-d, --dry-run`  
  如果设置了此选项，则不会压缩任何文件，而只是模拟整个过程。   
  用于检查是否所有文件都将会被正确处理。
- `-Q, --quiet`  
  抑制所有的输出，但 libcaesium 库的输出仍将被输出。
- `-h, --help`  
  显示命令行参数的摘要，就像您正在阅读的这个。
- `-v, --version`  
  打印当前的 caesiumclt 版本。


----------

###### 使用示例

将位于 `home` 目录中的 `image1.jpg` 无损压缩到名为 `output` 的文件夹中：
```
$ caesiumclt -q 0 -o ~/output/ ~/image.jpg
```

将位于 `home` 目录中的 `image1.jpg` 压缩到名为 `output` 的文件夹中，且有损压缩和质量设置为 `80`：
```
$ caesiumclt -q 80 -o ~/output/ ~/image.jpg
```

将位于 `home` 目录中的 `image1.jpg` 无损压缩到名为 `output` 的文件夹中，且保留 EXIF 元数据：
```
$ caesiumclt -q 0 -e -o ~/output/ ~/image.jpg
```

将位于 `home` 目录中的 `Pictures` 文件夹和其子文件夹无损压缩到名为 `output` 的文件夹中：
```
$ caesiumclt -q 0 -R -o ~/output/ ~/Pictures
```

将位于 `home` 目录中的 `Pictures` 文件夹和其子文件夹无损压缩到名为 `output` 的文件夹中，且保留输入文件夹的结构：
```
$ caesiumclt -q 0 -RS -o ~/output/ ~/Pictures
```

----------

###### 变更日志
* 0.16.0-beta - Using libcaesium Rust library
* 0.15.2-beta - Fixed Windows -RS bug
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
