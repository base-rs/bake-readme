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

If `readme.md` has no authored `Releases` section, the task adds one with the three latest versioned entries from `releases.md`. It skips `Unreleased` and links to the full release history. The task also adds or refreshes a package entry in `See Also` based on the root `Cargo.toml` name, description, and repository URL. Existing authored sections and entries are preserved.

A custom path is supported with `--path PATH`.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`, or `bump --version X.Y.Z`), commit with a `Release: vVERSION` trailer, then run `cargo bake cargo:release` and open a pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it. Follow the shared [Releasing skill](https://github.com/base-rs/base-project/blob/main/context/releasing.md) for the standard process.

## Releases

<!-- bake-readme:releases:start -->
See [releases.md](releases.md) for the full release history.

### v0.2.7

- Depend on `base-markdown` under the `base-rs` organization, replacing `core-markdown`.

### v0.2.6

- Migrate to the source-backed `core-markdown` document AST, preserving supported Markdown behavior and source bytes. Resolve reference links using the first matching definition.

### v0.2.5

- Use `core-markdown` in place of `socketry-markdown`.
<!-- bake-readme:releases:end -->

## See Also

- [`bake`](https://github.com/base-rs/bake).

- [`bake-releases`](https://github.com/base-rs/bake-releases).

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/base-rs/bake-readme). The [readme guidance](context/readme-structure.md) describes the intended scope and structure of project `readme.md` files.

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Read `.agents/context/index.md` to find relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.
