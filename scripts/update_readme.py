#!/usr/bin/env python3
"""Update all commit SHAs and line numbers in README.md based on rosu_pp_py.pyi."""

import ast
import json
import re
import urllib.request
from pathlib import Path

REPO = "MaxOhn/rosu-pp-py"
BRANCH = "main"
README = Path("README.md")
PYI = Path("rosu_pp_py.pyi")


def get_latest_sha() -> str:
    url = f"https://api.github.com/repos/{REPO}/commits/{BRANCH}"
    req = urllib.request.Request(url, headers={"User-Agent": "readme-updater"})
    with urllib.request.urlopen(req) as resp:
        return json.load(resp)["sha"]


def get_line_ranges(pyi_path: Path) -> dict[str, tuple[int, int]]:
    """Parse the .pyi file and return {ClassName: (start_line, end_line)}."""
    source = pyi_path.read_text()
    tree = ast.parse(source)
    lines = source.splitlines()

    top_level = [
        n for n in ast.walk(tree)
        if isinstance(n, ast.ClassDef) and n.col_offset == 0
    ]

    ranges: dict[str, tuple[int, int]] = {}
    for i, node in enumerate(top_level):
        start = node.lineno
        end = top_level[i + 1].lineno - 1 if i + 1 < len(top_level) else len(lines)
        while end > start and not lines[end - 1].strip():
            end -= 1
        ranges[node.name] = (start, end)

    return ranges


def update_readme(sha: str, ranges: dict[str, tuple[int, int]]) -> None:
    content = README.read_text()

    md_link = re.compile(
        r"\[`?(\w+)`?\]\("
        rf"(https://github\.com/{re.escape(REPO)}/blob/[0-9a-f]{{7,40}}/[^)]+)"
        r"\)"
    )

    def replace(m: re.Match) -> str:
        name = m.group(1)
        old_url = m.group(2)
        new_url = re.sub(
            rf"(github\.com/{re.escape(REPO)}/blob/)[0-9a-f]{{7,40}}",
            rf"\g<1>{sha}",
            old_url,
        )
        if name in ranges:
            start, end = ranges[name]
            new_url = re.sub(r"#L\d+(?:-L\d+)?$", f"#L{start}-L{end}", new_url)
        return f"[`{name}`]({new_url})"

    updated = md_link.sub(replace, content)

    if updated == content:
        print("Nothing to update.")
        return

    README.write_text(updated)
    print(f"README updated (SHA: {sha[:7]}).")
    for m in md_link.finditer(content):
        name = m.group(1)
        if name in ranges:
            print(f"  {name}: L{ranges[name][0]}-L{ranges[name][1]}")


if __name__ == "__main__":
    sha = get_latest_sha()
    ranges = get_line_ranges(PYI)
    update_readme(sha, ranges)
