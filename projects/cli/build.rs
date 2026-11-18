//! Build script: find the Claude skills to embed in the binary
//! (`features skills install`).
//!
//! In the repository they live in `projects/skills/skills`. The crates.io
//! package can't reach outside the crate, so the release workflow copies them
//! into `projects/cli/skills` before publishing, and that copy is used when the
//! repository folder isn't there.

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest_dir.join("../skills/skills"),
        manifest_dir.join("skills"),
    ];

    let Some(skills_dir) = candidates.iter().find(|dir| dir.is_dir()) else {
        panic!(
            "Claude skills not found: expected `{}` (repository) or `{}` (packaged copy)",
            candidates[0].display(),
            candidates[1].display()
        );
    };
    let skills_dir = skills_dir
        .canonicalize()
        .expect("could not resolve the skills folder");

    println!("cargo:rerun-if-changed={}", skills_dir.display());
    println!(
        "cargo:rustc-env=FEATURES_SKILLS_DIR={}",
        skills_dir.display()
    );
}
