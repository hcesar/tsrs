#!/usr/bin/env node
// Checks the packed npm packages before they are published: unpacks the main package and this platform's package into
// a scratch project's node_modules (the layout an install produces, without the registry), then runs the launcher,
// reads the package version and compiles a project whose tsconfig loads a transformer plugin.
//
//   node tools/fork/npm-smoke.mjs --plugin <path to the examples/transform_plugin library>

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import { createRequire } from "node:module";
import os from "node:os";
import path from "node:path";
import { parseArgs } from "node:util";

const repoRoot = path.resolve(import.meta.dirname, "..", "..");
const { values: opts } = parseArgs({ options: { plugin: { type: "string" } } });
if (!opts.plugin) {
    console.error("usage: node tools/fork/npm-smoke.mjs --plugin <library>");
    process.exit(2);
}

const manifest = JSON.parse(fs.readFileSync(path.join(repoRoot, "npm", "dist", "packages.json"), "utf8"));
const main = manifest.packages.at(-1);
const platform = `${process.platform}-${process.arch}${process.platform === "linux" ? "-gnu" : ""}`;
const native = manifest.packages.find(p => p.name.endsWith(`-${platform}`));
if (!native) throw new Error(`no package for ${platform} in ${manifest.packages.map(p => p.name).join(", ")}`);

const project = fs.mkdtempSync(path.join(os.tmpdir(), "tsrs-npm-smoke-"));
for (const { name, tarball } of [main, native]) {
    const dir = path.join(project, "node_modules", ...name.split("/"));
    fs.mkdirSync(dir, { recursive: true });
    execFileSync("tar", ["-xzf", tarball, "-C", dir, "--strip-components", "1"]);
}
const launcher = path.join(project, "node_modules", ...main.name.split("/"), "bin", "tsrs");
const tsrs = (...args) => execFileSync(process.execPath, [launcher, ...args], { cwd: project, encoding: "utf8" });

const versionLine = tsrs("--version").trim();
console.log(versionLine);
if (!versionLine.startsWith("Version ")) throw new Error(`unexpected --version output: ${versionLine}`);
const reported = createRequire(path.join(project, "index.js"))(main.name).version;
if (reported !== manifest.version) throw new Error(`require("${main.name}").version is ${reported}, expected ${manifest.version}`);

fs.mkdirSync(path.join(project, "src"));
fs.writeFileSync(
    path.join(project, "tsconfig.json"),
    JSON.stringify({
        compilerOptions: {
            target: "esnext",
            module: "esnext",
            rootDir: "src",
            outDir: "out",
            plugins: [{ name: "example-plugin", transform: path.resolve(opts.plugin) }],
        },
        include: ["src"],
    }),
);
fs.writeFileSync(
    path.join(project, "src", "index.ts"),
    `declare function debugLog<T>(value: T): T;
declare function typeLabel(value: unknown): string;
declare const __FILE_NAME__: string;

export function run(items: number[], count: number): string {
    debugLog(__FILE_NAME__);
    return typeLabel(items) + ':' + typeLabel(count);
}
`,
);
tsrs("-p", ".");
const js = fs.readFileSync(path.join(project, "out", "index.js"), "utf8");
for (const expected of [`import { __trace } from "tsrs-runtime";`, "/*#__PURE__*/ __trace(", `return "array" + ':' + "value";`]) {
    if (!js.includes(expected)) throw new Error(`the plugin did not run: expected ${expected} in\n${js}`);
}
fs.rmSync(project, { recursive: true, force: true });
console.log(`${main.name}@${manifest.version} and ${native.name} pass`);
