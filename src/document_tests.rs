// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use super::{marker_line, source_line_range};

#[test]
fn marker_search_respects_the_requested_source_range() {
    let document = "## Releases\n<!-- marker -->\n";
    let marker_offset = document.find("<!-- marker -->").unwrap();

    assert_eq!(
        marker_line(document, 0..marker_offset, "<!-- marker -->"),
        None
    );
    assert_eq!(
        marker_line(document, marker_offset..document.len(), "<!-- marker -->"),
        Some((marker_offset, marker_offset + "<!-- marker -->\n".len()))
    );
}

#[test]
fn finds_source_line_ranges_at_document_edges() {
    assert_eq!(source_line_range("last", 0), (0, 4));
    assert_eq!(source_line_range("first\nlast", 6), (6, 10));
    assert_eq!(source_line_range("first\nlast\n", 2), (0, 6));
}

#[test]
fn heading_bodies_preserve_blank_lines_for_all_source_endings() {
    for ending in ["\n", "\r\n", "\r"] {
        for heading in ["## Release\n", "Release\n-------\n"] {
            let source = format!("{heading}\nNotes.\n").replace('\n', ending);
            let headings = super::headings(&source);
            assert_eq!(headings[0].title, "Release");
            assert_eq!(
                &source[headings[0].body_start..],
                format!("{ending}Notes.{ending}")
            );
        }
    }
}
