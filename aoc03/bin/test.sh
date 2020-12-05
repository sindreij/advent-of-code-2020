#!/usr/bin/env bash
set -euo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR/.."

############################################################

TYPECHECK="bin/typecheck.sh"
UNITTEST="bin/unittest.sh"

$UNITTEST
# $TYPECHECK
