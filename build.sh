#!/usr/bin/env bash

apt update
apt install -y make curl gcc libssl-dev pkg-config

# node
curl -sL https://deb.nodesource.com/setup_12.x | bash -
apt install -y nodejs
# rust
curl https://sh.rustup.rs -sSf | sh -s -- -y
. ~/.cargo/env

make
