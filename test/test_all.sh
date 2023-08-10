#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

./directory_input/tsync.sh
./struct/tsync.sh
./type/tsync.sh
./const/tsync.sh
./enum/tsync.sh
./doc_comments/tsync.sh
