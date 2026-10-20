#!/usr/bin/env python3
"""Converts Go option declaration lists into Rust statics.

Usage: gen_decls.py [--vars name,...] <go file> <rust file>
Source 17 collects the declarations in tsoptions/declarations_generated.go; --vars selects
the variables that belong in each Rust module. The Go generator moved non-declaration metadata
to options_generated.go, so fields that remain in Rust's combined option table are preserved
from the target. Review those fields against options.ts when an upstream option changes.
Only the `[]*CommandLineOption{...}` and `CommandLineOption{...}` literals are converted; the
surrounding Rust (imports, functions) lives in the hand-written part of the target file between the
`// BEGIN GENERATED` / `// END GENERATED` markers.
"""
import argparse
import re

KIND = {
    "CommandLineOptionTypeString": "String",
    "CommandLineOptionTypeNumber": "Number",
    "CommandLineOptionTypeBoolean": "Boolean",
    "CommandLineOptionTypeObject": "Object",
    "CommandLineOptionTypeList": "List",
    "CommandLineOptionTypeListOrElement": "ListOrElement",
    "CommandLineOptionTypeEnum": "Enum",
    '"string"': "String",
    '"number"': "Number",
    '"boolean"': "Boolean",
    '"object"': "Object",
    '"list"': "List",
    '"enum"': "Enum",
}

CORE_ENUMS = ["ScriptTarget"]
MOVED_METADATA = {
    "affects_declaration_path", "affects_program_structure", "affects_semantic_diagnostics",
    "affects_build_info", "affects_bind_diagnostics", "affects_source_file",
    "affects_module_resolution", "affects_emit", "allow_js_flag", "strict_flag",
    "transpile_option_value", "allow_config_dir_template_substitution",
}


def snake(name):
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    return s.lower()


def tristate(v):
    return {"core.TSUnknown": "Tristate::Unknown", "core.TSTrue": "Tristate::True", "core.TSFalse": "Tristate::False"}[v]


def default_value(v):
    if v in ("true", "false"):
        return f"DefaultValueDescription::Bool({v})"
    if v == "nil":
        return "DefaultValueDescription::None"
    if v.startswith('"'):
        return f"DefaultValueDescription::Str({v})"
    if re.fullmatch(r"-?\d+", v):
        return f"DefaultValueDescription::Int({v})"
    if v.startswith("diagnostics."):
        return f"DefaultValueDescription::Message(&diagnostics::{v[len('diagnostics.'):]})"
    if v.startswith("core.TS"):
        return f"DefaultValueDescription::Tristate({tristate(v)})"
    for e in CORE_ENUMS:
        if v.startswith("core." + e):
            return f"DefaultValueDescription::{e}({e}::{v[len('core.' + e):]})"
    raise Exception("unknown default value " + v)


def convert_field(key, value):
    if key == "PathKind":
        # Rust's current option representation uses IsFilePath for path conversion.
        return None
    if key == "Name":
        return f"name: {value}"
    if key == "ShortName":
        return f"short_name: {value}"
    if key == "Kind":
        return f"kind: CommandLineOptionKind::{KIND[value]}"
    if key in ("Category", "Description"):
        assert value.startswith("diagnostics."), value
        return f"{snake(key)}: Some(&diagnostics::{value[len('diagnostics.'):]})"
    if key == "DefaultValueDescription":
        return f"default_value_description: {default_value(value)}"
    if key == "transpileOptionValue":
        return f"transpile_option_value: {tristate(value)}"
    if key == "extraValidation":
        return "extra_validation: ExtraValidation::" + value[len("extraValidation"):]
    if key == "minValue":
        return f"min_value: {value}"
    if value in ("true", "false"):
        if key == "IsTSConfigOnly":
            return f"is_tsconfig_only: {value}"
        return f"{snake(key)}: {value}"
    raise Exception(f"unknown field {key}: {value}")


def existing_metadata(target):
    generated = target.split("// BEGIN GENERATED\n", 1)[1].split("// END GENERATED", 1)[0]
    result = {}
    pattern = r"CommandLineOption \{\n(.*?)\n\s*\.\.CommandLineOption::DEFAULT\n\s*\}"
    for m in re.finditer(pattern, generated, re.S):
        body = m.group(1)
        name = re.search(r'name: "([^"]+)"', body)
        if name is None:
            continue
        short = re.search(r'short_name: "([^"]+)"', body)
        fields = {}
        for line in body.splitlines():
            field = re.fullmatch(r"\s*(\w+):\s*(.*),\s*", line)
            if field and field.group(1) in MOVED_METADATA:
                fields[field.group(1)] = field.group(2)
        result[(name.group(1), short.group(1) if short else "")] = fields
    return result


def convert_literal(body, indent, previous):
    fields = []
    for line in body.split("\n"):
        line = re.sub(r"//.*$", "", line).strip()
        if not line:
            continue
        m = re.fullmatch(r"(\w+):\s*(.+?),?", line)
        if not m:
            raise Exception("cannot parse line: " + line)
        field = convert_field(m.group(1), m.group(2))
        if field is not None:
            fields.append(field)
    name = re.search(r'\bName:\s*"([^"]+)"', body)
    short = re.search(r'\bShortName:\s*"([^"]+)"', body)
    if name:
        present = {field.split(":", 1)[0] for field in fields}
        for key, value in previous.get((name.group(1), short.group(1) if short else ""), {}).items():
            if key not in present:
                fields.append(f"{key}: {value}")
    pad = " " * (indent + 4)
    out = "CommandLineOption {\n"
    for f in fields:
        out += f"{pad}{f},\n"
    out += f"{pad}..CommandLineOption::DEFAULT\n"
    out += " " * indent + "}"
    return out


def scream(n):
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", n)
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    return s.upper()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--vars", help="comma-separated Go declarations to include")
    parser.add_argument("go_file")
    parser.add_argument("rust_file")
    args = parser.parse_args()
    selected = set(args.vars.split(",")) if args.vars else None
    go = open(args.go_file).read()
    target = open(args.rust_file).read()
    previous = existing_metadata(target)
    out = []
    # single named literals: var X = CommandLineOption{ ... }
    for m in re.finditer(r"^var (\w+) = CommandLineOption\{\n(.*?)\n\}", go, re.S | re.M):
        name, body = m.group(1), m.group(2)
        if selected is not None and name not in selected:
            continue
        vis = "pub " if name[0].isupper() else "pub(crate) "
        name = scream(name)
        out.append(f"{vis}static {name}: CommandLineOption = {convert_literal(body, 0, previous)};\n")
    for m in re.finditer(r"^var (\w+) = \[\]\*CommandLineOption\{\n(.*?)\n\}\n", go, re.S | re.M):
        name, body = m.group(1), m.group(2)
        if selected is not None and name not in selected:
            continue
        vis = "pub " if name[0].isupper() else "pub(crate) "
        name = scream(name)
        items = []
        # split top-level elements
        depth = 0
        cur = None
        for line in body.split("\n"):
            stripped = re.sub(r"//.*$", "", line).strip()
            if depth == 0:
                if stripped == "{":
                    depth = 1
                    cur = []
                    continue
                if stripped.startswith("&"):
                    items.append(("ref", stripped[1:].rstrip(",")))
                    continue
                if stripped == "":
                    continue
                raise Exception("unexpected top-level line: " + line)
            else:
                if stripped in ("},", "}"):
                    depth = 0
                    items.append(("lit", "\n".join(cur)))
                    cur = None
                    continue
                cur.append(line)
        refs = [i for i in items if i[0] == "ref"]
        lits = [i for i in items if i[0] == "lit"]
        # Literal elements go into a static array (stable addresses); the list itself is a
        # LazyLock<Vec<&'static CommandLineOption>> preserving the Go order, including &Named refs.
        arr = f"{name}_ITEMS"
        s = f"static {arr}: [CommandLineOption; {len(lits)}] = [\n"
        for _, body in lits:
            s += "    " + convert_literal(body, 4, previous) + ",\n"
        s += "];\n\n"
        s += f"{vis}static {name}: LazyLock<Vec<&'static CommandLineOption>> = LazyLock::new(|| {{\n"
        s += "    vec![\n"
        li = 0
        for kind, v in items:
            if kind == "ref":
                s += f"        &{scream(v)},\n"
            else:
                s += f"        &{arr}[{li}],\n"
                li += 1
        s += "    ]\n});\n"
        out.append(s)
    text = "\n".join(out)
    begin = target.index("// BEGIN GENERATED\n") + len("// BEGIN GENERATED\n")
    end = target.index("// END GENERATED")
    open(args.rust_file, "w").write(target[:begin] + text + target[end:])


main()
