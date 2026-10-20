#!/usr/bin/env python3
"""Validate imported source-32 locale catalogs against Rust diagnostics."""

import gzip
import json
from pathlib import Path
import re


ROOT = Path(__file__).resolve().parents[2]
LOCALES = ROOT / "crates/tsrs_diagnostics/loc"
GENERATED = ROOT / "crates/tsrs_diagnostics/src/generated.rs"
LOCALE_NAMES = {
    "cs-CZ", "de-DE", "es-ES", "fr-FR", "it-IT", "ja-JP", "ko-KR",
    "pl-PL", "pt-BR", "ru-RU", "tr-TR", "zh-CN", "zh-TW",
}
MESSAGE_PATTERN = re.compile(
    r'key: Key\(\s*("(?:[^"\\]|\\.)*")\s*,?\s*\),\s*'
    r'text:\s*("(?:[^"\\]|\\.)*")'
)
PLACEHOLDER_PATTERN = re.compile(r"\{\d+\}")


def main() -> None:
    generated = GENERATED.read_text(encoding="utf-8")
    pairs = MESSAGE_PATTERN.findall(generated)
    assert len(pairs) == generated.count("key: Key("), "could not parse all Rust diagnostics"
    english = {json.loads(key): json.loads(text) for key, text in pairs}
    assert len(english) == len(pairs), "duplicate diagnostic keys"

    files = {path.name for path in LOCALES.iterdir() if path.is_file()}
    expected_files = {
        f"{name}{suffix}"
        for name in LOCALE_NAMES
        for suffix in (".generated.json", ".json.gz")
    }
    assert files == expected_files, f"locale files differ: {files ^ expected_files}"

    for name in sorted(LOCALE_NAMES):
        handback = (LOCALES / f"{name}.generated.json").read_bytes()
        messages = json.loads(handback)
        assert isinstance(messages, dict)
        assert all(isinstance(key, str) and isinstance(value, str) for key, value in messages.items())
        canonical = json.dumps(messages, ensure_ascii=False, sort_keys=True, indent=2)
        assert handback == canonical.replace("\n", "\r\n").encode(), f"{name}: noncanonical handback"
        for key, translated in messages.items():
            if key in english:
                assert set(PLACEHOLDER_PATTERN.findall(translated)) == set(
                    PLACEHOLDER_PATTERN.findall(english[key])
                ), f"{name}: placeholder mismatch for {key}"

        archive = (LOCALES / f"{name}.json.gz").read_bytes()
        assert archive[:4] == b"\x1f\x8b\x08\x00" and archive[4:8] == b"\x00" * 4, f"{name}: gzip header"
        payload = gzip.decompress(archive)
        runtime = json.loads(payload)
        active = {key: value for key, value in messages.items() if key in english}
        assert runtime == active, f"{name}: runtime/handback mismatch"
        assert payload == json.dumps(active, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode(), (
            f"{name}: noncanonical runtime JSON"
        )

    print(f"validated {len(LOCALE_NAMES)} locale pairs against {len(english)} Rust diagnostics")


if __name__ == "__main__":
    main()
