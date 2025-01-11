#!/usr/bin/env bash

function _help() {
    echo "
    DESCRIPTION
    ----------------------------------------------------------------
    Compiles litmus program for execution within SP1 ZK-VM.

    NOTES
    ----------------------------------------------------------------
    Assumes that sp1 toolchain is installed and is available at '$HOME/.sp1'.
    "
}

function _main()
{
    # Activate sp1 toolchain.
    export PATH="$PATH:$HOME/.sp1/bin"

    # Build elf.
    cargo prove build \
        --elf-name litmus-riscv32im-zkvm-sp1-elf \
        --output-directory pgms/sp1/elf
}

# ----------------------------------------------------------------
# ENTRY POINT
# ----------------------------------------------------------------

unset _HELP

for ARGUMENT in "$@"
do
    KEY=$(echo "$ARGUMENT" | cut -f1 -d=)
    VALUE=$(echo "$ARGUMENT" | cut -f2 -d=)
    case "$KEY" in
        help) _HELP="show" ;;
        *)
    esac
done

if [ "${_HELP:-""}" = "show" ]; then
    _help
else
    _main
fi
