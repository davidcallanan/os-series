#!/bin/bash

BINUTILS_VERSION=$1

export PREFIX="/usr/local"
export TARGET=x86_64-elf
export PATH="$PREFIX/bin:$PATH"

cd $PREFIX/src
mkdir build-binutils
cd build-binutils
../binutils-${BINUTILS_VERSION}/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror
make -j `nproc`
make install

cd $PREFIX/src
rm -rf build-binutils.sh build-binutils binutils-${BINUTILS_VERSION}
