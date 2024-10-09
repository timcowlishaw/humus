#!/bin/bash
set -Eeuo pipefail
echo "Checking git committed and pushed..."
test -z "$(git status --porcelain)"
git merge-base --is-ancestor HEAD @{u}
VERSION=`cargo read-manifest | jq -r .version`
echo "Building release, version. $VERSION".
echo "Tagging..."
git tag v$VERSION
git push --tags
echo "Building release artefacts..."
cargo build --release
cross build --release --target x86_64-pc-windows-gnu
echo "Publishing docker image.."
docker build -t timcowlishaw/humus:latest -t timcowlishaw/humus:$VERSION .
docker push timcowlishaw/humus --all-tags
echo "Publishing crate..."
cargo publish --dry-run
echo "Done! Remember to create Github releases with the compiled artefacts"
