# Publishing Cyanea crates to crates.io

crates.io has **no organizations and no namespaces** — crate names live in one
flat global namespace, and the `cyanea-*` prefix is our de-facto namespace.
This document covers how ownership and publishing are set up for the org.

## Crate inventory

| Group | Crates | Registry |
|-------|--------|----------|
| Published | `cyanea-core`, `cyanea-seq`, `cyanea-io`, `cyanea-align`, `cyanea-omics`, `cyanea-stats`, `cyanea-ml`, `cyanea-chem`, `cyanea-struct`, `cyanea-phylo`, `cyanea-gpu` | crates.io |
| Publishable, not yet published | `cyanea-meta`, `cyanea-epi`, `cyanea-proteomics`, `cyanea-network`, `cyanea-datasets` | crates.io |
| Bindings (`publish = false`) | `cyanea-py` → PyPI, `cyanea-wasm` → npm (`@cyanea/bio`) | not crates.io |
| Not published | `benchmarks` (`publish = false`) | — |

## 1. Team ownership (the "org" mechanism)

crates.io lets a **GitHub team own a crate**: everyone in the team gets
publish/yank rights. Team owners *cannot* add/remove owners, so keep at least
one individual owner too.

```bash
# One-time: create a `publishers` team under the cyanea-bio GitHub org, then:
cargo login                       # a crates.io token that already owns the crates
TEAM=publishers ./scripts/crates-io-add-owners.sh
```

crates.io needs the GitHub `read:org` scope to read team membership — if
`cargo owner --add` reports a permission error, re-authenticate on crates.io and
grant it.

## 2. Trusted Publishing (token-less releases from CI)

Releases run through [`.github/workflows/release.yml`](../.github/workflows/release.yml),
which uses **Trusted Publishing**: GitHub Actions authenticates to crates.io via
OIDC (`rust-lang/crates-io-auth-action`), so **no crates.io token is stored** in
repo secrets.

One-time crates.io setup, **per publishable crate**
(Settings → Trusted Publishing → Add a GitHub Actions publisher):

| Field | Value |
|-------|-------|
| Repository owner | `cyanea-bio` |
| Repository name | `labs` |
| Workflow filename | `release.yml` |
| Environment | `crates-io` |

Also create a GitHub Environment named `crates-io` (repo → Settings →
Environments) — optionally with required reviewers to gate publishing.

## 3. First-time catch-up (the 5 unpublished crates)

`cargo publish --workspace` refuses to re-publish an existing version, so it
cannot publish the 5 missing crates while the other 11 are already at `0.1.0`.
Two options:

- **Publish them at `0.1.0` now** (manual, one-time): they only depend on
  already-published crates, so any order works:
  ```bash
  cargo publish -p cyanea-meta -p cyanea-epi -p cyanea-proteomics \
                -p cyanea-network -p cyanea-datasets
  ```
- **Or** just cut the next release (below) — every publishable crate, including
  these 5, publishes at the new version.

Afterwards, add the team owner to them (see the tail of the owner script).

## 4. Cutting a release

```bash
# 1. Bump `version` under [workspace.package] in Cargo.toml (e.g. 0.1.0 -> 0.1.1)
# 2. Commit, then tag and push:
git tag v0.1.1
git push origin v0.1.1
```

The `release.yml` workflow then validates (version-matches-tag, tests, a
`--dry-run` of the whole workspace), publishes every publishable member in
dependency order via `cargo publish --workspace`, and creates a GitHub Release.
