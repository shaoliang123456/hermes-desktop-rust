#!/bin/bash
set -e

if [ -z "$1" ]; then
  echo "Usage: ./release.sh <version>"
  echo "  ./release.sh patch   # 0.5.2 → 0.5.3"
  echo "  ./release.sh minor   # 0.5.2 → 0.6.0"
  echo "  ./release.sh major   # 0.5.2 → 1.0.0"
  echo "  ./release.sh 1.2.3   # explicit version"
  exit 1
fi

VERSION_TYPE=$1

BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
  echo "Error: must be on main branch (current: $BRANCH)"
  exit 1
fi

if [ -n "$(git status --porcelain)" ]; then
  echo "Error: working tree not clean. Commit or stash changes first."
  git status --short
  exit 1
fi

OLD_VERSION=$(node -p "require('./package.json').version")
npm version "$VERSION_TYPE" -m "chore: release v%s"
NEW_VERSION=$(node -p "require('./package.json').version")

echo ""
echo "Releasing: v$OLD_VERSION → v$NEW_VERSION"
echo ""

git push origin main
git push origin "v$NEW_VERSION"

echo ""
echo "✓ Pushed v$NEW_VERSION. CI will build and create the Release."
echo "  https://github.com/shaoliang123456/hermes-desktop-rust/actions"
