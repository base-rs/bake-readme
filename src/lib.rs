// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Reusable `readme.md` maintenance tasks for Bake.
//!
//! The task adds a short summary of recent entries from `releases.md` when a
//! project `readme.md` does not already contain an authored Releases section. When
//! the project root has a Cargo package manifest, it also adds or refreshes a
//! generated package entry in the `readme.md` See Also section when the source
//! repository link is not already present elsewhere in the readme. The whole
//! readme is written as `markdown:normalize` from `bake-markdown` writes it.
mod document;
mod metadata;

pub use document::{
    PackageMetadata, Release, ensure_releases_section, recent_releases, update_document,
    update_document_with_releases, update_releases_section,
};

mod readme_support {
    use bake::{Context, Error, Result};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    use super::{metadata::read_project_package, recent_releases, update_document_with_releases};

    trait TemporaryFile: Write {
        fn set_permissions(&self, permissions: fs::Permissions) -> std::io::Result<()>;
        fn sync_all(&self) -> std::io::Result<()>;
    }

    impl TemporaryFile for fs::File {
        fn set_permissions(&self, permissions: fs::Permissions) -> std::io::Result<()> {
            fs::File::set_permissions(self, permissions)
        }

        fn sync_all(&self) -> std::io::Result<()> {
            fs::File::sync_all(self)
        }
    }

    fn prepare_temporary_file(
        file: &mut impl TemporaryFile,
        path: &std::path::Path,
        document: &str,
    ) -> Result<()> {
        file.write_all(document.as_bytes())?;
        file.set_permissions(fs::metadata(path)?.permissions())?;
        file.sync_all()?;
        Ok(())
    }

    fn write_document_atomically(path: &std::path::Path, document: &str) -> Result<()> {
        let directory = path
            .parent()
            .expect("a canonical readme file path should have a parent directory");
        let mut temporary = tempfile::NamedTempFile::new_in(directory)?;
        prepare_temporary_file(temporary.as_file_mut(), path, document)?;
        temporary
            .persist(path)
            .map_err(|error| Error::from(error.error))?;
        Ok(())
    }

    /// Add recent release notes and Cargo package links to `readme.md` when needed.
    pub(super) fn update(context: &mut Context, path: PathBuf) -> Result<()> {
        let path = context.root().join(path).canonicalize()?;
        let document = fs::read_to_string(&path)
            .map_err(|error| Error::new(format!("{}: {error}", path.display())))?;
        let package = read_project_package(context)?;
        let release_path = context.root().join("releases.md");
        let releases = if release_path.is_file() {
            let release_document = fs::read_to_string(&release_path)
                .map_err(|error| Error::new(format!("{}: {error}", release_path.display())))?;
            recent_releases(&release_document)
        } else {
            Vec::new()
        };
        let updated = update_document_with_releases(&document, package.as_ref(), &releases);

        if updated == document {
            return Ok(());
        }

        write_document_atomically(&path, &updated)
    }

    #[cfg(test)]
    #[path = "readme_tests.rs"]
    mod tests;
}

/// Add recent release notes and Cargo package links to `readme.md` when needed.
#[bake::task]
pub fn update(
    context: &mut bake::Context,
    #[bake(
        default = "readme.md",
        help = "readme file relative to the project root."
    )]
    path: std::path::PathBuf,
) -> bake::Result<()> {
    readme_support::update(context, path)
}
