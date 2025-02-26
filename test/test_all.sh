#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

./directory_input/tsync.sh
./struct/tsync.sh
./type/tsync.sh
./const/tsync.sh
./const_enum_numeric/tsync.sh
./enum/tsync.sh
./enum_numeric/tsync.sh
./doc_comments/tsync.sh
./generic/tsync.sh
./issue-43/tsync.sh
./issue-55/tsync.sh
./issue-58/tsync.sh