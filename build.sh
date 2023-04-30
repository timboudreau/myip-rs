#!/bin/sh

cargo clean
HOST=`rustc -vV | grep 'host:' | cut -b7-`

RUSTFLAGS='-C target-cpu=native' cargo build -Z build-std=std,panic_abort --target ${HOST} --release
