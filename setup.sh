#!/bin/bash

set -e

echo "update apt source..."
sudo apt update

echo "install some tool..."
sudo apt install -y curl wget python3 python3-pip net-tools build-essential iproute2 git cmake libssl-dev iputils-ping iptables

echo "install Rust..."
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y

source "$HOME/.cargo/env"

echo " emp-toolkit .."
wget https://raw.githubusercontent.com/emp-toolkit/emp-readme/master/scripts/install.py

echo " emp-toolkit"
python3 install.py --install --tool --ot

echo "thfhe "
git clone --branch thfhe https://github.com/primus-labs/primus-fhe.git
git clone https://github.com/Atbnsanw/somesh.git
echo "finish"