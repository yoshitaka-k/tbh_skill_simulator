#!/bin/sh

# Windows（MinGW が必要。CI では Ubuntu 上でクロスコンパイル）
cargo bundle --release --target x86_64-pc-windows-gnu -f exe

# Linux
cargo bundle --release -f deb

# インストール
sudo dpkg -i target/release/bundle/deb/tbh-skill-simulator_*.deb

exit
