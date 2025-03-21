#!/bin/bash

cmake -S . -B build -G Ninja
ninja -C build
./build/anl $@
