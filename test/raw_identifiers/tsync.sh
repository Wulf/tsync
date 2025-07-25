#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

cargo run -- -i rust.rs -o typescript.d.ts
cargo run -- -i rust.rs -o typescript.ts
