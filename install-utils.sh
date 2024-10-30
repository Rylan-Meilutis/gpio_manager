#!/bin/bash
sudo apt-get update && sudo apt-get install -y cmake device-tree-compiler libfdt-dev
git clone https://github.com/raspberrypi/utils.git && cd utils || exit
cmake .
make -j "$(nproc)"
sudo make install
cd .. && rm -rf utils