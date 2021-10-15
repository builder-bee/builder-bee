#!/bin/sh

rustup component add clippy
cargo clippy --fix
cargo fmt