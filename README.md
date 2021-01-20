# libskia ![](https://github.com/feenkcom/libskia/workflows/Cargo%20Build/badge.svg)
C api wrapper around [rust-skia](https://github.com/rust-skia/rust-skia)

# Building

## Ubuntu 20.04
Starting from a minimal installation of `Ubuntu 20.04` we should install a couple of packages:
```
sudo apt install curl git pkg-config libssl-dev clang llvm python2 python libfontconfig1-dev libgl1-mesa-dev
```
Next we need `Rust` which can be installed using `rustup`:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```
Now let's clone the repository and `cd` in the cloned folder:
```
git clone https://github.com/feenkcom/libskia.git && cd libskia
```
Once inside we can start the build using the following `cargo` command:
```
cargo build --release --features "skia_linux"
```
