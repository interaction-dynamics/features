//! `features skills install`: copies the bundled Claude skills into a folder.

use std::fs;

use features_cli::skills::{InstallStatus, VERSION_FILE, bundled_skills, install};

fn statuses(report: &features_cli::skills::InstallReport) -> Vec<InstallStatus> {
    report.skills.iter().map(|(_, status)| *status).collect()
}

#[test]
fn installs_every_skill_with_its_references_and_scripts() {
    let target = tempfile::tempdir().unwrap();
    let skills_dir = target.path().join(".claude/skills");

    let report = install(&skills_dir, false, false).unwrap();

    assert_eq!(report.skills.len(), bundled_skills().len());
    assert!(
        statuses(&report)
            .iter()
            .all(|s| *s == InstallStatus::Installed)
    );
    assert_eq!(report.previous_version, None);
    for skill in bundled_skills() {
        assert!(
            skills_dir.join(&skill.name).join("SKILL.md").is_file(),
            "{}",
            skill.name
        );
    }
    assert!(
        skills_dir
            .join("document-feature/references/readme-template.md")
            .is_file()
    );
    assert!(
        skills_dir
            .join("explore-features/scripts/feature_map.py")
            .is_file()
    );
    // Paths in a skill are relative to the skill folder, not nested twice
    assert!(!skills_dir.join("create-feature/create-feature").exists());
    assert_eq!(
        fs::read_to_string(skills_dir.join(VERSION_FILE))
            .unwrap()
            .trim(),
        env!("CARGO_PKG_VERSION")
    );

    let skill = fs::read_to_string(skills_dir.join("create-feature/SKILL.md")).unwrap();
    assert!(skill.starts_with("---\nname: create-feature\n"));
}

#[test]
fn leaves_existing_skills_untouched_unless_forced() {
    let target = tempfile::tempdir().unwrap();
    let skills_dir = target.path();
    install(skills_dir, false, false).unwrap();

    let skill_file = skills_dir.join("create-feature/SKILL.md");
    let extra_file = skills_dir.join("create-feature/notes.md");
    fs::write(&skill_file, "edited").unwrap();
    fs::write(&extra_file, "mine").unwrap();
    fs::write(skills_dir.join(VERSION_FILE), "0.0.1\n").unwrap();

    let report = install(skills_dir, false, false).unwrap();
    assert!(
        statuses(&report)
            .iter()
            .all(|s| *s == InstallStatus::Skipped)
    );
    assert_eq!(fs::read_to_string(&skill_file).unwrap(), "edited");
    assert_eq!(
        fs::read_to_string(skills_dir.join(VERSION_FILE)).unwrap(),
        "0.0.1\n"
    );

    let report = install(skills_dir, true, false).unwrap();
    assert!(
        statuses(&report)
            .iter()
            .all(|s| *s == InstallStatus::Overwritten)
    );
    assert_eq!(report.previous_version.as_deref(), Some("0.0.1"));
    assert_ne!(fs::read_to_string(&skill_file).unwrap(), "edited");
    // Files that aren't part of the bundled skill are kept
    assert_eq!(fs::read_to_string(&extra_file).unwrap(), "mine");
}

#[test]
fn dry_run_writes_nothing() {
    let target = tempfile::tempdir().unwrap();
    let skills_dir = target.path().join("skills");

    let report = install(&skills_dir, false, true).unwrap();

    assert!(
        statuses(&report)
            .iter()
            .all(|s| *s == InstallStatus::Installed)
    );
    assert!(!skills_dir.exists());
}

#[test]
fn refuses_a_file_as_target() {
    let target = tempfile::tempdir().unwrap();
    let file = target.path().join("skills");
    fs::write(&file, "").unwrap();

    assert!(install(&file, false, false).is_err());
}
