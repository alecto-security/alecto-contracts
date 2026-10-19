#!/bin/sh
# Clone the evidence-bundle standard at the pinned tag and expose its
# fixture vectors for the mirror tests. The pin lives in
# vectors/evidence-bundle.tag; bumping it requires a PR because the mirror
# rule says the types and the vectors move together, on purpose.
set -e
TAG=$(tr -d '[:space:]' < vectors/evidence-bundle.tag)
if [ -d .vectors ]; then
  echo ".vectors already present; remove it to re-fetch"
  exit 0
fi
git clone --quiet --depth 1 --branch "$TAG" \
  https://github.com/alecto-security/evidence-bundle .vectors
VECTORS_DIR="$(pwd)/.vectors/fixtures"
export VECTORS_DIR
echo "vectors: evidence-bundle@$TAG at $VECTORS_DIR"
