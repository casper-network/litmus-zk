#!/usr/bin/env bash

# A type of actor representing a participating node.
declare _ELF_NAME="litmus-riscv32im-zkvm-sp1-elf"

# Set path -> output directory.
declare _ELF_OUTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"/elf

# Set path -> SP1 toolchain bin.
declare _SP1_BIN=$HOME/.sp1/bin1

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
    if [ -d "$_SP1_BIN" ]; then
        # Activate sp1 toolchain.
        export PATH="$PATH:$_SP1_BIN"

        # Build elf.
        cargo prove build \
            --elf-name "$_ELF_NAME" \
            --output-directory "$_ELF_OUTDIR"
    else
        echo "ERROR :: SP1 toolchain is not installed."
    fi
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
