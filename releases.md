# Releases

## Unreleased

- Include the `Release: vVERSION` trailer, GitHub Release creation, and the Base Project releasing skill in the standard Releasing paragraph.
- Discover guidance with `agent:context:list` and `agent:context:show` in the standard Agent Context subsection.
- Separate the generated release markers from the summary with blank lines, matching `markdown:normalize`.
- Always write LF line endings: CRLF and lone CR line endings in the readme and in release entries read from `releases.md` are converted to LF. A readme checked out with CRLF line endings, for example through `core.autocrlf`, is rewritten on each run.

## v0.2.7

- Depend on `base-markdown` under the `base-rs` organization, replacing `core-markdown`.

## v0.2.6

- Migrate to the source-backed `core-markdown` document AST, preserving supported Markdown behavior and source bytes. Resolve reference links using the first matching definition.

## v0.2.5

- Use `core-markdown` in place of `socketry-markdown`.

## v0.2.4

- Document generated task links and related project references.

## v0.2.3

- Adopt `socketry-project` 0.3.7 for shared project tasks and Markdown normalization.
- Require the aggregate test and coverage result for pull request merges.
- Refresh dependency examples and repository-owned agent guidance.

## v0.2.2

- Keep the Bake dependency open-ended from 0.18 so task libraries share their project's active task registry.

## v0.2.1

- Require Bake 0.18.0 for the shared task registry.

## v0.2.0

- Remove the redundant `bake_readme::readme` module; the task adapter is now available at `bake_readme::update`.

## v0.1.6

- Declare compatibility with the Bake 0.x API so task libraries can share one task registry when upgrading to crate-derived task namespaces.

## v0.1.5

- Use the shared `socketry-project` Releasing skill for the standard release process and remove references to the duplicate Bake Cargo publishing context.

## v0.1.4

- Improve generated README spacing and document the standard release process.
- Add context to errors when Cargo metadata cannot be launched.

## v0.1.3

- Update generated README sections with Markdown AST nodes while preserving existing package links.

## v0.1.2

- Create or update GitHub Releases after successful crates.io publication.
- Resolve the local task crate during version updates.

## v0.1.1

- Switch the runtime dependency from `socketry-bake` to `bake` 0.17.0.

## v0.1.0

- Create the initial `bake-readme` task library and `readme:update` task.
- Add metadata-driven package links to the `readme.md` See Also section.
