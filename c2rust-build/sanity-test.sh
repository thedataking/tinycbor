#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd -P)"
SCRIPT_PAR="$(dirname "$SCRIPT_DIR")"
CBORDUMP="$SCRIPT_PAR/bin/cbordump"
CBORFILE="$SCRIPT_PAR/src/cborparser.c.cbor"
REFERENCE_OUTPUT=$(mktemp)
TRANSLATED_OUTPUT=$(mktemp)

type -P ${CBORDUMP} >/dev/null || {
    echo >&2 "$CBORDUMP not found. Build tinycbor and retry."; exit 1;
}


if [ ! -f ${CBORFILE} ]; then
    echo >&2 "$CBORFILE not found. Run transpiler and retry."; exit 1;
fi

# generate ref output
${CBORDUMP} -c ${CBORFILE} > ${REFERENCE_OUTPUT}
# generate test output; pipe rustc warnings to /dev/null
cargo run -- -c ${CBORFILE} > ${TRANSLATED_OUTPUT} 2>/dev/null

# compare
diff ${REFERENCE_OUTPUT} ${TRANSLATED_OUTPUT} || { echo "fail; files differ."; exit 1; }

echo >&2 "PASS"

# cleanup
rm -f ${REFERENCE_OUTPUT} ${TRANSLATED_OUTPUT}
