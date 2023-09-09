#!/bin/bash

sudo docker pull quay.io/pypa/manylinux2014_x86_64
sudo docker run --rm -v `pwd`:/io quay.io/pypa/manylinux2014_x86_64 bash /io/build-wheels.sh
