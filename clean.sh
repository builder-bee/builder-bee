#!/bin/sh

rustup component add clippy
cargo clippy --fix -- -W clippy::pedantic
cargo fmt