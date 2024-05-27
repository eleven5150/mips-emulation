# Base image
FROM ubuntu:16.04

# Set version and description of mips-none-elf-gcc
LABEL version="1.0.0" description="mips-none-elf toolchain"

# Set binutils, GCC and newlib versions
ARG BINUTILS_VERSION=2.28
ARG GCC_VERSION=6.3.0
ARG NEWLIB_VERSION=2.5.0.20170623

# Set build parameters
ARG TARGET=mips-none-elf
ARG PREFIX=/usr/local/$TARGET
ENV PATH=$PATH:$PREFIX/bin

WORKDIR /home

RUN apt-get update && apt-get upgrade -y && apt-get install bzip2 gcc gcc-multilib g++ make wget -y \
 && wget http://ftp.gnu.org/gnu/binutils/binutils-$BINUTILS_VERSION.tar.gz \
 && tar -xzf binutils-$BINUTILS_VERSION.tar.gz \
 && rm binutils-$BINUTILS_VERSION.tar.gz \
 && wget https://ftp.gnu.org/gnu/gcc/gcc-$GCC_VERSION/gcc-$GCC_VERSION.tar.gz \
 && tar -xzf gcc-$GCC_VERSION.tar.gz \
 && rm gcc-$GCC_VERSION.tar.gz \
 && wget ftp://sourceware.org/pub/newlib/newlib-$NEWLIB_VERSION.tar.gz \
 && tar -xzf newlib-$NEWLIB_VERSION.tar.gz \
 && rm newlib-$NEWLIB_VERSION.tar.gz \
 && mkdir build-binutils \
 && cd build-binutils \
 && ../binutils-$BINUTILS_VERSION/configure --target=$TARGET --prefix=$PREFIX \
 && make -j5 all \
 && make -j5 install \
 && cd ../gcc-$GCC_VERSION \
 && ./contrib/download_prerequisites \
 && mkdir ../build-gcc \
 && cd ../build-gcc \
 && ../gcc-$GCC_VERSION/configure --target=$TARGET --prefix=$PREFIX --without-headers --with-newlib  --with-gnu-as --with-gnu-ld \
 && make -j5 all-gcc \
 && make -j5 install-gcc \
 && mkdir ../build-newlib \
 && cd ../build-newlib \
 && ../newlib-$NEWLIB_VERSION/configure --target=$TARGET --prefix=$PREFIX \
 && make -j5 all \
 && make -j5 install \
 && cd ../build-gcc \
 && ../gcc-$GCC_VERSION/configure --target=$TARGET --prefix=$PREFIX --with-newlib --with-gnu-as --with-gnu-ld --disable-shared --disable-libssp \
 && make -j5 all \
 && make -j5 install \
 && cd .. \
 && rm -rf binutils-$BINUTILS_VERSION \
 && rm -rf gcc-$GCC_VERSION \
 && rm -rf newlib-$NEWLIB_VERSION \
 && rm -rf build-binutils \
 && rm -rf build-gcc \
 && rm -rf build-newlib \
 && apt-get purge --autoremove -qq \
    bzip2 \
    gcc \
    gcc-multilib \
    g++ \
    make \
    wget \
 && rm -rf /var/lib/apt/lists/*
