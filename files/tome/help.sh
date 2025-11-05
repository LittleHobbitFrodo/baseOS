#!/bin/bash

source "$PWD/files/tome/header.sh"

#   This file is the help menu

if [ "$#" -eq 0 ]; then
    #   arg count == 0 -> show quick help
    echo "The tome will help you cast spells that allow you to manage your operating system project"
    echo "  $(note "you can only cast spells in the root directory of your project")"
    echo
    echo "  The tome can cast several spells at once: $(green ./tome) $(yellow spellX) + $(yellow spellY)"
    echo
    echo "$(yellow "Inventory of spells")"
    echo "  $(blue invoke)      - invokes the project setup routine"
    echo "  $(blue forge)       - forges the kernel"
    echo "  $(blue rune)        - carves and seals the ISO rune"
    echo "  $(blue ignite)      - ignites the flame inside the sealed ISO rune"
    echo
    echo
    echo "If the winds of curiosity stir your soul, run $(green ./tome) $(yellow help) $(blue "<spell>"), and the answers shall be revealed"
    echo
    echo
    echo "example commands:"
    echo "  $(green ./tome) $(yellow invoke) $(blue "x86_64 -reconf")"
    echo "      the $(yellow invoke) spell will setup the project for the $(blue x86_64) target"
    echo "          the $(blue -reconf) option will possibly reconfigure the project"
    echo
    echo "  $(green ./tome) $(yellow "forge -vocal") + $(yellow rune) + $(blue "carve seal") + $(yellow ignite)"
    echo "      the $(yellow forge) spell forges the OS kernel (for the invoked x86_64 target)"
    echo "      $(yellow rune): $(blue carve) prepares the ISO structure"
    echo "              $(blue )"



    # echo "$(green ./util) is extensible suite of convenience tools to help you manage your OS project comfortably"
    # echo "      $(note "run the utility only in the $(red "root directory") of the project")"
    # echo
    # echo "$(yellow subcommands): (red ones are not yet supported)"
    # echo "  $(blue setup)                       - prepares the $(green util) for use"
    # echo "  $(blue conf) <arch/s>               - configures the project"
    # echo "  $(blue reconf)                      - reconfigures the project (using different tools, etc.)"
    # echo "  $(red build) <arch/s>              - build the OS for target architectures"
    # echo "  $(red run) <arch/s>                - runs the built operating system in an emulator"
    # echo "  $(red dep) install/remove          - installs or deletes the necessary software"
    # echo "  $(red change)                      - allows you to change project information"
    # echo "  $(red make) iso/image <arch/s>     - builds an ISO image or disk image"
    # echo "  $(red check) <options> <arch/s>    - checks whether there are any errors in any part of the project"
    # echo "  $(blue help) <subcommand/s>         - gives hints about subcommands"
    # echo
    # echo
    # echo "$(note "The util tool allows you to run several subcommands at once:")"
    # echo "  for example: $(green util) $(blue "conf x86_64") + $(blue "dep install all") will configure the project for the x86_64 platform and install dependencies"
else

    #   iterate through args (subcommands)
    if [ "$#" -eq 1 ]; then
        if [ "$1" == "all" ]; then
            args=("invoke" "forge" "ignite" "rune")
            all=true
        else
            args=("${@:1}")
        fi
    else
        args=("${@:1}")
    fi



    count="${#args[@]}"

    #   iterate through all parameters
    for ((i = 0; i < count; i++)); do
        spell="${args[$i]}"

        case "$spell" in

            invoke)
                echo "$(green invoke) <arch/s>"
                echo "      $(yellow "invokes the project setup routine")"
                echo "      $(blue "expects CPU architectures") to be supported by the OS project"
                echo "          supported CPU architectures: $(green x86_64) $(red arm64)"
                echo
                echo "      installs rust, builds all spells and configures the project for this host machine"
                echo
                echo "      options:"
                echo "          $(yellow -reconf)         - reconfigures the project"
                echo
                echo "      example:"
                echo "          $(green ./tome) $(yellow invoke) $(blue "x86_64") configures the project for the x86_64 target"
            ;;

            forge)
                echo "$(green forge) <arch/s>"
                echo "      $(yellow "forges the OS kernel")"
                echo "      if you do not select any architecture, all invoked architectures will be used"
                echo
                echo "      options:"
                echo "          $(yellow "-debug")          - forges the OS with debug options"
                echo "          $(yellow "-verbose")        - shows the output of the compiler (cargo)"
                echo
                echo "      examples:"
                echo "          $(green ./tome) $(yellow forge) $(blue "x86_64") forges the kernel for the x86_64 target"
                echo "          $(green ./tome) $(yellow forge) $(blue -debug) forges the kernel for all invoked targets in debug mode (see $(blue invoke))"
            ;;
            rune)
                echo "$(green rune) <rituals>"
                echo "      $(yellow "carves and seals the ISO rune") (creates ISO structure and builds the ISO)"
                echo "      if you do not select any architecture, all invoked architectures will be used"
                echo "      $(yellow "The order of rituals is important"): choosing the wrong order can have unexpected consequences"
                echo
                echo "      rituals:"
                echo "          $(blue carve)           - carves the ISO rune (creates the fs structure)"
                echo "          $(blue seal)            - seals the ISO rune"
                echo "              - carves the rune, if it has not already been carved"
                echo
                echo "      options:"
                echo "          $(yellow -debug)          - carves the ISO rune with debug config for the bootloader"
                echo "          $(yellow -remove)         - remove the old ISO frame"
                echo
                echo
                echo "      example:"
                echo "          $(green ./tome) $(yellow rune) $(blue "carve seal x86_64") creates the ISO structure and builds the ISO file for the x86_64 target"
            ;;
            ignite)

                echo "$(green ignite) <arch/s>"
                echo "      $(yellow "ignites the flame inside the sealed ISO rune") (runs the OS in the QEMU emulator)"
                echo "      if you do not select any architecture, all invoked architectures will be used"
                echo
                echo "      options:"
                echo "          $(yellow -debug)          - runs the emulator with debug configuration"
                echo
                echo "      example:"
                echo "          $(green ./tome) $(yellow ignite) tries to run the OS in emulator for all targets"

            ;;

            *)
                error " $(blue help) unknown spell $spell"
            ;;
        esac

        if ((i < count - 1)); then
            #   print one line
            printf '%*s\n' $(tput cols) '' | tr ' ' '-'
        fi
    done

fi