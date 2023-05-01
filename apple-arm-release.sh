#!/bin/bash

set -u

CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed -e 's/version = "//' -e 's/"$//')

main() {
    need_cmd mkdir
    need_cmd rm
    need_cmd tar
    need_cmd cargo

    cargo build --release

    local _name="coterm-v${CURRENT_VERSION}-aarch64-apple-darwin"
    mkdir $_name
    cp target/release/coterm $_name
    cp README.md $_name
    cp LICENSE $_name

    tar -czvf $_name.tar.gz $_name
    rm -rf $_name

    gh release upload v${CURRENT_VERSION} $_name.tar.gz
    rm $_name.tar.gz

    echo "Done!"
}

need_cmd() {
    if ! check_cmd "$1"
    then err "need '$1' (command not found)"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
    return $?
}

main || exit 1
