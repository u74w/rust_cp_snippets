#!/bin/bash

RUST_BACKTRACE=1 cargo snippet -t vscode > rust.json || exit 1

cargo test

cp rust.json ~/.config/Code/User/snippets/rust.json

