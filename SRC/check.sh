#!/usr/bin/env bash
# Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
# MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
# Every check this repository owns, in one command:
#   bash SRC/check.sh
# The worker and the parser crate against the published SDK tag and the pinned
# git sources in Cargo.lock (a first run fetches them from GitHub), then the
# Python edge's own tests when pytest is installed (Tests/conftest.py finds
# apk_wireless_import in the sibling DATATYPE.wireless.CSV checkout).
set -euo pipefail
cd "$(dirname "$0")"
cargo test --workspace --locked
if python3 -c "import pytest" 2>/dev/null; then
  (cd "../IMPORT.file.PDF" && python3 -m pytest -q Tests)
else
  echo "check.sh: pytest not installed — Python edge not run" >&2
fi
