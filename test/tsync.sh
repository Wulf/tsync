#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cargo run -- -i "$SCRIPT_DIR/rust.rs" -o "$SCRIPT_DIR/typescript.d.ts"