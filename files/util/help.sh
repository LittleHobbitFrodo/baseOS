#!/bin/bash

source "$PWD/files/util/header.sh"

#   This file is the help menu

echo 

if [ "$#" -eq 0 ]; then
    #   arg count == 0 -> show quick help
    echo "$(green ./util) is extensible suite of convenience tools to help you manage your OS project comfortably"
    echo "      $(note "run the utility only in the $(red "root directory") of the project")"
    echo
    echo "$(yellow subcommands): (red ones are not yet supported)"
    echo "  $(blue setup)                       - prepares the $(green util) for use"
    echo "  $(blue conf) <arch/s>               - configures the project"
    echo "  $(blue reconf)                      - reconfigures the project (using different tools, etc.)"
    echo "  $(red build) <arch/s>              - build the OS for target architectures"
    echo "  $(red run) <arch/s>                - runs the built operating system in an emulator"
    echo "  $(red dep) install/remove          - installs or deletes the necessary software"
    echo "  $(red change)                      - allows you to change project information"
    echo "  $(red make) iso/image <arch/s>     - builds an ISO image or disk image"
    echo "  $(red check) <options> <arch/s>    - checks whether there are any errors in any part of the project"
    echo "  $(blue help) <subcommand/s>         - gives hints about subcommands"
    echo
    echo
    echo "$(note "The util tool allows you to run several subcommands at once:")"
    echo "  for example: $(green util) $(blue "conf x86_64") + $(blue "dep install all") will configure the project for the x86_64 platform and install dependencies"
else
    #   iterate through args (subcommands)
    args="${@:1}"
    for subcmd in $args; do
        case "$subcmd" in

            conf)
                echo CONF - help
            ;;
            forge)
                echo FORGE - help
            ;;
            *)
                error "$(blue help) unknown subcommand \"$subcmd\""
            ;;
        esac
    done

fi