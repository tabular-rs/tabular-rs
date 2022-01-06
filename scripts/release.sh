#!/bin/bash

# Prerequisites: `cargo release` installed locally.
#
# To perform a dry-run, run:
#    ./scripts/release.sh VERSION_NUMBER
# To perform the release:
#    ./scripts/release.sh VERSION_NUMBER --execute
#
# Arguments are passed into cargo-release.

set -e -o pipefail

VERSION_NUMBER="$1"
if [[ -z "$VERSION_NUMBER" ]]; then
    >&2 echo "error: version number must be specified as the first argument"
    exit 1
fi

shift
CARGO_RELEASE_ARGS=("$@")

if !(git diff --quiet); then
    >&2 echo "error: uncommitted changes in repo"
    exit 1
fi

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Add a link to the latest version to the changelog.
TAG_LINK_PREFIX="https://github.com/tov/tabular-rs/releases/tag"
TAG_LINK_LINE="[$VERSION_NUMBER]: $TAG_LINK_PREFIX/$VERSION_NUMBER"

if !(grep --fixed-strings --quiet "$TAG_LINK_LINE" CHANGELOG.md); then
    echo "$TAG_LINK_LINE" >> CHANGELOG.md
    git add CHANGELOG.md
    git commit -m "Add release link to CHANGELOG."
fi

# Run cargo release.
cargo release "$VERSION_NUMBER" "${CARGO_RELEASE_ARGS[@]}"
