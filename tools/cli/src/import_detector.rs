//! Module for detecting imports/dependencies in source code files
//!
//! This module scans source files for import statements and resolves them
//! to their actual file paths to detect cross-feature dependencies.

use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub file_path: String,
    pub line_number: usize,
    pub line_content: String,
    pub imported_path: String,
}

/// Represents language-specific import patterns
#[derive(Debug, Clone)]
enum ImportPattern {
    /// Rust: use statements
    Rust,
    /// JavaScript/TypeScript: import/require/export from
    JavaScript,
    /// Python: import/from...import
    Python,
    /// Go: import statements
    Go,
    /// Java/C#/Kotlin: import/using statements
    JavaLike,
    /// C/C++: #include statements
    CStyle,
    /// Ruby: require/require_relative
    Ruby,
    /// PHP: use/require/include
    Php,
    /// Shell: source/.
    Shell,
    /// CSS/SCSS/Less: @import
    Css,
}

/// Get import pattern for a file extension
fn get_import_pattern(extension: &str) -> Option<ImportPattern> {
    match extension {
        "rs" => Some(ImportPattern::Rust),
        "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => Some(ImportPattern::JavaScript),
        "py" => Some(ImportPattern::Python),
        "go" => Some(ImportPattern::Go),
        "java" | "kt" | "scala" => Some(ImportPattern::JavaLike),
        "cs" => Some(ImportPattern::JavaLike),
        "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" => Some(ImportPattern::CStyle),
        "rb" => Some(ImportPattern::Ruby),
        "php" => Some(ImportPattern::Php),
        "sh" | "bash" => Some(ImportPattern::Shell),
        "css" | "scss" | "less" => Some(ImportPattern::Css),
        _ => None,
    }
}

/// Extract import path from a Rust use statement
fn extract_rust_import(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // Remove "use " prefix and trailing semicolon
    let import_part = trimmed.strip_prefix("use ")?.trim_end_matches(';').trim();

    // Handle 'use crate::' or 'use super::' or 'use self::'
    if import_part.starts_with("crate::")
        || import_part.starts_with("super::")
        || import_part.starts_with("self::")
    {
        // Extract the module path (remove use blocks like {Type1, Type2})
        let path = if let Some(brace_pos) = import_part.find('{') {
            import_part[..brace_pos].trim()
        } else if let Some(as_pos) = import_part.find(" as ") {
            import_part[..as_pos].trim()
        } else {
            import_part
        };

        return Some(path.to_string());
    }

    None
}

/// Extract import path from JavaScript/TypeScript import/require
fn extract_javascript_import(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // import ... from "path" or import ... from 'path'
    if trimmed.starts_with("import ") {
        if let Some(from_pos) = trimmed.find(" from ") {
            let after_from = &trimmed[from_pos + 6..].trim();
            return extract_quoted_string(after_from);
        }
        // import "path" or import 'path'
        if let Some(quote_pos) = trimmed.find(['"', '\'']) {
            return extract_quoted_string(&trimmed[quote_pos..]);
        }
    }

    // export ... from "path"
    if trimmed.starts_with("export ")
        && trimmed.contains(" from ")
        && let Some(from_pos) = trimmed.find(" from ")
    {
        let after_from = &trimmed[from_pos + 6..].trim();
        return extract_quoted_string(after_from);
    }

    // require("path") or require('path')
    if trimmed.contains("require(")
        && let Some(paren_pos) = trimmed.find("require(")
    {
        let after_paren = &trimmed[paren_pos + 8..];
        return extract_quoted_string(after_paren);
    }

    None
}

/// Extract import path from Python import statements
fn extract_python_import(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // from ... import ...
    if trimmed.starts_with("from ")
        && let Some(import_pos) = trimmed.find(" import ")
    {
        let module_path = trimmed[5..import_pos].trim();
        // Only consider relative imports (starting with .)
        if module_path.starts_with('.') {
            return Some(module_path.to_string());
        }
    }

    // import ... (relative imports only)
    if let Some(import_part) = trimmed.strip_prefix("import ") {
        let import_part = import_part.trim();
        let module_path = if let Some(as_pos) = import_part.find(" as ") {
            &import_part[..as_pos]
        } else {
            import_part
        };

        // Only consider relative imports
        if module_path.starts_with('.') {
            return Some(module_path.trim().to_string());
        }
    }

    None
}

/// Extract import path from Go import statements
fn extract_go_import(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let after_import = trimmed.strip_prefix("import ")?.trim();
    extract_quoted_string(after_import)
}

/// Extract import path from Java-like languages (Java, C#, Kotlin, Scala)
fn extract_javalike_import(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // Java/Kotlin: import package.name
    if let Some(import_part) = trimmed.strip_prefix("import ") {
        let import_part = import_part.trim().trim_end_matches(';');
        // Skip static imports
        if import_part.starts_with("static ") {
            return None;
        }
        return Some(import_part.to_string());
    }

    // C#: using namespace
    if !trimmed.contains('=')
        && let Some(import_part) = trimmed.strip_prefix("using ")
    {
        let import_part = import_part.trim().trim_end_matches(';');
        return Some(import_part.to_string());
    }

    None
}

/// Extract include path from C/C++ #include statements
fn extract_c_include(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let after_include = trimmed.strip_prefix("#include ")?.trim();

    // #include "file.h" (local include)
    if let Some(path) = extract_quoted_string(after_include) {
        return Some(path);
    }

    // #include <file.h> (system include - we'll skip these)
    // But if it starts with a relative path marker, keep it
    if after_include.starts_with('<')
        && after_include.contains('/')
        && let Some(end) = after_include.find('>')
    {
        return Some(after_include[1..end].to_string());
    }

    None
}

/// Extract require path from Ruby
fn extract_ruby_require(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // require_relative "path" or require_relative 'path'
    if let Some(after_require) = trimmed.strip_prefix("require_relative ") {
        return extract_quoted_string(after_require.trim());
    }

    // require "path" or require 'path' (only relative paths with ./)
    if let Some(after_require) = trimmed.strip_prefix("require ")
        && let Some(path) = extract_quoted_string(after_require.trim())
        && path.starts_with('.')
    {
        return Some(path);
    }

    None
}

/// Extract require/include path from PHP
fn extract_php_include(line: &str) -> Option<String> {
    let trimmed = line.trim();

    for keyword in ["require", "require_once", "include", "include_once"] {
        if let Some(after_keyword) = trimmed.strip_prefix(keyword) {
            let after_keyword = after_keyword.trim();
            if let Some(path) = extract_quoted_string(after_keyword) {
                return Some(path);
            }
        }
    }

    None
}

/// Extract source path from Shell scripts
fn extract_shell_source(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // source path or . path
    if let Some(path) = trimmed.strip_prefix("source ") {
        let path = path.trim();
        return extract_quoted_string(path).or_else(|| Some(path.to_string()));
    }

    if let Some(path) = trimmed.strip_prefix(". ")
        && !trimmed.starts_with("..")
    {
        let path = path.trim();
        return extract_quoted_string(path).or_else(|| Some(path.to_string()));
    }

    None
}

/// Extract @import path from CSS/SCSS/Less
fn extract_css_import(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let after_import = trimmed.strip_prefix("@import ")?.trim();
    extract_quoted_string(after_import)
}

/// Extract a quoted string (single or double quotes)
fn extract_quoted_string(s: &str) -> Option<String> {
    let trimmed = s.trim();
    let first_char = trimmed.chars().next()?;

    // Check for quote characters
    if ['"', '\'', '`'].contains(&first_char)
        && let Some(end) = trimmed[1..].find(first_char)
    {
        return Some(trimmed[1..end + 1].to_string());
    }

    None
}

/// Extract import statement based on language pattern
fn extract_import(line: &str, pattern: &ImportPattern) -> Option<String> {
    match pattern {
        ImportPattern::Rust => extract_rust_import(line),
        ImportPattern::JavaScript => extract_javascript_import(line),
        ImportPattern::Python => extract_python_import(line),
        ImportPattern::Go => extract_go_import(line),
        ImportPattern::JavaLike => extract_javalike_import(line),
        ImportPattern::CStyle => extract_c_include(line),
        ImportPattern::Ruby => extract_ruby_require(line),
        ImportPattern::Php => extract_php_include(line),
        ImportPattern::Shell => extract_shell_source(line),
        ImportPattern::Css => extract_css_import(line),
    }
}

/// Scan a single file for import statements
pub fn scan_file_for_imports(file_path: &Path) -> Result<Vec<ImportStatement>> {
    let mut imports = Vec::new();

    let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let pattern = match get_import_pattern(extension) {
        Some(p) => p,
        None => return Ok(imports), // Unsupported file type
    };

    let content = fs::read_to_string(file_path)?;

    for (line_number, line) in content.lines().enumerate() {
        if let Some(imported_path) = extract_import(line, &pattern) {
            imports.push(ImportStatement {
                file_path: file_path.to_string_lossy().to_string(),
                line_number: line_number + 1, // 1-based
                line_content: line.trim().to_string(),
                imported_path,
            });
        }
    }

    Ok(imports)
}

/// Build a map of all files in the project for quick lookup
pub fn build_file_map(base_path: &Path) -> HashMap<String, PathBuf> {
    let mut file_map = HashMap::new();

    let skip_dirs = [
        "node_modules",
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

    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let dir_name = e.file_name().to_string_lossy();
                !skip_dirs.contains(&dir_name.as_ref())
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Ok(relative_path) = path.strip_prefix(base_path) {
                let key = relative_path.to_string_lossy().to_string();
                file_map.insert(key, path.to_path_buf());
            }
        }
    }

    file_map
}

/// Resolve an import path to an actual file path
pub fn resolve_import_path(
    import_path: &str,
    source_file: &Path,
    base_path: &Path,
    file_map: &HashMap<String, PathBuf>,
) -> Option<PathBuf> {
    let source_dir = source_file.parent()?;

    // Handle different types of imports
    if import_path.starts_with('.') {
        // Relative import (./file or ../file)
        resolve_relative_import(import_path, source_dir, base_path)
    } else if import_path.contains("::") {
        // Rust-style module path (crate::module::submodule)
        resolve_rust_module_path(import_path, source_file, base_path, file_map)
    } else if import_path.contains('/') {
        // Path-like import
        resolve_path_import(import_path, base_path, file_map)
    } else {
        // Package/module name - we'll skip these as they're external
        None
    }
}

/// Resolve relative imports like ./file or ../file
fn resolve_relative_import(
    import_path: &str,
    source_dir: &Path,
    base_path: &Path,
) -> Option<PathBuf> {
    let _import_path_clean = import_path
        .trim_start_matches("./")
        .trim_start_matches("../");

    // Try to resolve from source directory
    let candidate = source_dir.join(import_path);

    // First, check if the path is a directory and try index files
    // This handles: import foo from './folder' -> './folder/index.ts'
    if candidate.is_dir() {
        for index_name in ["index", "mod", "__init__"] {
            for idx_ext in ["ts", "tsx", "js", "jsx", "rs", "py"] {
                let index_path = candidate.join(format!("{}.{}", index_name, idx_ext));
                if index_path.exists() && index_path.starts_with(base_path) {
                    // Canonicalize to resolve .. and . in the path
                    if let Ok(canonical) = index_path.canonicalize() {
                        return Some(canonical);
                    }
                    return Some(index_path);
                }
            }
        }
    }

    // Try common extensions if no extension provided
    let extensions = [
        "", ".ts", ".tsx", ".js", ".jsx", ".rs", ".py", ".go", ".java", ".rb", ".php",
    ];

    for ext in extensions {
        let path_with_ext = if ext.is_empty() {
            candidate.clone()
        } else {
            // Append extension to the path
            let mut path_str = candidate.to_string_lossy().to_string();
            path_str.push_str(ext);
            PathBuf::from(path_str)
        };

        if path_with_ext.exists() && path_with_ext.starts_with(base_path) {
            // Canonicalize to resolve .. and . in the path
            if let Ok(canonical) = path_with_ext.canonicalize() {
                return Some(canonical);
            }
            return Some(path_with_ext);
        }

        // Also check if path with extension is a directory with index file
        // This handles edge cases like './folder.component' -> './folder.component/index.ts'
        if path_with_ext.is_dir() {
            for index_name in ["index", "mod", "__init__"] {
                for idx_ext in ["ts", "tsx", "js", "jsx", "rs", "py"] {
                    let index_path = path_with_ext.join(format!("{}.{}", index_name, idx_ext));
                    if index_path.exists() && index_path.starts_with(base_path) {
                        return Some(index_path);
                    }
                }
            }
        }
    }

    None
}

/// Resolve Rust module paths like crate::module::submodule
fn resolve_rust_module_path(
    import_path: &str,
    source_file: &Path,
    base_path: &Path,
    _file_map: &HashMap<String, PathBuf>,
) -> Option<PathBuf> {
    // Convert crate::module::submodule to module/submodule.rs or module/submodule/mod.rs
    let path_str = if let Some(stripped) = import_path.strip_prefix("crate::") {
        stripped
    } else if let Some(stripped) = import_path.strip_prefix("super::") {
        // Handle super:: by going up one directory
        return resolve_super_path(stripped, source_file, base_path);
    } else {
        import_path.strip_prefix("self::")?
    };

    // Convert :: to /
    let path_parts: Vec<&str> = path_str.split("::").collect();

    // Find src directory
    let src_dir = find_src_directory(base_path)?;

    // Try module/file.rs
    let mut module_path = src_dir.clone();
    for part in &path_parts {
        module_path = module_path.join(part);
    }

    if module_path.with_extension("rs").exists() {
        return Some(module_path.with_extension("rs"));
    }

    // Try module/mod.rs
    let mod_path = module_path.join("mod.rs");
    if mod_path.exists() {
        return Some(mod_path);
    }

    None
}

/// Resolve super:: paths in Rust
fn resolve_super_path(
    remaining_path: &str,
    source_file: &Path,
    base_path: &Path,
) -> Option<PathBuf> {
    let source_dir = source_file.parent()?;
    let parent_dir = source_dir.parent()?;

    if !parent_dir.starts_with(base_path) {
        return None;
    }

    let path_parts: Vec<&str> = remaining_path.split("::").collect();
    let mut module_path = parent_dir.to_path_buf();

    for part in &path_parts {
        module_path = module_path.join(part);
    }

    if module_path.with_extension("rs").exists() {
        return Some(module_path.with_extension("rs"));
    }

    let mod_path = module_path.join("mod.rs");
    if mod_path.exists() {
        return Some(mod_path);
    }

    None
}

/// Find the src directory in a Rust project
fn find_src_directory(base_path: &Path) -> Option<PathBuf> {
    let src_dir = base_path.join("src");
    if src_dir.is_dir() {
        return Some(src_dir);
    }

    // Look for src in subdirectories
    for entry in (fs::read_dir(base_path).ok()?).flatten() {
        let path = entry.path();
        if path.is_dir() {
            let nested_src = path.join("src");
            if nested_src.is_dir() {
                return Some(nested_src);
            }
        }
    }

    None
}

/// Resolve path-based imports
fn resolve_path_import(
    import_path: &str,
    _base_path: &Path,
    file_map: &HashMap<String, PathBuf>,
) -> Option<PathBuf> {
    // Try direct lookup
    if let Some(path) = file_map.get(import_path) {
        return Some(path.clone());
    }

    // Try with common extensions
    for ext in [
        "ts", "tsx", "js", "jsx", "rs", "py", "go", "java", "rb", "php",
    ] {
        let with_ext = format!("{}.{}", import_path, ext);
        if let Some(path) = file_map.get(&with_ext) {
            return Some(path.clone());
        }
    }

    // Try as directory with index
    for index_name in ["index", "mod", "__init__"] {
        for ext in ["ts", "tsx", "js", "jsx", "rs", "py"] {
            let index_path = format!("{}/{}.{}", import_path, index_name, ext);
            if let Some(path) = file_map.get(&index_path) {
                return Some(path.clone());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rust_import() {
        assert_eq!(
            extract_rust_import("use crate::models::Feature;"),
            Some("crate::models::Feature".to_string())
        );
        assert_eq!(
            extract_rust_import("use super::helper;"),
            Some("super::helper".to_string())
        );
        assert_eq!(
            extract_rust_import("use self::utils;"),
            Some("self::utils".to_string())
        );
    }

    #[test]
    fn test_extract_javascript_import() {
        assert_eq!(
            extract_javascript_import("import { Feature } from './models';"),
            Some("./models".to_string())
        );
        assert_eq!(
            extract_javascript_import("const x = require('../utils');"),
            Some("../utils".to_string())
        );
        assert_eq!(
            extract_javascript_import("export { Feature } from './models';"),
            Some("./models".to_string())
        );
    }

    #[test]
    fn test_extract_python_import() {
        assert_eq!(
            extract_python_import("from .models import Feature"),
            Some(".models".to_string())
        );
        assert_eq!(
            extract_python_import("from ..utils import helper"),
            Some("..utils".to_string())
        );
    }

    #[test]
    fn test_extract_quoted_string() {
        assert_eq!(
            extract_quoted_string("\"./path/to/file\""),
            Some("./path/to/file".to_string())
        );
        assert_eq!(
            extract_quoted_string("'./path/to/file'"),
            Some("./path/to/file".to_string())
        );
    }
}
