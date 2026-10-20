#!/usr/bin/env python3
"""Import a completed OneLoc handback without discarding pending translations.

This is the Python counterpart of TypeScript's tools/cmd/import-localization at
cb72b2e17b. The OneLoc pipeline itself belongs to the upstream repository.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import xml.etree.ElementTree as ET


def read_messages(path: Path) -> dict[str, str]:
    messages = json.loads(path.read_bytes().removeprefix(b"\xef\xbb\xbf"))
    if not isinstance(messages, dict) or not all(
        isinstance(key, str) and isinstance(value, str)
        for key, value in messages.items()
    ):
        raise ValueError(f"{path}: expected a JSON object of strings")
    return messages


def local_name(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


def read_metadata(path: Path) -> dict[str, str | None]:
    root = ET.parse(path).getroot()
    if local_name(root.tag) != "LCX":
        raise ValueError(f"{path}: expected an LCX document")

    messages: dict[str, str | None] = {}

    def visit(parent: ET.Element) -> None:
        for item in parent:
            if local_name(item.tag) != "Item":
                continue
            message = next((child for child in item if local_name(child.tag) == "Str"), None)
            if message is not None:
                item_id = item.get("ItemId", "")
                if not item_id.startswith(";") or len(item_id) == 1:
                    raise ValueError(f"{path}: invalid resource ID {item_id!r}")
                key = item_id[1:]
                if key in messages:
                    raise ValueError(f"{path}: duplicate resource ID {item_id!r}")
                target = next((child for child in message if local_name(child.tag) == "Tgt"), None)
                messages[key] = None if target is None else target.get("Stat", "")
            visit(item)

    visit(root)
    return messages


def import_localization(source: Path, handback: Path, output: Path) -> None:
    english = read_messages(source)
    returned = read_messages(handback)
    metadata = read_metadata(Path(f"{handback}.lct"))
    existing = read_messages(output) if output.exists() else {}
    existing = {key: value for key, value in existing.items() if key in english}

    for key, translation in returned.items():
        if key not in english:
            continue
        status = metadata.get(key)
        if status is None:
            raise ValueError(f"{handback}: missing localization metadata for {key!r}")
        if status == "Loc":
            existing[key] = translation
        elif status not in ("", "Update"):
            raise ValueError(f"{handback}: unknown localization status {status!r} for {key!r}")

    existing = {key: value for key, value in existing.items() if value != english[key]}
    data = json.dumps(existing, ensure_ascii=False, indent=2, sort_keys=True)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(data.replace("\n", "\r\n").encode())


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", required=True, type=Path)
    parser.add_argument("--handback", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    import_localization(args.source, args.handback, args.out)


if __name__ == "__main__":
    main()
