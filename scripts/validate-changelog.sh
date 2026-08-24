#!/usr/bin/env bash
# validate-changelog.sh — 检查 CHANGELOG.md 中是否存在指定版本的条目
set -euo pipefail

VERSION="${1:?Usage: $0 <version>}"

if ! grep -qE "## \[${VERSION}\]" CHANGELOG.md; then
  echo "CHANGELOG.md missing entry for version ${VERSION}"
  exit 1
fi

echo "✓ CHANGELOG.md contains entry for version ${VERSION}"
