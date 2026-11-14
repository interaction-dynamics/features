//! Watches a project for the file changes that can change its features.
//!
//! Events are filtered as they arrive: changes inside version control and build
//! output folders, default-skipped folders (`node_modules`) and `--ignore-path`
//! folders are dropped. The remaining ones only mark the features as stale
//! through a channel holding at most one signal, so a burst of events (or the
//! events arriving while a rescan runs) results in a single rescan.

use anyhow::Result;
use notify::event::ModifyKind;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::{Instant, timeout};

use crate::skip_dirs::{SKIPPED_DIRS, is_watch_ignored_dir};

/// How long no relevant event must arrive before rescanning
pub const QUIET_PERIOD: Duration = Duration::from_millis(400);

/// Longest wait for a quiet period, so a constant stream of events still rescans
pub const MAX_WAIT: Duration = Duration::from_secs(3);

/// Files that define a feature, whose content changes matter even when the
/// folder structure doesn't change
const FEATURE_DEFINITION_FILES: &[&str] = &["README.md", "README.mdx", "FEATURES.toml"];

/// Decides which file system events can change the features
#[derive(Debug, Clone)]
pub struct WatchFilter {
    /// The watched folder, as given and canonicalized, to make event paths relative
    roots: Vec<PathBuf>,
    /// Canonical `--ignore-path` folders
    ignore_paths: Vec<PathBuf>,
    /// Watch the folders in `SKIPPED_DIRS` instead of ignoring them
    include_skipped_dirs: bool,
}

impl WatchFilter {
    pub fn new(watch_path: &Path, ignore_paths: Vec<PathBuf>, include_skipped_dirs: bool) -> Self {
        let mut roots = vec![watch_path.to_path_buf()];
        if let Ok(absolute) = std::path::absolute(watch_path) {
            roots.push(absolute);
        }
        if let Ok(canonical) = fs::canonicalize(watch_path) {
            roots.push(canonical);
        }

        Self {
            roots,
            ignore_paths,
            include_skipped_dirs,
        }
    }

    /// Whether an event can change the features
    pub fn is_relevant(&self, event: &Event) -> bool {
        event
            .paths
            .iter()
            .any(|path| self.is_relevant_change(path, &event.kind))
    }

    /// Whether a change of this kind to `path` can change the features: files and
    /// folders added, removed or renamed, and edits to the files defining a feature
    pub fn is_relevant_change(&self, path: &Path, kind: &EventKind) -> bool {
        if self.is_ignored(path) {
            return false;
        }

        let changes_structure = kind.is_create()
            || kind.is_remove()
            || matches!(kind, EventKind::Modify(ModifyKind::Name(_)));

        changes_structure
            || path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| FEATURE_DEFINITION_FILES.contains(&name))
    }

    /// Whether `path` is inside a folder the watcher leaves out
    fn is_ignored(&self, path: &Path) -> bool {
        if self.is_in_ignore_path(path) {
            return true;
        }

        // Only look at folders inside the project, not at the folders it sits in
        let Some(relative) = self
            .roots
            .iter()
            .find_map(|root| path.strip_prefix(root).ok())
        else {
            return false;
        };

        relative.components().any(|component| {
            let Component::Normal(name) = component else {
                return false;
            };
            let Some(name) = name.to_str() else {
                return false;
            };

            is_watch_ignored_dir(name)
                || (!self.include_skipped_dirs && SKIPPED_DIRS.contains(&name))
        })
    }

    fn is_in_ignore_path(&self, path: &Path) -> bool {
        if self.ignore_paths.is_empty() {
            return false;
        }

        // A removed file can't be canonicalized anymore, but its folder can
        let canonical = fs::canonicalize(path).ok().or_else(|| {
            let parent = fs::canonicalize(path.parent()?).ok()?;
            Some(parent.join(path.file_name()?))
        });

        self.ignore_paths.iter().any(|ignored| {
            path.starts_with(ignored)
                || canonical
                    .as_ref()
                    .is_some_and(|canonical| canonical.starts_with(ignored))
        })
    }
}

/// Mark the features as stale. The channel holds at most one signal: if one is
/// already pending, a rescan is coming anyway and this one is dropped.
fn signal_change(tx: &Sender<()>) {
    let _ = tx.try_send(());
}

/// Start watching `watch_path`. The returned receiver gets a signal when a
/// relevant change happened; the watcher stops when it is dropped.
pub fn watch(watch_path: &Path, filter: WatchFilter) -> Result<(RecommendedWatcher, Receiver<()>)> {
    let (tx, rx) = mpsc::channel(1);

    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<Event>| match res {
            Ok(event) => {
                if filter.is_relevant(&event) {
                    signal_change(&tx);
                }
            }
            Err(e) => eprintln!("File watcher error: {:?}", e),
        },
        Config::default(),
    )?;
    watcher.watch(watch_path, RecursiveMode::Recursive)?;

    Ok((watcher, rx))
}

/// Wait for the next change, then until changes stop arriving for `quiet` (at
/// most `max_wait`), so a burst of changes leads to one rescan.
///
/// Returns false when the watcher stopped.
pub async fn next_change(changes: &mut Receiver<()>, quiet: Duration, max_wait: Duration) -> bool {
    if changes.recv().await.is_none() {
        return false;
    }

    let deadline = Instant::now() + max_wait;
    loop {
        let wait = quiet.min(deadline.saturating_duration_since(Instant::now()));
        if wait.is_zero() {
            return true;
        }
        match timeout(wait, changes.recv()).await {
            // Another change in the burst: keep waiting for quiet
            Ok(Some(())) => continue,
            // The watcher stopped: still rescan for the changes seen so far
            Ok(None) => return true,
            // Quiet period reached
            Err(_) => return true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{CreateKind, DataChange, ModifyKind, RemoveKind, RenameMode};

    const MODIFY: EventKind = EventKind::Modify(ModifyKind::Data(DataChange::Content));
    const CREATE: EventKind = EventKind::Create(CreateKind::File);

    fn filter(root: &Path) -> WatchFilter {
        WatchFilter::new(root, Vec::new(), false)
    }

    #[test]
    fn test_feature_definition_edits_are_relevant() {
        let root = tempfile::TempDir::new().unwrap();
        let filter = filter(root.path());

        for name in ["README.md", "README.mdx", "FEATURES.toml"] {
            let path = root.path().join("src/features/a").join(name);
            assert!(filter.is_relevant_change(&path, &MODIFY), "{name}");
        }
        let source = root.path().join("src/features/a/index.ts");
        assert!(!filter.is_relevant_change(&source, &MODIFY));
    }

    #[test]
    fn test_structure_changes_are_relevant() {
        let root = tempfile::TempDir::new().unwrap();
        let filter = filter(root.path());
        let path = root.path().join("src/features/new-feature");

        assert!(filter.is_relevant_change(&path, &CREATE));
        assert!(filter.is_relevant_change(&path, &EventKind::Remove(RemoveKind::Folder)));
        assert!(filter.is_relevant_change(
            &path,
            &EventKind::Modify(ModifyKind::Name(RenameMode::Both))
        ));
        // Coverage reports feed the dashboard, so they are watched too
        assert!(filter.is_relevant_change(&root.path().join("coverage/lcov.info"), &CREATE));
    }

    #[test]
    fn test_ignores_vcs_build_and_skipped_dirs() {
        let root = tempfile::TempDir::new().unwrap();
        let filter = filter(root.path());

        for dir in [
            ".git",
            "target",
            "dist",
            "node_modules",
            "src/node_modules/x",
        ] {
            let path = root.path().join(dir).join("README.md");
            assert!(!filter.is_relevant_change(&path, &CREATE), "{dir}");
        }

        // `--include-skipped-dirs` watches node_modules again, but not build output
        let filter = WatchFilter::new(root.path(), Vec::new(), true);
        assert!(filter.is_relevant_change(&root.path().join("node_modules/a/README.md"), &CREATE));
        assert!(!filter.is_relevant_change(&root.path().join("target/a/README.md"), &CREATE));
    }

    #[test]
    fn test_ignores_ignore_paths() {
        let root = tempfile::TempDir::new().unwrap();
        let ignored = root.path().join("legacy");
        fs::create_dir_all(&ignored).unwrap();
        let filter = WatchFilter::new(
            root.path(),
            vec![fs::canonicalize(&ignored).unwrap()],
            false,
        );

        // Existing and removed files inside the ignored folder, reported with
        // the watched path as given or canonicalized
        fs::write(ignored.join("kept.ts"), "").unwrap();
        for base in [ignored.clone(), fs::canonicalize(&ignored).unwrap()] {
            assert!(!filter.is_relevant_change(&base.join("kept.ts"), &CREATE));
            assert!(!filter.is_relevant_change(&base.join("gone.ts"), &CREATE));
        }
        assert!(filter.is_relevant_change(&root.path().join("src/a.ts"), &CREATE));
    }

    #[test]
    fn test_folders_above_the_project_are_not_checked() {
        // A project inside a folder named like an ignored one is still watched
        let parent = tempfile::TempDir::new().unwrap();
        let root = parent.path().join("build").join("project");
        fs::create_dir_all(&root).unwrap();
        let filter = filter(&root);

        assert!(filter.is_relevant_change(&root.join("src/features/a"), &CREATE));
        let canonical = fs::canonicalize(&root).unwrap();
        assert!(filter.is_relevant_change(&canonical.join("src/features/a"), &CREATE));
    }

    #[test]
    fn test_signals_while_one_is_pending_are_dropped() {
        let (tx, mut rx) = mpsc::channel(1);
        for _ in 0..10 {
            signal_change(&tx);
        }

        assert!(rx.try_recv().is_ok());
        assert!(rx.try_recv().is_err(), "only one rescan is queued");
    }

    #[tokio::test]
    async fn test_next_change_waits_for_the_burst_to_end() {
        let (tx, mut rx) = mpsc::channel(1);
        let quiet = Duration::from_millis(100);

        let sender = tokio::spawn(async move {
            for _ in 0..5 {
                signal_change(&tx);
                tokio::time::sleep(Duration::from_millis(30)).await;
            }
            tx
        });

        let start = Instant::now();
        assert!(next_change(&mut rx, quiet, Duration::from_secs(5)).await);
        // The burst lasts ~150ms, then a quiet period follows
        assert!(start.elapsed() >= Duration::from_millis(200));

        let _tx = sender.await.unwrap();
        assert!(
            rx.try_recv().is_err(),
            "the burst is consumed by one change"
        );
    }

    #[tokio::test]
    async fn test_next_change_stops_waiting_after_max_wait() {
        let (tx, mut rx) = mpsc::channel(1);

        let sender = tokio::spawn(async move {
            for _ in 0..40 {
                signal_change(&tx);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });

        let start = Instant::now();
        assert!(
            next_change(
                &mut rx,
                Duration::from_millis(100),
                Duration::from_millis(150)
            )
            .await
        );
        assert!(start.elapsed() < Duration::from_millis(300));
        sender.abort();
    }

    #[tokio::test]
    async fn test_next_change_returns_false_when_the_watcher_stops() {
        let (tx, mut rx) = mpsc::channel::<()>(1);
        drop(tx);
        assert!(!next_change(&mut rx, QUIET_PERIOD, MAX_WAIT).await);
    }
}
