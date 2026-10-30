// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use bake_readme::{
    PackageMetadata, Release, ensure_releases_section, recent_releases, update_document,
    update_document_with_releases,
};
use std::fs;

#[test]
fn inserts_releases_section_before_contributing() {
    let document =
        "# Project\n\nA short description.\n\n## Contributing\n\nContributions welcome.\n";
    assert_eq!(
        ensure_releases_section(document),
        "# Project\n\nA short description.\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n\n## Contributing\n\nContributions welcome.\n"
    );
}

#[test]
fn appends_when_no_preferred_section_exists() {
    assert_eq!(
        ensure_releases_section("# Project\n\nDescription."),
        "# Project\n\nDescription.\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n"
    );
}

#[test]
fn existing_section_is_preserved_exactly() {
    let document = "# Project\n\n## Releases\nCustom release links.\n";
    assert_eq!(ensure_releases_section(document), document);
}

#[test]
fn existing_section_is_written_with_unix_line_endings() {
    let document = "# Project\r\n\r\n## Releases\rCustom release links.\r\n";
    assert_eq!(
        ensure_releases_section(document),
        "# Project\n\n## Releases\nCustom release links.\n"
    );
}

#[test]
fn ignores_headings_in_fenced_code_and_block_quotes() {
    let document =
        "# Project\n\n```markdown\n## Releases\n```\n\n> ## Releases\n\n## Contributing\n";
    assert!(ensure_releases_section(document).contains("\n## Releases\n\nSee [releases.md]"));
}

#[test]
fn converts_crlf_line_endings_when_inserting() {
    let document = "# Project\r\n\r\n## Contributing\r\n";
    assert_eq!(
        ensure_releases_section(document),
        "# Project\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n\n## Contributing\n"
    );
}

#[test]
fn selects_the_three_most_recent_versioned_release_entries() {
    let document = "# Releases\n\n## Unreleased\n\nNot published yet.\n\n## v1.2.0\n\n- Third.\n\n## v1.1.0\n\n- Second.\n\n## v1.0.0\n\n- First.\n\n## v0.9.0\n\n- Older.\n";
    assert_eq!(
        recent_releases(document),
        vec![
            Release {
                name: "v1.2.0".to_owned(),
                notes: "- Third.".to_owned()
            },
            Release {
                name: "v1.1.0".to_owned(),
                notes: "- Second.".to_owned()
            },
            Release {
                name: "v1.0.0".to_owned(),
                notes: "- First.".to_owned()
            },
        ]
    );
}

#[test]
fn returns_no_recent_releases_without_a_top_level_releases_heading() {
    assert!(recent_releases("# Project\n\n## v1.0.0\n\n- Not under Releases.\n").is_empty());
    assert!(recent_releases("## Releases\n\n### v1.0.0\n\n- Wrong heading depth.\n").is_empty());
}

#[test]
fn includes_recent_releases_in_generated_section_before_contributing() {
    let releases = [Release {
        name: "v1.2.0".to_owned(),
        notes: "- Improve startup.".to_owned(),
    }];
    let updated = update_document_with_releases("# Project\n\n## Contributing\n", None, &releases);
    assert!(
        updated.contains("## Releases\n\n<!-- bake-readme:releases:start -->"),
        "{updated:?}"
    );
    assert!(updated.contains("### v1.2.0\n\n- Improve startup."));
    assert!(updated.contains("<!-- bake-readme:releases:end -->\n\n## Contributing"));
}

#[test]
fn refreshes_generated_release_entries_and_preserves_authored_section_text() {
    let initial = [Release {
        name: "v1.0.0".to_owned(),
        notes: "- Initial release.".to_owned(),
    }];
    let document = update_document_with_releases("# Project\n", None, &initial);
    let document = document.replace(
        "<!-- bake-readme:releases:start -->",
        "A short release summary.\n\n<!-- bake-readme:releases:start -->",
    );
    let latest = [Release {
        name: "v1.1.0".to_owned(),
        notes: "- Faster builds.".to_owned(),
    }];
    let updated = update_document_with_releases(&document, None, &latest);
    assert!(updated.contains("A short release summary.\n\n<!-- bake-readme:releases:start -->"));
    assert!(updated.contains("### v1.1.0\n\n- Faster builds."));
    assert!(!updated.contains("v1.0.0"));
}

#[test]
fn preserves_authored_releases_section() {
    let document = "# Project\n\n## Releases\n\nSee the project website for release notes.\n";
    let releases = [Release {
        name: "v1.2.0".to_owned(),
        notes: "- Recent change.".to_owned(),
    }];
    assert_eq!(
        update_document_with_releases(document, None, &releases),
        document
    );
}

#[test]
fn creates_see_also_from_cargo_package_metadata() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: Some("A useful library for examples.".to_owned()),
        repository: Some("https://github.com/example/example-crate".to_owned()),
    };
    let updated = update_document("# Example\n\n## Contributing\n", Some(&metadata));
    assert!(updated.contains(
		"- [example-crate](https://github.com/example/example-crate) — A useful library for examples. <!-- bake-readme:package -->"
	));
    assert!(updated.contains("See [releases.md](releases.md)"));
}

#[test]
fn refreshes_only_the_generated_entry_in_an_existing_see_also_section() {
    let original = "# Example\n\n## See Also\n\n- [Other](https://example.com)\n- [old-name](https://github.com/example/project) — Old description. <!-- bake-readme:package -->\n";
    let metadata = PackageMetadata {
        name: "new-name".to_owned(),
        description: Some("Updated description.".to_owned()),
        repository: Some("https://github.com/example/project".to_owned()),
    };
    let updated = update_document(original, Some(&metadata));
    assert!(updated.contains("- [Other](https://example.com)\n"));
    assert!(updated.contains(
		"- [new-name](https://github.com/example/project) — Updated description. <!-- bake-readme:package -->"
	));
    assert!(!updated.contains("Old description."));
}

#[test]
fn leaves_manual_see_also_content_untouched() {
    let original =
        "# Example\n\n## See Also\n\n- [Project source](https://github.com/example/project)\n";
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: Some("Description.".to_owned()),
        repository: Some("https://github.com/example/project".to_owned()),
    };
    let updated = update_document(original, Some(&metadata));
    assert_eq!(
        updated
            .matches("https://github.com/example/project")
            .count(),
        1
    );
}

#[test]
fn task_updates_a_custom_readme_under_project_root() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("project-notes.md");
    fs::write(&path, "# Project\n").unwrap();

    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());
    context
        .call("readme:update", &["--path", "project-notes.md"])
        .unwrap();
    let updated = fs::read_to_string(&path).unwrap();
    assert!(updated.contains("[releases.md](releases.md)"));

    context
        .call("readme:update", &["--path", "project-notes.md"])
        .unwrap();
    assert_eq!(fs::read_to_string(path).unwrap(), updated);
}

#[test]
fn task_reports_a_missing_readme_path() {
    let directory = tempfile::tempdir().unwrap();
    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());

    assert!(context.call("readme:update", &[]).is_err());
}

#[test]
fn task_reports_readme_read_errors() {
    let directory = tempfile::tempdir().unwrap();
    fs::create_dir(directory.path().join("readme.md")).unwrap();
    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());

    assert!(context.call("readme:update", &[]).is_err());
}

#[test]
fn task_reports_invalid_cargo_manifest_errors() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(directory.path().join("readme.md"), "# Example\n").unwrap();
    fs::write(
        directory.path().join("Cargo.toml"),
        "this is not valid TOML[",
    )
    .unwrap();
    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());

    let error = context.call("readme:update", &[]).unwrap_err();

    assert!(error.to_string().contains("cargo metadata failed"));
}

#[test]
fn task_reports_invalid_release_file_encoding() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(directory.path().join("readme.md"), "# Example\n").unwrap();
    fs::write(directory.path().join("releases.md"), [0xff]).unwrap();
    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());

    assert!(context.call("readme:update", &[]).is_err());
}

#[test]
fn task_reads_package_metadata_from_the_root_cargo_manifest() {
    let directory = tempfile::tempdir().unwrap();
    let manifest = directory.path().join("Cargo.toml");
    fs::write(
		manifest,
		"[package]\nname = \"metadata-project\"\nversion = \"0.1.0\"\nedition = \"2024\"\ndescription = \"A project with metadata.\"\nrepository = \"https://example.com/source\"\n",
	)
	.unwrap();
    fs::create_dir(directory.path().join("src")).unwrap();
    fs::write(directory.path().join("src/main.rs"), "fn main() {}\n").unwrap();
    let path = directory.path().join("readme.md");
    fs::write(&path, "# Example\n").unwrap();

    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());
    context.call("readme:update", &[]).unwrap();
    let updated = fs::read_to_string(path).unwrap();
    assert!(updated.contains(
		"- [metadata-project](https://example.com/source) — A project with metadata. <!-- bake-readme:package -->"
	));
}

#[test]
fn task_reads_recent_releases_from_the_project_root() {
    let directory = tempfile::tempdir().unwrap();
    let readme = directory.path().join("readme.md");
    fs::write(&readme, "# Example\n").unwrap();
    fs::write(
        directory.path().join("releases.md"),
        "# Releases\n\n## Unreleased\n\n- Not published.\n\n## v0.1.0\n\n- Initial release.\n",
    )
    .unwrap();

    let registry = bake::Registry::discover().unwrap();
    let mut context = registry.context(directory.path());
    context.call("readme:update", &[]).unwrap();
    let updated = fs::read_to_string(readme).unwrap();
    assert!(updated.contains("### v0.1.0\n\n- Initial release."));
    assert!(!updated.contains("### Unreleased"));
}

#[test]
fn adds_metadata_entry_without_repository_as_inline_code() {
    let metadata = PackageMetadata {
        name: " local-crate ".to_owned(),
        description: Some("  A   local crate.  ".to_owned()),
        repository: None,
    };

    let updated = update_document("# Example\n", Some(&metadata));

    assert!(updated.contains("- `local-crate` — A local crate. <!-- bake-readme:package -->"));
}

#[test]
fn adds_metadata_entry_with_blank_repository_and_description() {
    let metadata = PackageMetadata {
        name: "local-crate".to_owned(),
        description: Some(" \n\t ".to_owned()),
        repository: Some("  ".to_owned()),
    };

    let updated = update_document("# Example\n", Some(&metadata));

    assert!(updated.contains("- `local-crate` <!-- bake-readme:package -->"));
}

#[test]
fn inserts_metadata_entry_into_an_existing_see_also_section() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: Some("https://example.com/source".to_owned()),
    };
    let original = "# Example\n\n## See Also\n\n- [Related](https://example.com/related)\n";

    let updated = update_document(original, Some(&metadata));

    assert!(
        updated
            .contains("- [example-crate](https://example.com/source) <!-- bake-readme:package -->")
    );
    assert!(updated.contains("- [Related](https://example.com/related)"));
}

#[test]
fn appends_see_also_when_there_is_no_contributing_section() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: None,
    };

    let updated = update_document("# Example", Some(&metadata));

    assert!(updated.contains("## See Also\n\n- `example-crate` <!-- bake-readme:package -->"));
}

#[test]
fn avoids_adding_package_link_when_source_is_already_linked_outside_see_also() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: Some("https://example.com/source".to_owned()),
    };
    let original =
        "# Example\n\nSource: [repository](https://example.com/source).\n\n## Contributing\n";

    let updated = update_document(original, Some(&metadata));

    assert!(!updated.contains("## See Also"));
    assert!(!updated.contains("bake-readme:package"));
}

#[test]
fn avoids_adding_package_link_when_source_uses_a_reference_link() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: Some("https://example.com/source".to_owned()),
    };
    let original =
        "# Example\n\nSource: [repository][source].\n\n[source]: https://example.com/source\n";

    let updated = update_document(original, Some(&metadata));

    assert!(!updated.contains("## See Also"));
    assert!(!updated.contains("bake-readme:package"));
}

#[test]
fn removes_generated_see_also_section_when_its_source_link_is_repeated_elsewhere() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: Some("https://example.com/source".to_owned()),
    };
    let original = "# Example\n\n## See Also\n\n- [example-crate](https://example.com/source) <!-- bake-readme:package -->\n\n## Contributing\n\nSource: [repository](https://example.com/source).\n";

    let updated = update_document(original, Some(&metadata));

    assert!(!updated.contains("## See Also"));
    assert!(updated.contains("## Contributing"));
}

#[test]
fn removes_generated_see_also_section_at_the_document_start() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: Some("https://example.com/source".to_owned()),
    };
    let original = "## See Also\n\n- [example-crate](https://example.com/source) <!-- bake-readme:package -->\n\n## Contributing\nSource: [repository](https://example.com/source).\n";

    let updated = update_document(original, Some(&metadata));

    assert!(!updated.contains("## See Also"));
    assert!(updated.starts_with("## Releases\n"));
}

#[test]
fn removes_only_generated_see_also_entry_when_manual_entries_remain() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: Some("https://example.com/source".to_owned()),
    };
    let original = "# Example\n\n## See Also\n\n- [Related](https://example.com/related)\n- [example-crate](https://example.com/source) <!-- bake-readme:package -->\n\n## Contributing\n\nSource: [repository](https://example.com/source).\n";

    let updated = update_document(original, Some(&metadata));

    assert!(updated.contains("## See Also"));
    assert!(updated.contains("- [Related](https://example.com/related)"));
    assert!(!updated.contains("bake-readme:package"));
}

#[test]
fn inserts_entry_into_empty_see_also_section() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: None,
    };

    let updated = update_document("# Example\n\n## See Also\n", Some(&metadata));

    assert!(updated.contains("## See Also\n\n- `example-crate` <!-- bake-readme:package -->"));
}

#[test]
fn inserts_entry_before_content_without_a_trailing_newline() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: None,
    };

    let updated = update_document("## See Also\nExisting content", Some(&metadata));

    assert!(updated.contains(
        "## See Also\n\n- `example-crate` <!-- bake-readme:package -->\n\nExisting content"
    ));
}

#[test]
fn inserts_see_also_before_contributing_with_one_preceding_newline() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: None,
    };

    let updated = update_document("# Example\n## Contributing\n", Some(&metadata));

    assert!(updated.contains("## Releases\n\nSee [releases.md]"));
    assert!(updated.contains("## See Also\n\n- `example-crate` <!-- bake-readme:package -->"));
    assert!(updated.contains("## Contributing"));
    assert!(updated.find("## Releases").unwrap() < updated.find("## See Also").unwrap());
}

#[test]
fn inserts_see_also_before_a_document_start_contributing_heading() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: None,
    };

    let updated = update_document("## Contributing\n", Some(&metadata));

    assert!(updated.starts_with("## Releases\n\nSee [releases.md]"));
    assert!(updated.contains("## See Also\n\n- `example-crate` <!-- bake-readme:package -->"));
    assert!(updated.contains("## Contributing"));
}

#[test]
fn appends_see_also_with_existing_blank_lines_and_crlf() {
    let metadata = PackageMetadata {
        name: "example-crate".to_owned(),
        description: None,
        repository: None,
    };

    let lf_updated = update_document("# Example\n\n", Some(&metadata));
    assert!(lf_updated.contains("# Example\n\n## Releases\n"));

    let crlf_updated = update_document("# Example\r\nDescription", Some(&metadata));
    assert!(crlf_updated.contains("\n\n## Releases\n"));
    assert!(!crlf_updated.contains('\r'));
}

#[test]
fn refreshes_generated_entry_without_a_trailing_newline() {
    let metadata = PackageMetadata {
        name: "new-name".to_owned(),
        description: Some("Current description.".to_owned()),
        repository: Some("https://example.com/source".to_owned()),
    };
    let original =
        "## See Also\n- [old-name](https://example.com/old) <!-- bake-readme:package -->";

    let updated = update_document(original, Some(&metadata));

    assert!(updated.contains(
        "- [new-name](https://example.com/source) — Current description. <!-- bake-readme:package -->"
    ));
}

#[test]
fn refreshes_generated_entry_and_converts_crlf_line_endings() {
    let metadata = PackageMetadata {
        name: "new-name".to_owned(),
        description: None,
        repository: Some("https://example.com/source".to_owned()),
    };
    let original =
        "## See Also\r\n\r\n- [old-name](https://example.com/old) <!-- bake-readme:package -->\r\n";

    let updated = update_document(original, Some(&metadata));

    assert!(updated.contains("## See Also\n\n- [new-name](https://example.com/source)"));
    assert!(!updated.contains('\r'));
}

#[test]
fn upgrades_legacy_generated_releases_link() {
    let document = "# Example\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n\n## Contributing\n";
    let releases = [Release {
        name: "v1.0.0".to_owned(),
        notes: "- Initial release.".to_owned(),
    }];

    let updated = update_document_with_releases(document, None, &releases);

    assert!(updated.contains("<!-- bake-readme:releases:start -->"));
    assert!(updated.contains("### v1.0.0\n\n- Initial release."));
    assert!(updated.contains("<!-- bake-readme:releases:end -->"));
}

#[test]
fn leaves_releases_section_unchanged_when_markers_are_reversed_or_incomplete() {
    let releases = [Release {
        name: "v1.0.0".to_owned(),
        notes: "- Initial release.".to_owned(),
    }];

    for document in [
        "## Releases\n<!-- bake-readme:releases:end -->\nOld text\n<!-- bake-readme:releases:start -->\n",
        "## Releases\n<!-- bake-readme:releases:start -->\nOld text\n",
    ] {
        assert_eq!(
            update_document_with_releases(document, None, &releases),
            document
        );
    }
}

#[test]
fn ignores_release_markers_outside_the_generated_section() {
    let document = "<!-- bake-readme:releases:start -->\n\n## Releases\n<!-- bake-readme:releases:start -->\nold\n<!-- bake-readme:releases:end -->\n";
    let releases = [Release {
        name: "v1.0.0".to_owned(),
        notes: "- Initial release.".to_owned(),
    }];

    let updated = bake_readme::update_releases_section(document, &releases);

    assert!(updated.starts_with("<!-- bake-readme:releases:start -->\n\n## Releases\n"));
    assert!(updated.contains("### v1.0.0\n\n- Initial release."));
    assert!(!updated.contains("\nold\n"));
}

#[test]
fn separates_release_markers_with_blank_lines_and_is_idempotent() {
    let releases = [Release {
        name: "v1.0.0".to_owned(),
        notes: "- Initial release.".to_owned(),
    }];
    let document = update_document_with_releases("# Project\n", None, &releases);

    assert_eq!(
        document,
        "# Project\n\n## Releases\n\n<!-- bake-readme:releases:start -->\n\nSee [releases.md](releases.md) for the full release history.\n\n### v1.0.0\n\n- Initial release.\n\n<!-- bake-readme:releases:end -->\n"
    );
    assert_eq!(
        update_document_with_releases(&document, None, &releases),
        document
    );

    let padded = document
        .replace("start -->\n\n", "start -->\n\n \t\n\n")
        .replace(
            "\n\n<!-- bake-readme:releases:end",
            "\n\n\n  \n<!-- bake-readme:releases:end",
        );
    assert_eq!(
        update_document_with_releases(&padded, None, &releases),
        document
    );
}

#[test]
fn upgrades_legacy_crlf_release_link_with_unix_line_endings() {
    let document = "## Releases\r\n\r\nSee [releases.md](releases.md) for the release history.\r\n\r\n## Contributing\r\n";
    let releases = [Release {
        name: "v1.0.0".to_owned(),
        notes: "- Initial release.".to_owned(),
    }];

    let updated = update_document_with_releases(document, None, &releases);

    assert!(updated.contains("<!-- bake-readme:releases:start -->\n\nSee [releases.md]"));
    assert!(
        updated
            .contains("- Initial release.\n\n<!-- bake-readme:releases:end -->\n\n## Contributing")
    );
    assert!(!updated.contains('\r'));
}

#[test]
fn converts_crlf_release_entries_from_release_notes() {
    let releases = recent_releases(
        "# Releases\r\n\r\nLine one\r\nline two\r\n--------\r\n\r\n- One.\r- Two.\r\n",
    );

    assert_eq!(releases[0].name, "Line one\nline two");
    assert_eq!(releases[0].notes, "- One.\n- Two.");
    let custom = [Release {
        name: "v1.0.0\r".to_owned(),
        notes: String::new(),
    }];
    assert!(!update_document_with_releases("# Project\n", None, &custom).contains('\r'));
}

#[test]
fn renders_release_without_notes_and_converts_crlf() {
    let document = "## Releases\r\n<!-- bake-readme:releases:start -->\r\nold\r\n<!-- bake-readme:releases:end -->\r\n";
    let releases = [Release {
        name: "v1.0.0".to_owned(),
        notes: String::new(),
    }];

    let updated = update_document_with_releases(document, None, &releases);

    assert!(updated.contains("<!-- bake-readme:releases:start -->\n\nSee [releases.md]"));
    assert!(updated.contains("### v1.0.0\n\n<!-- bake-readme:releases:end -->"));
    assert!(!updated.contains('\r'));
}

#[test]
fn inserts_releases_section_before_see_also_and_handles_spacing() {
    let cases = [
        (
            "# Example\n## See Also\n",
            "# Example\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n\n## See Also\n",
        ),
        (
            "# Example\n\n## Contributing\n",
            "# Example\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n\n## Contributing\n",
        ),
    ];

    for (document, expected) in cases {
        assert_eq!(ensure_releases_section(document), expected);
    }
}

#[test]
fn appends_releases_section_for_empty_and_already_spaced_documents() {
    assert_eq!(
        ensure_releases_section(""),
        "## Releases\n\nSee [releases.md](releases.md) for the release history.\n"
    );
    assert_eq!(
        ensure_releases_section("# Example\n\n"),
        "# Example\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n"
    );
    assert_eq!(
        ensure_releases_section("# Example\n"),
        "# Example\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n"
    );
    assert_eq!(
        ensure_releases_section("# Example\r\nDescription"),
        "# Example\nDescription\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n"
    );
}

#[test]
fn inserts_releases_section_at_the_start_and_before_a_tightly_spaced_heading() {
    assert_eq!(
        ensure_releases_section("## Contributing\n"),
        "## Releases\n\nSee [releases.md](releases.md) for the release history.\n\n## Contributing\n"
    );
    assert_eq!(
        ensure_releases_section("# Example\n## Contributing\n"),
        "# Example\n\n## Releases\n\nSee [releases.md](releases.md) for the release history.\n\n## Contributing\n"
    );
}

#[test]
fn reference_links_use_the_first_matching_definition() {
    let package = PackageMetadata {
        name: "example".to_owned(),
        description: None,
        repository: Some("https://example.com/repository".to_owned()),
    };
    for (first, second, generates_entry) in [
        (
            "https://example.com/other",
            "https://example.com/repository",
            true,
        ),
        (
            "https://example.com/repository",
            "https://example.com/other",
            false,
        ),
    ] {
        let source =
            format!("# Project\n\n[Source][source]\n\n[source]: {first}\n[source]: {second}\n");
        let updated = update_document(&source, Some(&package));
        assert_eq!(
            updated.contains("<!-- bake-readme:package -->"),
            generates_entry
        );
    }
}
