#!/usr/bin/env python3
"""Print a compact map of the features from a features-cli JSON report.

The JSON report of `features scan --json` contains each feature's full README,
specs and decision records, which can be large. This script prints a short
index first, then the details of the features you ask for.

Usage:
    features scan ./src --json --skip-changes > features.json
    python3 feature_map.py features.json                   # one line per feature
    python3 feature_map.py features.json --feature checkout  # details of one feature
    python3 feature_map.py features.json --owner team-payments
    python3 feature_map.py features.json --file src/features/checkout/api.ts
    features records ./src --json > records.json
    python3 feature_map.py features.json --records records.json  # project-level records

--feature matches a feature name or path (case-insensitive, substring).
--file prints the feature that contains a file (path relative to the scanned
folder, or ending with the feature path). --records prints the project-level
specs and decisions (root `docs/` records not attached to a feature).
Uses the Python standard library only.
"""

import argparse
import json
import sys


def flatten(features, depth=0):
    for feature in features:
        yield feature, depth
        yield from flatten(feature.get("features", []), depth + 1)


def first_line(text):
    for line in (text or "").splitlines():
        line = line.strip()
        if line and not line.startswith("#"):
            return line
    return ""


def feature_records(feature):
    """Specs and decisions of a feature; older reports only have raw decisions."""
    if "records" in feature:
        return feature["records"]
    return [
        {"kind": "decision", "id": str(index), "title": first_heading(text), "content": text}
        for index, text in enumerate(feature.get("decisions", []), 1)
    ]


def first_heading(text):
    for line in (text or "").splitlines():
        line = line.strip()
        if line.startswith("#"):
            return line.lstrip("#").strip()
    return "Untitled"


def record_line(record):
    kind = "Spec" if record.get("kind") == "spec" else "ADR"
    details = [record[key] for key in ("status", "date") if record.get(key)]
    tasks = record.get("tasks")
    if tasks:
        details.append(f"{tasks['done']}/{tasks['total']} tasks")
    line = f"- [{kind}] {record.get('id')}: {record.get('title')}"
    if details:
        line += f" ({', '.join(details)})"
    links = record.get("links") or {}
    linked = [
        f"{label} {', '.join(links[key])}"
        for key, label in (
            ("specs", "specs:"),
            ("decisions", "decisions:"),
            ("supersedes", "supersedes:"),
            ("superseded_by", "superseded by:"),
        )
        if links.get(key)
    ]
    if linked:
        line += f" [{'; '.join(linked)}]"
    if record.get("path"):
        line += f" <{record['path']}>"
    return line


def print_records(records, title):
    if not records:
        return
    print(f"\n## {title}\n")
    for record in records:
        print(record_line(record))
    for record in records:
        kind = "Spec" if record.get("kind") == "spec" else "Decision record"
        print(f"\n### {kind} {record.get('id')}: {record.get('title')}\n")
        print((record.get("content") or "").strip())
        for sub_file in record.get("files") or []:
            print(f"\n#### {sub_file.get('name')}\n")
            print((sub_file.get("content") or "").strip())


def summary(feature, depth):
    owner = feature.get("owner") or "no owner"
    if feature.get("owner") and feature.get("is_owner_inherited"):
        owner += " (inherited)"
    parts = [f"{'  ' * depth}- {feature['name']} -> {feature['path']} [{owner}]"]
    description = first_line(feature.get("description"))
    if description:
        parts.append(f": {description}")
    records = feature_records(feature)
    specs = sum(1 for r in records if r.get("kind") == "spec")
    decisions = len(records) - specs
    counts = [
        f"{count} {label}"
        for count, label in ((specs, "spec(s)"), (decisions, "decision(s)"))
        if count
    ]
    if counts:
        parts.append(f" ({', '.join(counts)})")
    return "".join(parts)


def details(feature):
    print(f"# {feature['name']}\n")
    print(f"- Path: {feature['path']}")
    owner = feature.get("owner") or "none"
    if feature.get("owner") and feature.get("is_owner_inherited"):
        owner += " (inherited from parent)"
    print(f"- Owner: {owner}")
    stats = feature.get("stats") or {}
    for key in ("files_count", "lines_count", "todos_count"):
        if key in stats:
            print(f"- {key.replace('_count', '').capitalize()}: {stats[key]}")
    for key, value in sorted((feature.get("meta") or {}).items()):
        if isinstance(value, (str, int, float, bool)):
            print(f"- {key}: {value}")
        else:
            print(f"- {key}: {json.dumps(value)[:200]}")
    nested = [f["name"] for f in feature.get("features", [])]
    if nested:
        print(f"- Nested features: {', '.join(nested)}")
    deps = {}
    for dep in feature.get("dependencies", []):
        key = (dep.get("type"), dep.get("featurePath"))
        deps[key] = deps.get(key, 0) + 1
    if deps:
        print("- Imports into other features:")
        for (kind, path), count in sorted(deps.items(), key=lambda item: str(item[0])):
            print(f"  - {kind} {path}: {count} import(s)")
    print("\n## Description\n")
    print((feature.get("description") or "(none)").strip())
    print_records(feature_records(feature), "Specs and decisions")
    print()


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument(
        "input", nargs="?", help="JSON file from `features scan --json` (default: stdin)"
    )
    parser.add_argument("--feature", help="print the details of matching features")
    parser.add_argument("--owner", help="list only the features of this owner")
    parser.add_argument("--file", help="print the feature that contains this file")
    parser.add_argument(
        "--records",
        metavar="RECORDS_JSON",
        help="print the project-level records of `features records --json`",
    )
    args = parser.parse_args()

    if args.records:
        with open(args.records, encoding="utf-8") as handle:
            records = json.load(handle).get("records", [])
        if not records:
            print("No project-level specs or decisions.")
            return
        print(f"{len(records)} project-level record(s)")
        print_records(records, "Project specs and decisions")
        return

    if args.input:
        with open(args.input, encoding="utf-8") as handle:
            data = json.load(handle)
    else:
        data = json.load(sys.stdin)

    entries = list(flatten(data))

    if args.file:
        target = args.file.replace("\\", "/")
        while target.startswith("./"):
            target = target[2:]
        matches = [
            (f, d)
            for f, d in entries
            if target.startswith(f["path"].rstrip("/") + "/")
            or ("/" + f["path"].rstrip("/") + "/") in ("/" + target)
        ]
        if not matches:
            print(f"{args.file} is not inside any feature.")
            return
        # The deepest matching feature is the closest one.
        feature = max(matches, key=lambda m: len(m[0]["path"]))[0]
        details(feature)
        return

    if args.feature:
        needle = args.feature.lower()
        matches = [
            f for f, _ in entries if needle in f["name"].lower() or needle in f["path"].lower()
        ]
        if not matches:
            print(f"No feature matches '{args.feature}'.")
            sys.exit(1)
        for feature in matches:
            details(feature)
        return

    if args.owner:
        entries = [(f, 0) for f, _ in entries if (f.get("owner") or "") == args.owner]

    print(f"{len(entries)} feature(s)\n")
    for feature, depth in entries:
        print(summary(feature, depth))


if __name__ == "__main__":
    main()
