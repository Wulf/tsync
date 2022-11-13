#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

./file_input/tsync.sh
./directory_input/tsync.sh
./const/tsync.sh
./enum/tsync.sh