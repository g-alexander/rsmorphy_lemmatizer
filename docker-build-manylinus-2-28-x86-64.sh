#!/bin/bash

docker pull quay.io/pypa/manylinux_2_28_x86_64
docker run --rm -v `pwd`:/io quay.io/pypa/manylinux_2_28_x86_64 bash /io/build-wheels_2_28.sh
