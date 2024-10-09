#!/bin/bash
set -Eeuo pipefail
VERSION=`cargo read-manifest | jq .version`
echo "Building release, version. $VERSION".
echo "Tagging..."
git tag v$VERSION
git push --tags
echo "Building release artefacts..."
cargo build --release
cross build --release --target x86_64-pc-windows-gnu
echo "Publishing docker image.."
docker build -t timcowlishaw/humus:latest timcowlishaw/humus:$VERSION
docker push timcowlishaw/humus:latest timcowlishaw/humus:$VERSION
echo "Publishing crate..."
cargo publish
echo "Done! Remember to create Github releases with the compiled artefacts"
