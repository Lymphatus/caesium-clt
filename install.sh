#!/bin/sh

#libcaesium
git clone https://github.com/Lymphatus/libcaesium.git
cd libcaesium || exit
sudo chmod +x install.sh
./install.sh
mkdir build
cd build || exit
cmake ..
make
sudo make install