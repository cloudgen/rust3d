#!/bin/sh
# rust3d test runner. The suite locks process cwd and env, so it runs on one thread.
set -eu
ROOT="$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
cargo test -- --test-threads=1
