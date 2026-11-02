#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
CKB_SOURCE_ROOT="${CKB_SOURCE_ROOT:-/workspace/wo01-sources/ckb}"
WO04_TMP="${WO04_TMP:-/workspace/wo04-fixture-db}"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/wo04-target}"
if [[ -d "$CARGO_TARGET_DIR/debug" ]]; then MIN_TMP_FREE_BYTES=$((4 * 1024 * 1024 * 1024)); else MIN_TMP_FREE_BYTES=$((12 * 1024 * 1024 * 1024)); fi
MIN_DATA_FREE_BYTES=$((4 * 1024 * 1024 * 1024))

if [[ ! -f "$CKB_SOURCE_ROOT/Cargo.toml" ]]; then
  echo "Set CKB_SOURCE_ROOT to the CKB source checkout at a4aa1286a45bc1d42c30f6698dc51521c3749b40" >&2
  exit 2
fi
SOURCE_COMMIT="$(git -C "$CKB_SOURCE_ROOT" rev-parse HEAD)"
if [[ "$SOURCE_COMMIT" != "a4aa1286a45bc1d42c30f6698dc51521c3749b40" ]]; then
  echo "Unexpected CKB source commit: $SOURCE_COMMIT" >&2
  exit 2
fi
TMP_AVAILABLE_BYTES="$(df -PB1 /tmp | awk 'NR==2 {print $4}')"
mkdir -p "$(dirname "$WO04_TMP")"
DATA_AVAILABLE_BYTES="$(df -PB1 "$(dirname "$WO04_TMP")" | awk 'NR==2 {print $4}')"
if (( TMP_AVAILABLE_BYTES < MIN_TMP_FREE_BYTES )); then echo "Insufficient /tmp free space for pinned CKB build while retaining 4 GiB" >&2; exit 2; fi
if (( DATA_AVAILABLE_BYTES < MIN_DATA_FREE_BYTES )); then
  echo "Need at least 4 GiB free on the fixture filesystem; found $((DATA_AVAILABLE_BYTES / 1024 / 1024 / 1024)) GiB" >&2
  exit 2
fi

EXAMPLE="$CKB_SOURCE_ROOT/store/examples/wo04_fixture.rs"
if [[ -e "$EXAMPLE" ]]; then
  echo "Refusing to overwrite existing source file: $EXAMPLE" >&2
  exit 2
fi
mkdir -p "$(dirname "$EXAMPLE")" "$(dirname "$REPO_ROOT/evidence/wo04/store-fixture.csv")"
cp "$REPO_ROOT/experiments/wo04/fixture.rs" "$EXAMPLE"
cleanup() { rm -f "$EXAMPLE"; rmdir "$(dirname "$EXAMPLE")" 2>/dev/null || true; }
trap cleanup EXIT

cd "$REPO_ROOT"
CARGO_TARGET_DIR="$CARGO_TARGET_DIR" cargo run --manifest-path "$CKB_SOURCE_ROOT/Cargo.toml" -p ckb-store --example wo04_fixture --offline -- "$WO04_TMP" > "$REPO_ROOT/evidence/wo04/store-fixture.csv"
AFTER_BYTES="$(df -PB1 "$(dirname "$WO04_TMP")" | awk 'NR==2 {print $4}')"
if (( AFTER_BYTES < 4 * 1024 * 1024 * 1024 )); then
  echo "Post-run free space fell below 4 GiB" >&2
  exit 3
fi
