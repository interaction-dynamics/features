#!/usr/bin/env python3
"""Report on how readable each feature is on its own.

Reads the JSON output of `features scan PATH --json --skip-changes` (from a file
or stdin) and lists, per feature: missing description, missing owner, imports
into sibling features, size and decision records. Optionally lists the source
files that are not inside any feature.

Usage:
    features scan ./src --json --skip-changes | python3 feature_report.py
    python3 feature_report.py report.json --root ./src --strict

Exit code: 1 when there are errors (duplicate names), or warnings with
--strict; 0 otherwise. Uses the Python standard library only.
"""

import argparse
import json
import os
import sys
from collections import defaultdict

IGNORED_DIRS = {
    "node_modules",
    "target",
    "dist",
    "build",
    "out",
    "coverage",
    "vendor",
    "__pycache__",
}


def flatten(features, parent=None):
    for feature in features:
        yield feature, parent
        yield from flatten(feature.get("features", []), feature)


def files_outside_features(root, feature_paths):
    """Count source files under `root` that are not inside any feature folder."""
    counts = defaultdict(int)
    prefixes = [p.rstrip("/") + "/" for p in feature_paths]
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [
            d for d in dirnames if not d.startswith(".") and d not in IGNORED_DIRS
        ]
        for filename in filenames:
            if filename.startswith("."):
                continue
            rel = os.path.relpath(os.path.join(dirpath, filename), root)
            rel = rel.replace(os.sep, "/")
            if any(rel.startswith(prefix) for prefix in prefixes):
                continue
            top = rel.split("/", 1)[0] if "/" in rel else "(root files)"
            counts[top] += 1
    return dict(sorted(counts.items()))


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument(
        "input", nargs="?", help="JSON file from `features scan --json` (default: stdin)"
    )
    parser.add_argument(
        "--root", help="scanned folder, to list the files outside any feature"
    )
    parser.add_argument("--max-files", type=int, default=40)
    parser.add_argument("--max-lines", type=int, default=4000)
    parser.add_argument(
        "--strict", action="store_true", help="exit with 1 on warnings too"
    )
    args = parser.parse_args()

    if args.input:
        with open(args.input, encoding="utf-8") as handle:
            data = json.load(handle)
    else:
        data = json.load(sys.stdin)

    entries = list(flatten(data))
    errors, warnings, notes = [], [], []

    by_name = defaultdict(list)
    for feature, _ in entries:
        by_name[feature["name"]].append(feature["path"])
    for name, paths in sorted(by_name.items()):
        if len(paths) > 1:
            errors.append(f"Duplicate feature name '{name}': {', '.join(paths)}")

    rows = []
    coupling = defaultdict(list)
    for feature, parent in entries:
        path = feature["path"]
        stats = feature.get("stats") or {}
        files = stats.get("files_count")
        lines = stats.get("lines_count")
        owner = feature.get("owner") or ""
        inherited = feature.get("is_owner_inherited", False)

        if not (feature.get("description") or "").strip():
            warnings.append(
                f"{path}: no description (add a README with a title and a sentence after it)"
            )
        if not owner:
            warnings.append(f"{path}: no owner (add `owner` to the README frontmatter)")
        if files is not None and lines is not None:
            if files > args.max_files or lines > args.max_lines:
                notes.append(
                    f"{path}: large feature ({files} files, {lines} lines); "
                    "consider nested features along product boundaries"
                )

        for dep in feature.get("dependencies", []):
            if dep.get("type") == "sibling":
                coupling[(path, dep.get("featurePath", "?"))].append(dep)

        rows.append(
            (
                path,
                feature["name"],
                f"{owner} (inherited)" if owner and inherited else owner or "-",
                "-" if files is None else str(files),
                "-" if lines is None else str(lines),
                str(len(feature.get("decisions", []))),
                str(stats.get("todos_count", "-")),
            )
        )

    for (source, target), deps in sorted(coupling.items()):
        examples = ", ".join(
            f"{d.get('sourceFilename')}:{d.get('line')} -> {d.get('targetFilename')}"
            for d in deps[:3]
        )
        more = f" (+{len(deps) - 3} more)" if len(deps) > 3 else ""
        circular = " [circular]" if (target, source) in coupling else ""
        warnings.append(
            f"{source}: imports {len(deps)} time(s) from sibling feature {target}{circular}: "
            f"{examples}{more}"
        )

    print(f"# Feature report\n\n{len(entries)} feature(s)\n")
    if rows:
        print("| Path | Name | Owner | Files | Lines | Decisions | TODOs |")
        print("| ---- | ---- | ----- | ----- | ----- | --------- | ----- |")
        for row in rows:
            print("| " + " | ".join(cell.replace("|", "\\|") for cell in row) + " |")
        print()

    for title, items in (("Errors", errors), ("Warnings", warnings), ("Notes", notes)):
        print(f"## {title} ({len(items)})\n")
        for item in items:
            print(f"- {item}")
        print()

    if args.root:
        outside = files_outside_features(args.root, [f["path"] for f, _ in entries])
        print(f"## Files outside any feature ({sum(outside.values())})\n")
        print("Shared code and application glue belong here; feature code doesn't.\n")
        for folder, count in outside.items():
            print(f"- {folder}: {count} file(s)")
        print()

    if errors or (args.strict and warnings):
        sys.exit(1)


if __name__ == "__main__":
    main()
