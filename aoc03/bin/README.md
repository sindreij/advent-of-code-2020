# Scripts

Scripts are in their own folder to keep a clean root.

## Shared header

The scripts share this header:

```shell

#!/usr/bin/env bash
set -euo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR/.."

############################################################
```

The headers ensures:

1. Immediate stop on error
2. All scripts are run from the root directory. This means we can do ../bin/tcr.sh from inside the
   src/ directory, and it works.
