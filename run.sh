#!/usr/bin/env bash
SCRIPT_DIR="$( cd "$( dirname "$(readlink -f "${BASH_SOURCE[0]}")" )" >/dev/null 2>&1 && pwd )"

"$SCRIPT_DIR/target/release/rs-temp" "$@"