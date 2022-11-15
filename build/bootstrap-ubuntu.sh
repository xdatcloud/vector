#!/bin/bash

sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list

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

# needed by onig_sys
apt-get install -y \
  libclang1-9 \
  llvm-9

cat <<-EOF > "$CARGO_HOME"/config.toml
[source.crates-io]
replace-with = "tuna"

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
EOF
