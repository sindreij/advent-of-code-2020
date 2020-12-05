#!/usr/bin/env bash
set -euo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR/.."

############################################################

TEST="./bin/test.sh"

echo "testing ..." && $TEST && echo "test ok." && \
git add . && git commit -m 'working' --quiet || \
git reset --hard --quiet
