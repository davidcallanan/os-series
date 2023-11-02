FROM debian:latest

SHELL ["/bin/bash", "-c"]
CMD ["/bin/bash"]

ARG BINUTILS_VERSION=2.41
ARG GCC_VERSION=13.2.0

RUN echo "deb http://security.debian.org/debian-security bullseye-security main contrib non-free" > /etc/apt/sources.list

RUN set -x \
    && apt-get update \
    && apt-get install -y curl build-essential nasm xorriso grub-pc-bin grub-common make wget gcc libgmp3-dev libmpfr-dev libisl-dev libmpc-dev texinfo bison flex make bzip2 patch

# Pull binutils and gcc source code
RUN set -x \
    && mkdir -p /usr/local/src \
    && cd /usr/local/src \
    && wget -q https://ftp.gnu.org/gnu/binutils/binutils-${BINUTILS_VERSION}.tar.gz \
    && wget -q https://ftp.gnu.org/gnu/gcc/gcc-${GCC_VERSION}/gcc-${GCC_VERSION}.tar.gz \
    && tar zxf binutils-${BINUTILS_VERSION}.tar.gz \
    && tar zxf gcc-${GCC_VERSION}.tar.gz \
    && rm binutils-${BINUTILS_VERSION}.tar.gz gcc-${GCC_VERSION}.tar.gz \
    && chown -R root:root binutils-${BINUTILS_VERSION} \
    && chown -R root:root gcc-${GCC_VERSION} \
    && chmod -R o-w,g+w binutils-${BINUTILS_VERSION} \
    && chmod -R o-w,g+w gcc-${GCC_VERSION}

# Copy compile scripts
COPY files/src /usr/local/src/

# Copy gcc patches
COPY files/gcc/t-x86_64-elf /usr/local/src/gcc-${GCC_VERSION}/gcc/config/i386/
COPY files/gcc/config.gcc.patch /usr/local/src/gcc-${GCC_VERSION}/gcc/

# Build and install binutils and the cross-compiler
RUN set -x \
    && cd /usr/local/src \
    && ./build-binutils.sh ${BINUTILS_VERSION} \
    && ./build-gcc.sh ${GCC_VERSION}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --target x86_64-unknown-none --profile complete -y

RUN source "$HOME/.cargo/env" \
    rustup target add x86_64-unknown-none

VOLUME /root/env
WORKDIR /root/env