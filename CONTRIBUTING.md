# Contributing

## Development setup

This is a Rust crate with no non-Rust dependencies. Clone the repo and build:

```bash
git clone https://github.com/pilgrimagesoftware/fulltime-plugin-api.git
cd fulltime-plugin-api
cargo build
```

The toolchain is pinned in `rust-toolchain.toml`; `rustup` picks it up automatically.

Before opening a PR, run what CI runs:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features --workspace
cargo test --no-default-features
cargo test --doc
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## Branching and PRs

This repo follows the standard `master`/`develop` git-flow:

- `master` reflects the latest released version; nothing is committed here directly.
- `develop` is the integration branch.
  Branch `feature/*` or `fix/*` off `develop`, and open your PR back into `develop`.
- `release/*` branches are cut automatically by the release workflow (see [RELEASING.md](RELEASING.md)).
  You shouldn't need to create one by hand.

CI (`ci.yaml`) runs `fmt`, `clippy`, `test`, and `doc` on every PR; all must pass before merging.

## Commit messages

Commits follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (`feat:`, `fix:`, `chore:`, `docs:`, etc.).
This isn't enforced by CI today, but it's what `git-cliff` reads to generate the changelog and compute the next version on release.
A commit that doesn't follow the convention won't show up in the changelog and won't contribute to the version bump.

## Changing the WIT contract

`wit/data-provider.wit` is the source of truth for the canonical schema and the `data-provider` interface.
The Rust types in `src/bindings.rs` are generated from it via `wit-bindgen`, not hand-written.
If you change a record or function signature there, update the matching requirement/scenario in `openspec/changes/define-league-data-contract/specs/` (or a new OpenSpec change, if the original change has already been archived), and bump `SCHEMA_VERSION`/`INTERFACE_VERSION` in `src/lib.rs` per the versioning policy in [`docs/plugin-authoring.md`](docs/plugin-authoring.md#versioning).
