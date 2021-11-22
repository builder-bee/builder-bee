#!/bin/sh

rustup component add clippy
rustup run nightly cargo clippy --fix
rustup run nightly cargo fmt