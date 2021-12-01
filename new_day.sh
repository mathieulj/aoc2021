#!/bin/bash
set -exuo pipefail

NAME=$1

mkdir $NAME
mkdir $NAME/src

sed -i '' -e "/INSERT HERE/i \\
    \"$NAME\"," Cargo.toml

sed "s/name = .*/name = \"$NAME\"/" template/Cargo.toml > $NAME/Cargo.toml
sed "s/template::/$NAME::/" template/src/main.rs > $NAME/src/main.rs
sed "s/template::/$NAME::/" template/src/lib.rs > $NAME/src/lib.rs
touch $NAME/input.txt
