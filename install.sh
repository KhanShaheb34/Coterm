#!/bin/bash

set -u

CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed -e 's/version = "//' -e 's/"$//')
UPDATE_ROOT="https://github.com/KhanShaheb34/Coterm/releases/download/v${CURRENT_VERSION}"

main() {
    downloader --check
    need_cmd uname
    need_cmd mktemp
    need_cmd chmod
    need_cmd mkdir
    need_cmd rm
    need_cmd rmdir
    need_cmd tar
    need_cmd which
    need_cmd dirname

    get_architecture || return 1
    local _arch="$RETVAL"
    assert_nz "$_arch" "arch"

    local _tardir="coterm-v${CURRENT_VERSION}-${_arch}"
    local _url="$UPDATE_ROOT/${_tardir}.tar.gz"
    echo "Downloading Coterm from $_url"
    local _dir="$(mktemp -d 2>/dev/null || ensure mktemp -d -t coterm)"
    local _file="$_dir/input.tar.gz"
    local _coterm="$_dir/coterm"
    local _path="/usr/local/bin/coterm"
    local _shortpath="/usr/local/bin/ct"

    printf '%s\n' 'info: downloading Coterm' 1>&2

    ensure mkdir -p "$_dir"
    downloader "$_url" "$_file"
    if [ $? != 0 ]; then
      say "failed to download $_url"
      say "this may be a standard network error, but it may also indicate"
      say "that Coterm's release process is not working. When in doubt"
      say "please feel free to open an issue!"
      exit 1
    fi
    ensure tar xf "$_file" --strip-components 1 -C "$_dir"
    echo "Copying Coterm to $_path and $_shortpath"
    ensure sudo cp "$_coterm" "$_path"
    ensure chmod +x "$_path"
    ensure sudo cp "$_coterm" "$_shortpath"
    ensure chmod +x "$_shortpath"

    ignore rm -rf "$_dir"
}

get_architecture() {
    local _ostype="$(uname -s)"
    local _cputype="$(uname -m)"

    set +u
    if [ -n "$TARGETOS" ]; then
        _ostype="$TARGETOS"
    fi

    if [ -n "$TARGETARCH" ]; then
        _cputype="$TARGETARCH"
    fi
    set -u


    if [ "$_ostype" = Darwin -a "$_cputype" = i386 ]; then
        if sysctl hw.optional.x86_64 | grep -q ': 1'; then
            local _cputype=x86_64
        fi
    fi

    case "$_ostype" in
        Linux | linux)
            local _ostype=unknown-linux-musl
            ;;

        Darwin)
            local _ostype=apple-darwin
            ;;

        MINGW* | MSYS* | CYGWIN*)
            local _ostype=pc-windows-msvc
            ;;

        *)
            err "no precompiled binaries available for OS: $_ostype"
            ;;
    esac

    case "$_cputype" in
        x86_64 | x86-64 | x64 | amd64)
            local _cputype=x86_64
            ;;
        arm64 | aarch64)
            local _cputype=aarch64
            ;;
        *)
            err "no precompiled binaries available for CPU architecture: $_cputype"

    esac

    local _arch="$_cputype-$_ostype"

    RETVAL="$_arch"
}

say() {
    echo "Coterm-init: $1"
}

err() {
    say "$1" >&2
    exit 1
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

need_ok() {
    if [ $? != 0 ]; then err "$1"; fi
}

assert_nz() {
    if [ -z "$1" ]; then err "assert_nz $2"; fi
}


ensure() {
    "$@"
    need_ok "command failed: $*"
}


ignore() {
    "$@"
}


downloader() {
    if check_cmd curl
    then _dld=curl
    elif check_cmd wget
    then _dld=wget
    else _dld='curl or wget' 
    fi

    if [ "$1" = --check ]
    then need_cmd "$_dld"
    elif [ "$_dld" = curl ]
    then curl -sSfL "$1" -o "$2"
    elif [ "$_dld" = wget ]
    then wget "$1" -O "$2"
    else err "Unknown downloader"
    fi
}

main "$@" || exit 1
