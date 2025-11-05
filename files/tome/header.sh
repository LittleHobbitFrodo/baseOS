#!/bin/bash

function black() { echo -e "\033[0;30m$1\033[0m"; }
function red() { echo -e "\033[0;31m$1\033[0m"; }
function green() { echo -e "\033[0;32m$1\033[0m"; }
function yellow() { echo -e "\033[1;33m$1\033[0m"; }
function blue() { echo -e "\033[0;34m$1\033[0m"; }
function magenta() { echo -e "\033[0;35m$1\033[0m"; }
function cyan() { echo -e "\033[0;36m$1\033[0m"; }
function white() { echo -e "\033[0m$1\033[0m"; }

function error() { echo "$(red error): $1"; }
function warning() { echo "$(yellow warning): $1"; }
function note() { echo "$(magenta note): $1"; }

user_error=1
internal_error=2
spell_path="$PWD/files/tome/"
util_config="config/tome.toml"

function get_arch() {
    #   converts various architecture names into one used in the tome
    case $1 in
        x86_64)
            echo x86_64
        ;;
        arm64)
            echo aarch64
        ;;
        aarch64)
            echo aarch64
        ;;
        amd64)
            echo x86_64
        ;;
        x64)
            echo x86_64
        ;;
        *)
            echo ERR
        ;;
    esac
}

function search_path() {        #   searchs path for specific command
    #   returns "NONE" if not found
    local old_ifs="$IFS"
    IFS=":"
    for i in $PATH; do
        if [ -e "$i/$1" ]; then
            IFS="$old_ifs"
            echo "$i/$1"
            return
        fi
    done
    IFS="$old_ifs"
    echo NONE
}

function ask() {        #   asks user and returns if he agrees or not
    read -p "$1? [y/N]: " ans
    case "$ans" in
        [yY][eE][sS]|[yY]) echo y ;;
        *) echo n ;;
    esac
}

#   rebuilds the tome scripts
#   - return ERR on failure
function invoke_spells() {
    #   cargo needs to be in path

    local vocal=false

    if [ "$#" -ge 1 ]; then
        if [ "$1" == "-vocal" ]; then
            local vocal=true
            note "invoking spells"
        fi
    fi

    
    local dir="$PWD"
    cd "$spell_path"

    if [ ! -e "./commands/" ]; then mkdir ./commands; fi

    if ls ./commands/* >/dev/null 2>&1; then
        #   there is at least one file in ./commands/
        rm ./commands/*
    fi

    files="$(find "src/" -maxdepth 1 -type f -name "*.rs" ! -name 'main.rs')"

    IFS="
"
    for i in $files; do

        local name="$(basename "$i")"

        local name="${name%.*}"

        if [ "$vocal" == true ]; then
            note "invoking the $(blue "$name") spell"
        fi

        if [ "$name" == "conf" ]; then
            #   build the conf script with special feature
            output="$(cargo build --bin conf --color always --features "disable_config_preloading" 2>&1)"
        else
            output="$(cargo build --bin "$name" --color always 2>&1)"
        fi
        if [ "$?" != 0 ]; then
            error "failed to invoke spell \"$name\""
            echo -e "$output"
            exit $internal_error
        fi

        cp "target/debug/$name" ./commands/

    done

    cd $dir

    if [ "$vocal" == true ]; then
        note "invocation completed"
    fi

}

#   tries to install rustup
#   - exits on failure
function install_rustup() {

    #   check if cargo is not in path
    rustup="$(search_path rustup)"

    if [ "$rustup" = "NONE" ]; then

        error "$(blue rustup) (and $(blue cargo)) is not installed or added to $(red PATH)"
        note "$(blue cargo) is required for building the operating system and preparing util functionalities"
        note "if $(blue rustup) is installed, add it to $(red PATH)"

        install="$(ask "do you wish to proceed with installation")"

        if [ "$install" != 'y' ]; then
            error "cannot proceed without $(blue cargo)"
            exit 1
        fi

        note "installing $(blue rustup)"
        note "the $(blud rustup) installation will most likely need your attention"
        for i in {5..0}; do sleep 1; done

        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        if [ "$?" != 0 ]; then
            error "$(blue rustup) installation failed"
            exit 2
        fi

        note "you will propably have to add $(blue cargo) to your $(red PATH) environment variable"
        note "the rustup installation above should reveal all steps"

        if [ "$(ask "is $(blue rustup) in $(red PATH)")" != 'n' ]; then
            error "please add $(blue rustup) to your $(red PATH) environmental variable"
            exit $internal_error
        fi

    else
        if [ "$#" -ge 1 ]; then
            if [ "$1" = notify ]; then
                note "$(blue rustup) is already installed"
            fi
        fi
    fi

}

function setup() {
    install_rustup notify
    build_scripts -vocal
}

function configure() {

    if [ ! -e "$spell_path/commands/conf" ]; then
        error "unknown spell \"$subcmd\""
        exit $internal_error
    fi

    exec "$spell_path/commands/conf" $args
}
