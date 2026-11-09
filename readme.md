# `bake-readme`

Reusable `readme.md` maintenance tasks for Bake. `readme:update` keeps the short release summary and package links in a project's `readme.md` up to date.

## Motivation

A project's `readme.md` should help people quickly understand what the project does, why it exists, and where to start. Release summaries and package links are useful, but they are easy to forget or leave stale when maintained by hand. Bake Readme refreshes those derived sections while leaving the project's explanation and examples under its authors' control.

## Usage

Add this crate to the project's private Bake task executable and regenerate its task links:

```sh
cargo bake --regenerate
cargo add --manifest-path bake/Cargo.toml bake-readme
cargo bake --regenerate
```

Then run the task from the project root:

```sh
cargo bake readme:update
```

If `readme.md` has no authored `Releases` section, the task adds one with the three latest versioned entries from `releases.md`. It skips `Unreleased` and links to the full release history. The task also adds or refreshes a package entry in `See Also` based on the root `Cargo.toml` name, description, and repository URL. Existing authored sections and entries are preserved. The task writes the whole readme as `cargo bake markdown:normalize` would, so the two tasks never rewrite each other's output.

A custom path is supported with `--path PATH`.

## Releasing

Prepare a release with `cargo bake cargo:release:patch` (or `minor`, `major`, or `bump --version X.Y.Z`) from a clean checkout of `main`; it bumps the version, commits with the `Release: vVERSION` trailer, validates the candidate, and opens the pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it, then creates or updates the matching GitHub Release from `releases.md`. Follow the shared [Releasing skill](https://github.com/base-rs/base-project/blob/main/context/releasing.md) for the standard process.

## Releases

<!-- bake-readme:releases:start -->

See [releases.md](releases.md) for the full release history.

### v0.3.2

- Require `bake` 0.20 or later, instead of 0.18 or later, so that Cargo resolves this crate to the same `bake` as a task package that requires `bake` 0.20.2. In a workspace whose `rust-version` is below the 1.88 that `bake` 0.20.2 declares, Cargo resolved the old requirement to `bake` 0.19.2, which declares none, and the task package failed to compile against two `bake` versions.

### v0.3.1

- Declare Rust 1.88 as the minimum supported version in `rust-version`, as required by the let-chains in `bake` 0.20.

### v0.3.0

- Write the whole readme in the form `markdown:normalize` writes it, by passing the updated document through `base_markdown::normalize_markdown`, the normalizer `bake-markdown` uses. Authored content outside the generated regions is normalized too: for example, soft-wrapped lines are joined and `*` list markers become `-`. The library functions that return updated documents normalize their results the same way. Requires `base-markdown` 0.1.3.
- Find headings and markers with the syntax the normalizer parses, including front matter and math, so a heading-like line inside a math block or front matter no longer places generated sections there.
- Insert the generated package entry into an authored See Also list without a blank line, so a tight list stays tight.

<!-- bake-readme:releases:end -->

## See Also

- [`bake`](https://github.com/base-rs/bake).

- [`bake-releases`](https://github.com/base-rs/bake-releases).

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/base-rs/bake-readme). The [readme guidance](context/readme-structure.md) describes the intended scope and structure of project `readme.md` files.

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.
