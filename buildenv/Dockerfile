FROM debian:12

# STEP 1: Set up gcc-cross-x86_64-elf
# Adapted from https://github.com/randomdude/gcc-cross-x86_64-elf/blob/master/Dockerfile

ARG BINUTILS_VERSION=2.41
ARG GCC_VERSION=13.2.0

RUN set -x \
	&& apt-get update \
	&& apt-get install -y wget gcc libgmp3-dev libmpfr-dev libisl-dev libmpc-dev texinfo bison flex make bzip2 patch build-essential

RUN set -x \
	&& mkdir -p /usr/local/src \
	&& cd /usr/local/src \
	&& wget -q https://ftpmirror.gnu.org/gnu/binutils/binutils-${BINUTILS_VERSION}.tar.gz \
	&& wget -q https://ftpmirror.gnu.org/gnu/gcc/gcc-${GCC_VERSION}/gcc-${GCC_VERSION}.tar.gz \
	&& tar zxf binutils-${BINUTILS_VERSION}.tar.gz \
	&& tar zxf gcc-${GCC_VERSION}.tar.gz \
	&& rm binutils-${BINUTILS_VERSION}.tar.gz gcc-${GCC_VERSION}.tar.gz \
	&& chown -R root:root binutils-${BINUTILS_VERSION} \
	&& chown -R root:root gcc-${GCC_VERSION} \
	&& chmod -R o-w,g+w binutils-${BINUTILS_VERSION} \
	&& chmod -R o-w,g+w gcc-${GCC_VERSION}

COPY src /usr/local/src/

COPY gcc/t-x86_64-elf /usr/local/src/gcc-${GCC_VERSION}/gcc/config/i386/
COPY gcc/config.gcc.patch /usr/local/src/gcc-${GCC_VERSION}/gcc/

RUN set -x \
	&& cd /usr/local/src \
	&& bash build-binutils.sh ${BINUTILS_VERSION} \
	&& bash build-gcc.sh ${GCC_VERSION}
	
# STEP 2: Set up build environment

RUN apt-get update 
RUN apt-get upgrade -y
RUN apt-get install -y nasm
RUN apt-get install -y xorriso
RUN apt-get install -y grub-pc-bin
RUN apt-get install -y grub-common

VOLUME /root/env
WORKDIR /root/env
