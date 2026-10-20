import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { createTypeScriptModuleLoader } from "../../src/vscode/extensionApi.ts";

function createPackage(t: test.TestContext, name: string, value: number): string {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), "tsrs-module-loader-"));
    t.after(() => fs.rmSync(root, { recursive: true, force: true }));
    fs.mkdirSync(path.join(root, "dist"));
    fs.writeFileSync(path.join(root, "package.json"), JSON.stringify({
        name,
        type: "module",
        exports: { "./test": "./dist/api.js" },
    }));
    fs.writeFileSync(path.join(root, "dist", "api.js"), `export const value = ${value};\n`);
    fs.writeFileSync(path.join(root, "dist", "private.js"), "export const privateValue = true;\n");
    return path.join(root, "package.json");
}

test("loads and caches the selected package's exported API module", async t => {
    const first = createPackage(t, "typescript-selected", 1);
    const second = createPackage(t, "tsrs", 2);
    const firstLoader = createTypeScriptModuleLoader(first);
    const firstModule = await firstLoader.importModule("typescript/test");
    assert.equal((firstModule as { value: number; }).value, 1);
    assert.equal(await firstLoader.importModule("typescript/test"), firstModule);
    const secondModule = await createTypeScriptModuleLoader(second).importModule("typescript/test");
    assert.equal((secondModule as { value: number; }).value, 2);
    await assert.rejects(firstLoader.importModule("typescript/private"), { code: "ERR_PACKAGE_PATH_NOT_EXPORTED" });
});

test("validates the full TypeScript API specifier", async t => {
    const loader = createTypeScriptModuleLoader(createPackage(t, "typescript-selected", 1));
    for (const specifier of ["", "unstable/test", "typescript/", "./test", "typescript/../test", "typescript/unstable\\test"]) {
        await assert.rejects(loader.importModule(specifier), new Error(`Invalid TypeScript API module export path '${specifier}'.`));
    }
});

test("reports a selected manifest without a package name", async t => {
    const manifest = createPackage(t, "typescript-selected", 1);
    fs.writeFileSync(manifest, "{}");
    await assert.rejects(
        createTypeScriptModuleLoader(manifest).importModule("typescript/test"),
        new Error(`TypeScript API package manifest at '${manifest}' does not contain a package name.`),
    );
});
