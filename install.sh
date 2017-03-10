#!/bin/sh

#libcaesium
git clone https://github.com/Lymphatus/libcaesium.git
cd libcaesium
sudo chmod +x install.sh
./install.sh
mkdir build
cd build
cmake ..
make
sudo make install