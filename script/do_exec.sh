# Set path -> output directory.
declare _HERE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Set path -> env.toml.
declare _CFG="$_HERE"/env.toml


function _help() {
    echo "
    DESCRIPTION
    ----------------------------------------------------------------
    Executes litmus kernel within SP1 ZK-VM.

    NOTES
    ----------------------------------------------------------------
    Assumes that local env.toml is available at '$_HERE/env.toml'.
    "
}

function _main()
{
    if [ -f "$_CFG" ]; then
        RUST_BACKTRACE=full
        RUST_LOG=info
        cargo run --release -- --execute --path-to-config "$_CFG"
    else
        echo "ERROR :: env.toml is not found -> '$_CFG'."
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
