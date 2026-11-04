//! Folder names left out when walking a project.

/// Folders every walker skips by default. Add a name here to skip another folder
/// everywhere; `--include-skipped-dirs` preserves them for feature detection,
/// stats and import scanning.
pub const SKIPPED_DIRS: &[&str] = &["node_modules"];

/// Folders the metadata comment and file map walkers also leave out, since they
/// don't hold source files to look at.
const NON_SOURCE_DIRS: &[&str] = &[
    "target",
    "dist",
    "build",
    ".git",
    ".svn",
    ".hg",
    "vendor",
    "__pycache__",
    ".next",
    ".nuxt",
    "coverage",
];

/// Check if the metadata comment and file map walkers should leave a folder out
pub fn is_non_source_dir(dir_name: &str) -> bool {
    SKIPPED_DIRS.contains(&dir_name) || NON_SOURCE_DIRS.contains(&dir_name)
}
