#!/bin/bash

apt-get update

apt-get install -y \
  cmake \
  apt-transport-https \

# we need LLVM >= 3.9 for onig_sys/bindgen
cat <<-EOF > /etc/apt/sources.list.d/llvm.list
deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-9 main
deb-src http://apt.llvm.org/xenial/ llvm-toolchain-xenial-9 main
EOF

wget -q https://apt.llvm.org/llvm-snapshot.gpg.key && apt-key add llvm-snapshot.gpg.key

apt-get update

apt-get dist-upgrade -y

# needed by onig_sys
apt-get install -y \
  libclang1-9 \
  llvm-9

apt install -y protobuf-compiler

apt-get -y install build-essential git clang cmake libclang-dev \
libsasl2-dev libstdc++-10-dev libssl-dev libxxhash-dev zlib1g-dev zlib1g

