# libskia ![](https://github.com/feenkcom/libskia/workflows/Cargo%20Build/badge.svg)
C-style wrapper around [rust-skia](https://github.com/rust-skia/rust-skia)

# Building

## Ubuntu 18.04
```
sudo apt install git curl clang pkg-config libssl-dev libgl1-mesa-dev libfontconfig1-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

git clone https://github.com/feenkcom/libskia.git && cd libskia

cargo build --release
```
