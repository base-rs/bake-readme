// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Property checks that `readme:update` output is exactly what `markdown:normalize` writes.

use bake_readme::{
    PackageMetadata, Release, ensure_releases_section, update_document,
    update_document_with_releases, update_releases_section,
};
use base_markdown::normalize_markdown;

/// Source documents from the readme tests, plus unnormalized authored content.
const DOCUMENTS: &[&str] = &[
    "",
    "# Project\n",
    "# Project",
    "# Project\n\nDescription.",
    "# Example\n\n",
    "# Example\r\nDescription",
    "# Project\n\nA short description.\n\n## Contributing\n\nContributions welcome.\n",
    "# Project\n\n## Releases\nCustom release\nlinks.\n",
    "# Project\r\n\r\n## Releases\rCustom release links.\r\n",
    "# Project\n\n```markdown\n## Releases\n```\n\n> ## Releases\n\n## Contributing\n",
    "# Project\r\n\r\n## Contributing\r\n",
    "# Project\n\n## Releases\n\nSee the project website for release notes.\n",
    "# Example\n## See Also\n",
    "# Example\n## Contributing\n",
    "## Contributing\n",
    "# Example\n\n## See Also\n",
    "## See Also\nExisting content",
    "# Example\n\n## See Also\n\n- [Other](https://example.com)\n- [old-name](https://github.com/example/project) — Old description. <!-- bake-readme:package -->\n",
    "# Example\n\n## See Also\n\n- [Project source](https://github.com/example/project)\n",
    "# Example\n\nSource: [repository](https://example.com/source).\n\n## Contributing\n",
    "# Example\n\nSource: [repository][source].\n\n[source]: https://example.com/source\n",
    "# Example\n\n## See Also\n\n- [example-crate](https://example.com/source) <!-- bake-readme:package -->\n\n## Contributing\n\nSource: [repository](https://example.com/source).\n",
    "## See Also\n\n- [example-crate](https://example.com/source) <!-- bake-readme:package -->\n\n## Contributing\nSource: [repository](https://example.com/source).\n",
    "# Example\n\n## See Also\n\n- [Related](https://example.com/related)\n- [example-crate](https://example.com/source) <!-- bake-readme:package -->\n\n## Contributing\n\nSource: [repository](https://example.com/source).\n",
    "## See Also\n- [old-name](https://example.com/old) <!-- bake-readme:package -->",
    "## See Also\r\n\r\n- [old-name](https://example.com/old) <!-- bake-readme:package -->\r\n",
    "# Example\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n\n## Contributing\n",
    "## Releases\r\n\r\nSee [releases.md](releases.md) for the release history.\r\n\r\n## Contributing\r\n",
    "## Releases\n<!-- bake-readme:releases:end -->\nOld text\n<!-- bake-readme:releases:start -->\n",
    "## Releases\n<!-- bake-readme:releases:start -->\nOld text\n",
    "<!-- bake-readme:releases:start -->\n\n## Releases\n<!-- bake-readme:releases:start -->\nold\n<!-- bake-readme:releases:end -->\n",
    "## Releases\r\n<!-- bake-readme:releases:start -->\r\nold\r\n<!-- bake-readme:releases:end -->\r\n",
    "# Project\n\n## Releases\n\n<!-- bake-readme:releases:start -->\n\n \t\n\nSee [releases.md](releases.md) for the full release history.\n\n\n  \n<!-- bake-readme:releases:end -->\n",
    "# Project\n\n[Source][source]\n\n[source]: https://example.com/other\n[source]: https://example.com/repository\n",
    "# P\n\n## See Also\n\n- [a](https://a)\n- [b](https://b)\n",
    "# P\n\n$$\n## Contributing\n$$\n\nText\n",
    "---\nReleases\n---\n\n# P\n",
    "# P\n\n- [x] a\n  - b\n  - c\n- [ ] d\n\n  ```\n  x\n  ```\n",
    "Wrapped\nheading\n---\n\n## See Also\n",
    "# P\n\n- a\n  - b\n\n  More.\n",
    "# Project\n\nA soft\nwrapped paragraph with *wrapped\nemphasis*.\n\n* Star\n* bullets\n\n## See Also\n\n* [Other](https://example.com) <!-- note -->\n",
];

fn packages() -> Vec<Option<PackageMetadata>> {
    let package = |description: Option<&str>, repository: Option<&str>| {
        Some(PackageMetadata {
            name: "example-crate".to_owned(),
            description: description.map(str::to_owned),
            repository: repository.map(str::to_owned),
        })
    };
    vec![
        None,
        package(
            Some("A useful *library*\nfor examples."),
            Some("https://example.com/source"),
        ),
        package(None, Some("https://github.com/example/project")),
        package(Some("  A   local crate.  "), None),
        package(Some(" \n\t "), Some("  ")),
    ]
}

fn release_sets() -> Vec<Vec<Release>> {
    let release = |name: &str, notes: &str| Release {
        name: name.to_owned(),
        notes: notes.to_owned(),
    };
    vec![
        Vec::new(),
        vec![release("v1.0.0", "- Initial release.")],
        vec![
            release("v1.1.0", "* Wrapped\nnote with `code`.\r\n* Second."),
            release("v1.0.0\r", ""),
        ],
    ]
}

fn normalize(document: &str) -> String {
    normalize_markdown(document).unwrap()
}

/// Check that an update writes normalized LF output, commutes with normalization, and is idempotent.
fn assert_normalized_update(update: impl Fn(&str) -> String, source: &str) {
    let updated = update(source);

    assert!(!updated.contains('\r'), "{source:?} -> {updated:?}");
    assert_eq!(
        normalize(&updated),
        updated,
        "normalize(update({source:?}))"
    );
    assert_eq!(
        update(&normalize(source)),
        updated,
        "update(normalize({source:?}))"
    );
    assert_eq!(update(&updated), updated, "update(update({source:?}))");
}

#[test]
fn update_document_output_is_normalized_and_idempotent() {
    for package in packages() {
        for source in DOCUMENTS {
            assert_normalized_update(
                |document| update_document(document, package.as_ref()),
                source,
            );
        }
    }
}

#[test]
fn update_document_with_releases_output_is_normalized_and_idempotent() {
    for package in packages() {
        for releases in release_sets() {
            for source in DOCUMENTS {
                assert_normalized_update(
                    |document| update_document_with_releases(document, package.as_ref(), &releases),
                    source,
                );
            }
        }
    }
}

#[test]
fn releases_section_output_is_normalized_and_idempotent() {
    for releases in release_sets() {
        for source in DOCUMENTS {
            assert_normalized_update(ensure_releases_section, source);
            assert_normalized_update(
                |document| update_releases_section(document, &releases),
                source,
            );
        }
    }
}

#[test]
fn normalizes_authored_content_outside_generated_regions() {
    let updated = update_document(
        "# Project\n\nA soft\nwrapped paragraph.\n\n* Star bullet\n",
        None,
    );

    assert_eq!(
        updated,
        "# Project\n\nA soft wrapped paragraph.\n\n- Star bullet\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n"
    );
}

#[test]
fn keeps_an_authored_see_also_list_tight() {
    let package = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: Some("https://example.com/source".to_owned()),
    };

    assert_eq!(
        update_document(
            "# P\n\n## See Also\n\n- [a](https://a)\n- [b](https://b)\n\n## Releases\n\nNotes.\n",
            Some(&package)
        ),
        "# P\n\n## See Also\n\n- [example-crate](https://example.com/source) <!-- bake-readme:package -->\n- [a](https://a)\n- [b](https://b)\n\n## Releases\n\nNotes.\n"
    );
}

#[test]
fn finds_headings_with_the_normalizer_syntax() {
    let updated = update_document(
        "---\nReleases\n---\n\n# P\n\n$$\n## Contributing\n$$\n",
        None,
    );

    assert_eq!(
        updated,
        "---\nReleases\n---\n\n# P\n\n$$\n## Contributing\n$$\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n"
    );
}

#[test]
fn keeps_task_list_structure() {
    let source = "# P\n\n- [x] a\n  - b\n  - c\n\n## Releases\n\nNotes.\n";

    assert_eq!(update_document(source, None), source);
}
