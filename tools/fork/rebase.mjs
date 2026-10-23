#!/usr/bin/env node
// Rebases the fork's commits: `node tools/fork/rebase.mjs <onto> [<base>]` runs `git rebase --onto <onto> <base>`
// (base defaults to onto). Conflicts in the generated Cargo.lock and tools/lint/unsafe_sites.tsv take the upstream
// side and are regenerated; any other conflict aborts the rebase, lists the files in conflicts.txt and exits 1.

import { execFileSync, spawnSync } from "node:child_process";
import fs from "node:fs";

const GENERATED = ["Cargo.lock", "tools/lint/unsafe_sites.tsv"];

const [onto, base = onto] = process.argv.slice(2);
if (!onto) {
    console.error("usage: node tools/fork/rebase.mjs <onto> [<base>]");
    process.exit(2);
}

const git = (...args) => execFileSync("git", args, { encoding: "utf8" }).trim();
const tryGit = (...args) => spawnSync("git", args, { stdio: "inherit", env: { ...process.env, GIT_EDITOR: "true", GIT_SEQUENCE_EDITOR: "true" } }).status === 0;
const inHead = file => spawnSync("git", ["cat-file", "-e", `HEAD:${file}`]).status === 0;

function regenerate() {
    execFileSync("cargo", ["metadata", "--format-version", "1"], { stdio: ["ignore", "ignore", "inherit"] });
    if (fs.existsSync("tools/lint/unsafe_sites.tsv")) {
        // Upstream moved this lint from Python to JavaScript; older release tags still have the Python one.
        if (fs.existsSync("tools/lint/source.mjs")) {
            execFileSync(process.execPath, ["tools/lint/source.mjs", "--update"], { stdio: "inherit" });
        }
        else {
            execFileSync("python3", ["tools/lint/source.py", "--update"], { stdio: "inherit" });
        }
    }
}

function stop(conflicts) {
    fs.writeFileSync("conflicts.txt", conflicts.map(file => `${file}\n`).join(""));
    tryGit("rebase", "--abort");
    console.error(`rebase onto ${onto} stopped${conflicts.length ? `: conflicts in ${conflicts.join(", ")}` : ""}`);
    process.exit(1);
}

let done = tryGit("rebase", "--onto", onto, base);
while (!done) {
    const conflicts = git("diff", "--name-only", "--diff-filter=U").split("\n").filter(Boolean);
    if (conflicts.length === 0 || conflicts.some(file => !GENERATED.includes(file))) stop(conflicts);
    // During a rebase, "ours" is the upstream side. A file upstream does not have (yet) is dropped.
    for (const file of conflicts) {
        if (inHead(file)) git("checkout", "--ours", "--", file);
        else git("rm", "--quiet", "--", file);
    }
    regenerate();
    git("add", "--", ...GENERATED.filter(file => fs.existsSync(file)));
    done = tryGit("rebase", "--continue");
}

// A clean merge can still leave a generated file stale (an upstream version bump changes every Cargo.lock entry of
// the workspace). Fold the refresh into the fork commit that last touched the file, or the first fork commit.
regenerate();
const stale = GENERATED.filter(file => fs.existsSync(file) && git("status", "--porcelain", "--", file));
for (const file of stale) {
    const owner = git("log", "-1", "--format=%H", `${onto}..HEAD`, "--", file)
        || git("rev-list", "--reverse", `${onto}..HEAD`).split("\n")[0];
    git("add", "--", file);
    git("commit", "--quiet", `--fixup=${owner}`, "--", file);
}
if (stale.length && !tryGit("rebase", "--quiet", "--interactive", "--autosquash", onto)) stop([]);
