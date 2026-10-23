#!/usr/bin/env node
// Packs the fork's npm packages. Upstream's npm/build.mjs stages `tsrs` and `@tsrs/<platform>`; this renames them to
// <name> and <platform-prefix><platform>, since npm/build.mjs and the launcher hard-code `@tsrs/` and the fork leaves
// upstream files alone.
//
//   node tools/fork/npm-pack.mjs --artifacts <dir> --name <@scope/name> --platform-prefix <@scope/prefix-> [--version <v>]
//
// --version overrides npm/build.mjs's version (a fix release `<version>.<n>`, or a test release). Writes the tarballs and
// packages.json (publish order: platform packages, then the main package) to npm/dist.

import { execFileSync, spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { parseArgs } from "node:util";

const REPOSITORY = { type: "git", url: "git+https://github.com/hcesar/tsrs-with-transformers.git" };

const repoRoot = path.resolve(import.meta.dirname, "..", "..");
const out = path.join(repoRoot, "npm", "dist");
const { values: opts } = parseArgs({
    options: {
        artifacts: { type: "string" },
        name: { type: "string" },
        "platform-prefix": { type: "string" },
        version: { type: "string" },
    },
});
if (!opts.artifacts || !opts.name || !opts["platform-prefix"]) {
    console.error("usage: node tools/fork/npm-pack.mjs --artifacts <dir> --name <@scope/name> --platform-prefix <@scope/prefix-> [--version <v>]");
    process.exit(2);
}

execFileSync(process.execPath, [path.join(repoRoot, "npm", "build.mjs"), "--artifacts", opts.artifacts, "--out", out], { stdio: "inherit" });
const staged = JSON.parse(fs.readFileSync(path.join(out, "packages.json"), "utf8"));
const version = opts.version ?? staged.version;

const rename = name => (name.startsWith("@tsrs/") ? opts["platform-prefix"] + name.slice("@tsrs/".length) : opts.name);
const readJson = file => JSON.parse(fs.readFileSync(file, "utf8"));
const writeJson = (file, value) => fs.writeFileSync(file, JSON.stringify(value, undefined, 4) + "\n");

const packages = [];
for (const { name: oldName, dir } of staged.packages) {
    const name = rename(oldName);
    const manifestFile = path.join(dir, "package.json");
    const manifest = readJson(manifestFile);
    manifest.name = name;
    manifest.version = version;
    // npm provenance requires the repository to be the one that publishes.
    manifest.repository = REPOSITORY;
    if (manifest.optionalDependencies) {
        manifest.optionalDependencies = Object.fromEntries(Object.keys(manifest.optionalDependencies).map(dep => [rename(dep), version]));
    }

    if (oldName.startsWith("@tsrs/")) {
        const platform = oldName.slice("@tsrs/".length);
        manifest.description = `The ${platform} binary for ${opts.name}`;
        fs.writeFileSync(
            path.join(dir, "README.md"),
            `# ${name}\n\nThe \`${platform}\` binary of [\`${opts.name}\`](https://www.npmjs.com/package/${opts.name}). ` +
                `Do not install this package directly: install \`${opts.name}\`, which depends on it through ` +
                "`optionalDependencies`.\n",
        );
    }
    else {
        manifest.description = "tsrs with ttsc-style native transformer plugins";
        for (const file of ["lib/getExePath.js", "lib/platform.js"]) {
            const full = path.join(dir, file);
            fs.writeFileSync(full, fs.readFileSync(full, "utf8").replaceAll("@tsrs/", opts["platform-prefix"]));
        }
        const leftovers = spawnSync("grep", ["-rl", "--", "@tsrs/", "lib", "bin", "dist"], { cwd: dir, encoding: "utf8" }).stdout.trim();
        if (leftovers) throw new Error(`the launcher still names @tsrs/ in: ${leftovers}`);
    }
    writeJson(manifestFile, manifest);

    const packed = JSON.parse(execFileSync("pnpm", ["pack", "--json", "--pack-destination", out], { cwd: dir, encoding: "utf8" }));
    const tarball = path.resolve(out, packed.filename);
    if (oldName.startsWith("@tsrs/")) {
        // Some pnpm versions (12.5.1) pack the binary without its executable bit.
        const exe = manifest.files[0];
        const listing = execFileSync("tar", ["-tvzf", tarball], { encoding: "utf8" }).split("\n");
        if (!listing.some(line => line.startsWith("-rwx") && line.endsWith(` package/${exe}`))) {
            throw new Error(`${path.basename(tarball)}: package/${exe} is not executable`);
        }
    }
    packages.push({ name, dir, tarball });
    console.log(`packed ${path.relative(repoRoot, tarball)}`);
}
writeJson(path.join(out, "packages.json"), { version, packages });
