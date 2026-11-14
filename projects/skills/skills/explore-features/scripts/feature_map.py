#!/usr/bin/env python3
"""Print a compact map of the features from a features-cli JSON report.

The JSON report of `features scan --json` contains each feature's full README,
specs and decision records, which can be large. This script prints a short
index first, then the details of the features you ask for.

Usage:
    features scan ./src --json --project-records --skip-changes > features.json
    python3 feature_map.py features.json                   # one line per feature
    python3 feature_map.py features.json --feature checkout  # details of one feature
    python3 feature_map.py features.json --owner team-payments
    python3 feature_map.py features.json --file src/features/checkout/api.ts
    python3 feature_map.py features.json --records         # every spec and decision

The report is either an array of features (`features scan --json`) or
`{"features": [...], "records": [...]}` (`--project-records`, which adds the
project-level specs and decisions). Reports of older versions without
`records` fall back to the raw `decisions`.

--feature matches a feature name or path (case-insensitive, substring).
--file prints the feature that contains a file (path relative to the scanned
folder, or ending with the feature path). Uses the Python standard library only.
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


def records_of(feature):
    """The specs and decisions of a feature, or its raw decisions in old reports."""
    if "records" in feature:
        return feature["records"] or []
    return [
        {
            "kind": "decision",
            "id": f"decision-{index}",
            "title": first_heading(content) or f"Decision {index}",
            "content": content,
            "links": {},
        }
        for index, content in enumerate(feature.get("decisions", []), 1)
    ]


def first_heading(text):
    for line in (text or "").splitlines():
        line = line.strip()
        if line.startswith("#"):
            return line.lstrip("#").strip()
    return ""


SOURCES = {"speckit": "Spec Kit", "kiro": "Kiro"}


def record_line(record, feature_path=None):
    kind = "spec" if record.get("kind") == "spec" else "decision"
    facts = [record[key] for key in ("status", "date") if record.get(key)]
    tasks = record.get("tasks")
    if tasks:
        facts.append(f"tasks {tasks['done']}/{tasks['total']}")
    if record.get("source") in SOURCES:
        facts.append(SOURCES[record["source"]])
    line = f"- [{kind}] {record['id']}: {record.get('title', '')}"
    if facts:
        line += f" ({', '.join(facts)})"
    if record.get("path"):
        line += f" -> {record['path']}"
    if feature_path is not None:
        line += f" [feature: {feature_path or 'project'}]"
    links = [
        f"{key.replace('_', ' ')}: {', '.join(ids)}"
        for key, ids in (record.get("links") or {}).items()
        if ids
    ]
    if links:
        line += f"\n    links: {'; '.join(links)}"
    return line


def record_counts(records):
    specs = sum(1 for r in records if r.get("kind") == "spec")
    decisions = len(records) - specs
    parts = []
    if specs:
        parts.append(f"{specs} spec(s)")
    if decisions:
        parts.append(f"{decisions} decision(s)")
    return ", ".join(parts)


def summary(feature, depth):
    owner = feature.get("owner") or "no owner"
    if feature.get("owner") and feature.get("is_owner_inherited"):
        owner += " (inherited)"
    parts = [f"{'  ' * depth}- {feature['name']} -> {feature['path']} [{owner}]"]
    description = first_line(feature.get("description"))
    if description:
        parts.append(f": {description}")
    counts = record_counts(records_of(feature))
    if counts:
        parts.append(f" ({counts})")
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
    records = records_of(feature)
    if records:
        print("\n## Specs and decisions\n")
        for record in records:
            print(record_line(record))
        for record in records:
            kind = "Spec" if record.get("kind") == "spec" else "Decision"
            print(f"\n## {kind} {record['id']}: {record.get('title', '')}\n")
            print((record.get("content") or "").strip())
            for file in record.get("files") or []:
                print(f"\n(also: {file['path']})")
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
        "--records", action="store_true", help="list every spec and decision record"
    )
    args = parser.parse_args()

    if args.input:
        with open(args.input, encoding="utf-8") as handle:
            data = json.load(handle)
    else:
        data = json.load(sys.stdin)

    if isinstance(data, dict):
        features = data.get("features", [])
        project_records = data.get("records", [])
    else:
        features, project_records = data, []

    entries = list(flatten(features))

    if args.records:
        lines = [
            record_line(record, feature["path"])
            for feature, _ in entries
            for record in records_of(feature)
        ]
        lines += [record_line(record, "") for record in project_records]
        print(f"{len(lines)} spec(s) and decision(s)\n")
        for line in lines:
            print(line)
        return

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

    if project_records and not args.owner:
        print(f"\nProject-level specs and decisions ({record_counts(project_records)}):")
        for record in project_records:
            print(record_line(record))


if __name__ == "__main__":
    main()
