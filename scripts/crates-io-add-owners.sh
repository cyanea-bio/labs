#!/usr/bin/env bash
#
# Add a GitHub team from the cyanea-bio org as an owner of every publishable
# Cyanea crate on crates.io, so publish/yank rights are managed centrally
# through the org instead of a single account.
#
# Background: crates.io has no organizations or namespaces. The closest thing
# is *team ownership* — a GitHub team can own a crate, and everyone GitHub
# recognizes as a member gets publish/yank rights. Team owners cannot add or
# remove owners (only individual "named" owners can), so keep at least one
# individual owner as well.
#
# Prerequisites:
#   1. Create the team on GitHub under the cyanea-bio org (default: "publishers").
#   2. `cargo login` with a crates.io token whose account already owns the
#      crates, and re-authenticate on crates.io granting the `read:org` GitHub
#      scope (needed so crates.io can read team membership).
#   3. A crate must already be published — `cargo owner --add` fails otherwise.
#
# Usage:
#   TEAM=publishers ./scripts/crates-io-add-owners.sh          # apply
#   DRY_RUN=1 TEAM=publishers ./scripts/crates-io-add-owners.sh # print only

set -euo pipefail

ORG="cyanea-bio"
TEAM="${TEAM:-publishers}"
OWNER="github:${ORG}:${TEAM}"
DRY_RUN="${DRY_RUN:-0}"

# Crates published to crates.io. (cyanea-py -> PyPI and cyanea-wasm -> npm are
# `publish = false` and intentionally not on crates.io.)
PUBLISHED=(
  cyanea-core
  cyanea-seq
  cyanea-stats
  cyanea-ml
  cyanea-chem
  cyanea-struct
  cyanea-align
  cyanea-gpu
  cyanea-phylo
  cyanea-omics
  cyanea-io
)

# Publishable but not yet on crates.io — add the team to these *after* their
# first publish (see docs/PUBLISHING.md).
UNPUBLISHED=(
  cyanea-meta
  cyanea-epi
  cyanea-proteomics
  cyanea-network
  cyanea-datasets
)

echo "Adding owner ${OWNER} to ${#PUBLISHED[@]} published crates"
for crate in "${PUBLISHED[@]}"; do
  if [ "${DRY_RUN}" = "1" ]; then
    echo "  cargo owner --add ${OWNER} ${crate}"
  else
    echo "==> ${crate}"
    cargo owner --add "${OWNER}" "${crate}"
  fi
done

echo
echo "Once these are published on crates.io, add the team to them too:"
for crate in "${UNPUBLISHED[@]}"; do
  echo "  cargo owner --add ${OWNER} ${crate}"
done
