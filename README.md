> **This is a fork of [oxc-project/tsrs](https://github.com/oxc-project/tsrs).**
> It adds native AST transformer plugins declared in `tsconfig.json`, with the `plugins[].transform`
> syntax of [ttsc](https://ttsc.dev/) for TypeScript-Go. Everything else is upstream `tsrs` at its latest release.
> The upstream README follows the fork section.

## Fork: transformer plugins

A plugin is a shared library (`.dylib` / `.so`) that runs inside the emit pipeline. It reads the AST, queries the type
checker and returns a rewritten source file:

```jsonc
{
  "compilerOptions": {
    "plugins": [
      // Before the TypeScript transforms (type erasure, import elision, decorators, class fields…)
      { "transform": "./plugins/libmy_plugin.dylib" },
      // After the TypeScript and downlevel transforms, before the module transform
      { "transform": "${configDir}/plugins/libmy_plugin.dylib", "after": true },
      // On the .d.ts output
      { "transform": "./plugins/libmy_plugin.dylib", "afterDeclarations": true }
    ]
  }
}
```

`transform` is a library path, resolved from the tsconfig's directory. Install from npm (Linux x64 and arm64, macOS
arm64); the package provides the `tsrs` command, and its version is the upstream `tsrs` release it is built from:

```sh
npm i -D tsrs-with-transformers
npx tsrs -p tsconfig.json
```

Or build from source:

```sh
cargo build --release -p tsrs_cli
target/release/tsrs -p tsconfig.json
```

To write a plugin, build a Rust `cdylib` that depends only on
[`crates/tsrs_plugin_api`](crates/tsrs_plugin_api/src/lib.rs). That crate is the C ABI, and it has no dependencies.
The plugin exports `tsrs_plugin_transform`. [`examples/transform_plugin`](examples/transform_plugin/src/lib.rs) is a
complete example: it rewrites calls, injects an import and queries the checker. Plugins run on macOS and Linux.

What the fork changes, kept small so it rebases cleanly:

| where | what |
| --- | --- |
| `crates/tsrs_plugin_api` (new) | the C ABI and safe `Api` wrapper that plugins build against |
| `crates/tsrs_plugin_host` (new) | the library loader and the host side of the ABI, with its tests |
| `examples/transform_plugin`, `crates/tsrs_cli/tests/plugin_emit.rs` (new) | an example plugin and an end-to-end CLI test that loads it |
| `crates/tsrs_core/src/compileroptions.rs` | `PluginImport` keeps `transform`, `after` and `afterDeclarations` |
| `crates/tsrs_tsoptions/src/{parsinghelpers,gojson}.rs` | read and write those three fields |
| `crates/tsrs_compiler/src/emitter.rs`, `Cargo.toml` | three one-line calls into `tsrs_plugin_host` |
| `tools/lint/unsafe_sites.tsv` | generated rows for the new crates' `unsafe` blocks (in releases that have the file) |
| `tools/fork` (new) | the rebase helper and the npm packaging (`tsrs-with-transformers`, `@tsrs-with-transformers/compiler-*`) |

The plugin API numbers syntax kinds on its own, append-only scale, and the host translates them by name. So when
upstream inserts or reorders kinds, existing plugin binaries keep working.

`main` is the newest upstream release tag plus one fork commit, so it never carries unreleased upstream code. To move
it to a new release: `git fetch upstream --tags && node tools/fork/rebase.mjs <release tag>`. It regenerates
`Cargo.lock` and `unsafe_sites.tsv` itself and stops on any other conflict. Then run `cargo test -p tsrs_plugin_host` and
`cargo test -p tsrs_cli --test plugin_emit`. `tsrs_plugin_host`'s kind test names any new kind to append to
`crates/tsrs_plugin_api/src/kind.rs`.

---

# `tsrs`

[![npm](https://img.shields.io/npm/v/tsrs)](https://www.npmjs.com/package/tsrs)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue)](https://github.com/oxc-project/tsrs/blob/main/LICENSE)

`tsrs` is a drop-in replacement for the TypeScript 7 compiler and language server, written in Rust.
Our goal is 100% conformance, with much lower memory use and faster type checking.

## Get started

```sh
pnpm add -D tsrs
pnpm exec tsrs --noEmit
```

You can try `tsrs` today and give us feedback on performance, memory usage, and conformance.
Try checking and emitting code in the [browser playground](https://tsrs-playground.void.app).

## Conformance

`tsrs` currently targets the main branch of [microsoft/TypeScript](https://github.com/microsoft/typescript).
We aim to release a stable version against TypeScript 7.1 with 100% conformance. In our testing on existing projects,
we have yet to find a correctness bug in `tsrs`.

There are known order issues: the pinned Go compiler can produce different diagnostics and traces depending on
parallel parse, resolution, and check order, while `tsrs` canonicalizes some checker history by default. For the
closest comparison, use `--checkerAssignment go --noLazyMembers`; tsgo parser and resolver races can still vary
between runs.

`--init` and `--showConfig` are not supported. See [compatibility and evidence](https://github.com/oxc-project/tsrs/blob/main/docs/COMPATIBILITY.md)
for supported features and known gaps.

<a href="https://github.com/oxc-project/tsrs/blob/main/bench/RESULTS.md">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/oxc-project/tsrs/main/bench/large-projects-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/oxc-project/tsrs/main/bench/large-projects.svg">
    <img alt="tsrs vs the TypeScript nightly: times faster against times less peak memory on seven projects at 1, 2, 4 and 8 parallel checker threads" src="https://raw.githubusercontent.com/oxc-project/tsrs/main/bench/large-projects.svg">
  </picture>
</a>

## Extra features

[`@tsrs/wasm`](https://github.com/oxc-project/tsrs/blob/main/npm/tsrs-wasm/README.md) provides WebAssembly support
targeting `wasm32-wasip1` (WASI Preview 1).

## Learn more

- [CLI usage and memory tuning](https://github.com/oxc-project/tsrs/blob/main/docs/USAGE.md)
- [Editor setup](https://github.com/oxc-project/tsrs/blob/main/docs/LSP.md#editor-setup)
- [Node API (unstable)](https://github.com/oxc-project/tsrs/blob/main/docs/NODE_API.md)
- [WebAssembly for Node and browsers](https://github.com/oxc-project/tsrs/blob/main/npm/tsrs-wasm/README.md)
- [Benchmarks and methodology](https://github.com/oxc-project/tsrs/blob/main/bench/README.md)
- [Release notes](https://github.com/oxc-project/tsrs/blob/main/docs/STATUS.md)
- [Engineering notes](https://github.com/oxc-project/tsrs/blob/main/notes/README.md)

## Contributing

Issues and pull requests are currently disabled until we have the bandwidth to triage them.

See the [contributing guide](https://github.com/oxc-project/tsrs/blob/main/CONTRIBUTING.md)
for building from source and running the suites, the [Code of Conduct](https://github.com/oxc-project/tsrs/blob/main/CODE_OF_CONDUCT.md), and
[security policy](https://github.com/oxc-project/tsrs/blob/main/SECURITY.md) for reporting vulnerabilities.

## Acknowledgments

We thank the following organizations and contributors:

<p>
  <a href="https://depot.dev">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/oxc-project/tsrs/main/assets/sponsors/depot-dark.svg">
      <img alt="Depot" src="https://raw.githubusercontent.com/oxc-project/tsrs/main/assets/sponsors/depot-light.svg" height="32">
    </picture>
  </a>
  &nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://www.owner.com">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/oxc-project/tsrs/main/assets/sponsors/owner-dark.svg">
      <img alt="Owner.com" src="https://raw.githubusercontent.com/oxc-project/tsrs/main/assets/sponsors/owner-light.svg" height="32">
    </picture>
  </a>
</p>

- [Depot CI](https://depot.dev) for sponsoring our continuous integration.
- [Owner.com](https://www.owner.com) for sponsoring the initial work.
- [Max Schwenk](https://github.com/maschwenk) for the port.
- [@walkerdb](https://github.com/walkerdb) for improvements to the project.
- [@JiangWeixian](https://github.com/JiangWeixian) for transferring the `tsrs` npm package name.

We extend special thanks to the [TypeScript team](https://www.typescriptlang.org/) for their support.

## License

`tsrs` is a derivative of TypeScript (Copyright Microsoft Corporation), licensed under [Apache-2.0](https://github.com/oxc-project/tsrs/blob/main/LICENSE).
The bundled `lib.*.d.ts` files are TypeScript's. See [NOTICE](https://github.com/oxc-project/tsrs/blob/main/NOTICE) for attribution.
Not affiliated with or endorsed by Microsoft.
