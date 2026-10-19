# TypeScript ports for issue 623

Port [issue 623](https://github.com/oxc-project/tsrs/issues/623) in upstream chronological order, one tsrs commit per
upstream commit, including evidence-only commits for changes that need no Rust code. Each corresponding commit has
an `Upstream-Commit` trailer. Assign each upstream commit to a dedicated subagent, and finish its validation and
corresponding commit before starting the next subagent. Start at `b85298b6a81f772d080b0455de0ca9d744cd6fd6`; finish at
`5237fd46890956e4bb8177a1a05dcd69e4166560` (112 commits). Keep the published TypeScript pin until the final entry.

Each entry must retain every passing checker baseline in both histories and pass the emit baselines, workspace
checks, Clippy, source checks, and generated-code verification. Run the affected subsystem's additional gates.
Performance compares release binaries on `vscode`, `webpack`, `Compiler-Unions`, and `t3code-server`, single-threaded
and default: one warmup and ten alternating before/after pairs per cell. Compare median instructions retired, wall
time, and peak RSS. Fix confirmed performance regressions or other gate failures in the corresponding commit,
document the cause and fix in its commit message, and revalidate before advancing. Keep all measurement artifacts.
Measurements use macOS `/usr/bin/time -l` for instructions and RSS, and a monotonic high-resolution clock around the
command for wall time. For a no-op port, byte-identical compiler binaries and runtime assets establish no
performance regression without timing the same executable twice. Reuse other completed checks only when their
inputs are demonstrably unchanged; compare complete passing sets rather than totals alone.

After the identical-binary control in entry 2 also failed the original zero-tolerance median rule, the user
instructed removal of that blocking rule and continuation without human guidance. Routine migration work requires
no further human approval. The replacement measurement policy is `paired-sign-test-v1`:

- Retain all three raw before/after medians and every pair in the initial eight cells.
- For each metric, compute paired relative changes and the exact one-sided sign-test probability
  `P[Binomial(n_non_tied, 0.5) >= positive_pairs]`, excluding exactly equal pairs.
- A positive raw median change screens for confirmation when that probability is at most 0.10.
- Run exactly one independent ten-pair confirmation batch per screened cell against the same preceding binary,
  reversing the initial pair order. Evaluate every screened metric in that cell.
- A reproducible regression requires positive raw median changes in both batches and a pooled paired sign-test
  probability at most 0.01. Diagnose and repair confirmed regressions faithfully, retaining the failed evidence.
  Do not repeat unchanged candidates until a favorable batch appears.
- Otherwise report **no detected regression**, not proof of exact equality. Keep old strict-rule records unchanged;
  new verdicts identify their input files by SHA-256. Binary/runtime identity still suffices for a no-op port.

The reusable local evaluator and confirmation collector are `target/port-623/noise_gate.py`; the local manifest
records the current policy and superseded rule. Evaluator sanity checks cover identical samples, consistent
increases, exact binomial tails, excluded ties and mixed samples.

Prerequisite `55e77eaf` isolates the macOS watch fixtures and replaces the OS-dependent event-drop stress test with
deterministic callback injection. No compiler behavior changes. Workspace: 1,064 passed, four existing ignored;
checker passes: 13,458 diagnostics and 12,779 types/symbols in Go history (12,778 types/symbols in canonical history);
emit: 13,462 diagnostics, 13,392 JS/declaration outputs, 149 JS maps, and 156 source maps. No crashes or timeouts.

## 1. Azure Node installation task — passed

Upstream: `9adc871ff47f79b3e99ff2b1cbe9efb140d76af8` — Update Azure pipeline Node installation task (#64547).

Only `tools/pipelines/steps/setup-node-npm-ci.yml` and `tools/pipelines/steps/setup-vsce.yml` change: `NodeTool@0`
becomes `UseNode@1`, and its version input is renamed. tsrs has no corresponding Azure pipelines. No runtime or
fixture changes are needed: the upstream `tsc` tree is `ca347d58f39812fa168e48469f3d2c442467c9ee` at both the old pin
and this commit. The tsrs change is this evidence record; the compiler binary is unchanged.

Validation retained all baseline passes in both checker histories: 13,458 diagnostics and 12,779 types/symbols in
Go history; 13,458 diagnostics and 12,778 types/symbols in canonical history. The existing four diagnostic
differences and canonical `objectLiteralNormalization` type/symbol difference remain unchanged. The emit run
retained 13,462 diagnostics, 12,779 types/symbols, 13,392 JS/declaration outputs, 149 JS maps, and 156 source maps.
All complete passing sets match the baseline; no crashes or timeouts occurred. The unchanged workspace inputs
retain the prerequisite's 1,064 passing checks, four existing ignored checks, native/WASM checks, generated-code
freshness, and regression cases. Clippy, source checks, and formatting also pass.

Performance passes by binary and runtime-asset identity. The baseline and candidate release binaries both have
SHA-256 `ca8457b2c4540510e4ff45c2a111c97b18ea963f9af8c9bea5c8226c9622e23f`. All 113 bundled library files are
byte-identical; their sorted path-to-SHA-256 JSON manifest has SHA-256
`07e760b2613fce61b0eaa87c4cd965264d812ae4fb8b9dc522b1ba5e37e8c6a7`. These libraries are embedded in the default
release compiler. No compiler or runtime changes, additional fixes, or capability changes were needed.

Candidate conformance, pass-set comparison, release build, and identity artifacts are retained locally under
`target/port-623/0001-9adc871ff4/`; the preceding release binary and bundled-library snapshot are under
`target/port-623/baseline/`. Inherited workspace and generated-code evidence is under
`target/port-623/watch-diagnosis/`. The published TypeScript pin remains unchanged.

## 2. Late expando alias exports — passed

Upstream: `697b847e11e91dfbff79f1d189b802b751999f03` — Add export modifiers to other expando exports of
namespaces on all codepaths which add export declarations (#64545).

The declaration transformer checks for an existing export declaration before the alias branch and calls
`add_export_modifier_to_expando_members` before adding either an alias export or a renamed export. Earlier
namespace members remain exported; the initial variable of a renamed export remains private. The new upstream
`declarationEmitLateExpandoAliasAddsExports` case fails its declaration output before the port and passes after it.
The ignored reference checkout advances to this source revision; the published TypeScript pin stays unchanged.

Both checker histories retain every previous passing baseline and add the new case: 13,459 diagnostics and
12,780 types/symbols in Go history; 13,459 diagnostics and 12,779 types/symbols in canonical history. The four
existing checker diagnostic differences and canonical `objectLiteralNormalization` difference are unchanged.
Full emit passes 13,463 diagnostics, 12,780 types/symbols, 13,393 JS/declaration outputs, 149 JS maps and 156 source
maps. Complete pass-set comparisons find no losses, crashes or timeouts. All ten expando declaration oracle cases
match a Go compiler built at this revision. Workspace checks pass (1,064 passed, four existing ignored), as do
native/WASM checks, Clippy, source checks, formatting and all 36 regressions. Regeneration leaves every crate file
byte-identical to its pre-generation snapshot. No additional source fixes were required.

The original release comparison stopped at its first cell because the superseded strict gate rejected any median increase. VS Code,
single-threaded, ten alternating pairs after warmup: instructions 79,564,397,188.5 → 79,568,375,304.5 (+0.0050%);
wall 7.4676780835 → 7.447164208 seconds (-0.2747%); peak RSS 1,685,700,608 → 1,685,700,608 bytes. The VS Code
configuration enables neither `declaration` nor `composite`, so the changed declaration transformer does not run
in this `--noEmit --incremental false` workload.

An identical-binary control using the same saved predecessor path and SHA-256 on both sides also failed that
gate: instructions 79,517,198,584 → 79,515,883,546.5 (-0.0017%); wall 7.4730112505 → 7.478658208 seconds
(+0.0756%); peak RSS remains 1,685,700,608 bytes. Both batches retain all ten pairs; no tolerance or unchanged
candidate retries were introduced. The remaining seven cells were then collected once as evidence, retaining
the original VS Code single-threaded cell and every strict failure flag. Those original records retain their
`acceptance_passed=false`; the revised-policy verdict is separate.

| Project / mode | Instructions, billions before → after | Wall seconds before → after | Peak RSS MiB before → after | Strict increases |
| --- | ---: | ---: | ---: | --- |
| vscode / single | 79.564397 → 79.568375 | 7.467678 → 7.447164 | 1607.609 → 1607.609 | instructions |
| vscode / default | 91.443870 → 91.373049 | 1.006581 → 0.999924 | 2108.453 → 2109.805 | RSS |
| webpack / single | 10.566873 → 10.566473 | 0.905326 → 0.889525 | 311.344 → 311.328 | none |
| webpack / default | 14.680447 → 14.527130 | 0.200681 → 0.199886 | 498.531 → 493.344 | none |
| Compiler-Unions / single | 3.211941 → 3.211971 | 0.259233 → 0.263429 | 59.625 → 59.625 | instructions, wall |
| Compiler-Unions / default | 3.885955 → 3.885974 | 0.148487 → 0.145966 | 83.031 → 83.203 | instructions, RSS |
| t3code-server / single | 38.137103 → 38.372979 | 3.164334 → 3.173378 | 728.703 → 728.703 | instructions, wall |
| t3code-server / default | 122.113708 → 121.988545 | 1.351808 → 1.324767 | 2096.805 → 2091.438 | none |

The t3code-server configuration also has neither declaration nor composite enabled. Compiler-Unions enables
declaration, but its 41 semantic diagnostics prevent the declaration-diagnostics phase from running. These
configuration facts do not exclude whole-binary layout effects from the release rebuild. In single-threaded
t3code-server, eight of ten pairs increase instructions and wall time; their respective paired ranges are
-0.0156% to +0.8920% and -1.0021% to +8.3221%. Compiler-Unions has five increasing instruction pairs in each
mode. These initial batches establish no performance cause.

Applying `paired-sign-test-v1` screens only t3code-server/single instructions and wall time: each has eight
positive pairs out of ten, exact p = 0.0546875. The one independent confirmation batch reverses the initial pair
order and yields instructions 38,160,143,868.5 → 38,159,962,929.5 (-0.000474%); wall 3.177846896 → 3.1720029795
seconds (-0.183895%); peak RSS 764,100,608 → 764,100,608 bytes. It has five positive instruction pairs and four
positive wall-time pairs. Pooled probabilities are 0.1315879822 for instructions and 0.2517223358 for wall time;
neither raw median increase repeats. The verdict is **no detected regression** across all eight cells. No
performance source changes were needed; this does not establish exact performance equality.

Artifacts are under `target/port-623/0002-697b847e11/`: full conformance lists, `pass-set-comparison.json`, generator
snapshots, gate logs, the Go reference binary, the frozen candidate `tsrs-release` and 113 bundled library files,
and `identity.json`. `perf-attempt-1.log` records the initial Python 3.9 setup error; the runner needs Python 3.12.
`perf-attempt-2/` contains the failed candidate comparison, and `perf-control-aa/` the failed identical-binary
control. `perf-remaining/results.json` retains all eight original cells and strict flags; `perf-summary.json`
records every paired delta. `perf-confirmation/` records the sole confirmation batch, and
`noise-policy-verdict.json` records the accepted verdict with SHA-256 references to both input datasets. The
accepted release binary SHA-256 is `83daa2fa410ca5fbb6bc02ff35ac70022d11841fbdf290bd737425ba19571134`; all 113
bundled library files remain byte-identical to the preceding accepted snapshot.

## 3. Named expression declaration names — passed

Upstream: `81d3f3f5e89786acc33d1881a0864bb0bd68068b` — Fix flaky diagnostic added by declaration emit through
`MarkLinkedReferencesRecursively` (#64482).

`is_in_expression_context` now treats the name of a function or class expression as a declaration name, matching
the new Go switch arm. Before the port, `noFlakyDiagnosticsExpressionNameShadowedInBody1` gained a spurious
post-emit TS2448 diagnostic for the function name, and its `.types` baseline incorrectly resolved that name to the
shadowing body's `number` variable. After the port, its diagnostics, types, symbols and JavaScript all pass. The
same case's emitted files match a Go compiler built at this source revision. The ignored reference checkout
advances to this revision; the published TypeScript pin stays unchanged.

Both checker histories retain every previous passing baseline and add the new case: 13,460 diagnostics and
12,781 types/symbols in Go history; 13,460 diagnostics and 12,780 types/symbols in canonical history. Full emit
passes 13,464 diagnostics, 12,781 types/symbols, 13,394 JS/declaration outputs, 149 JS maps and 156 source maps.
All twelve complete passing sets retain every preceding pass, with no crashes or timeouts. The four existing
checker diagnostic differences and canonical `objectLiteralNormalization` difference remain unchanged.

Workspace checks pass (1,064 passed, four existing ignored), as do native/WASM checks, Clippy, source checks,
formatting and all 36 regressions. Regeneration leaves all 1,167 crate files byte-identical to their pre-generation
snapshot. Because editor type and symbol queries share the changed predicate, the full Go fourslash gate also
runs at this source revision: 4,074 exercised groups pass, 472 are skipped, and none fail. An isolated metadata
wrapper selects the current source without changing the published pin. No additional source fixes were required.

Performance compares the frozen preceding release with this candidate across all eight project/mode cells under
`paired-sign-test-v1`. The reusable `target/port-623/collect.py` collects every initial cell without the superseded
raw-median stop; `noise_gate.py` evaluates those measurements and collects the prescribed confirmation batches.
One warmup and ten alternating pairs per cell yield these initial medians:

| Project / mode | Instructions, billions before → after | Wall seconds before → after | Peak RSS MiB before → after |
| --- | ---: | ---: | ---: |
| vscode / single | 79.775161 → 79.759270 | 7.513546 → 7.511081 | 1607.609 → 1607.609 |
| vscode / default | 91.417699 → 91.442431 | 1.039140 → 1.023495 | 2107.812 → 2109.258 |
| webpack / single | 10.570323 → 10.568779 | 0.902873 → 0.904547 | 311.328 → 311.328 |
| webpack / default | 14.627140 → 14.504238 | 0.202013 → 0.204132 | 495.719 → 491.234 |
| Compiler-Unions / single | 3.212018 → 3.212066 | 0.260791 → 0.259606 | 59.625 → 59.625 |
| Compiler-Unions / default | 3.886428 → 3.883809 | 0.148370 → 0.147829 | 83.305 → 83.219 |
| t3code-server / single | 38.183003 → 38.181974 | 3.197595 → 3.206484 | 728.703 → 728.703 |
| t3code-server / default | 121.235186 → 121.856587 | 1.370333 → 1.379313 | 2083.805 → 2088.070 |

Only webpack/default wall time and t3code-server/single wall time screen for confirmation: each has eight positive
pairs out of ten, exact p = 0.0546875. Each receives exactly one independent ten-pair batch with reversed pair
order. webpack/default wall time changes from 0.2029849795 to 0.202675500 seconds (-0.152464%); its pooled p is
0.0576591492. t3code-server/single changes from 3.1910962295 to 3.209990125 seconds (+0.592082%), but only four
confirmation pairs increase, and pooled p is 0.2517223358. Neither meets the confirmed-regression rule. The verdict
is **no detected regression**, not proof of exact equality; no performance source changes were needed. All
measured runs retain identical diagnostics and exit codes. No build or correctness workload overlaps measurement.

Artifacts are under `target/port-623/0003-81d3f3f5e8/`: before/after reproduction, complete conformance lists,
`pass-set-comparison.json`, unchanged-difference lists, generator snapshots, gate logs, isolated fourslash
metadata/reference/JSON results, the Go oracle, and the frozen release and 113 bundled library files.
`perf-initial/` and `perf-confirmation/` retain every raw sample; `noise-policy-verdict.json` references both
datasets by SHA-256. `identity.json` records binary, asset and evidence hashes. The accepted release binary has
SHA-256 `994dfbdb156784716df7ed9cb8bf342769f91b38624afec1f701d9450234cb56`; all 113 bundled library files remain
byte-identical to the preceding accepted snapshot.

## 4. Cyclic structures and declaration truncation — passed

Upstream: `834e7862c031a3f5f24c53acf51056a4f558c3e4` — Report cyclic structures and truncation during declaration
emit (#64461).

The node builder routes anonymous and union cycle guards through `create_cyclic_structure_placeholder`, which
reports the cyclic-structure error unless anonymous identifiers are allowed. Exceeding the symbol nesting limit
also sets `truncating`, so declaration emit reports the existing serialization-length diagnostic. This follows
the Go helper, call sites and control flow. The new `declarationEmitUnserializableTypes` case previously omitted
four TS5088 diagnostics and one TS7056 diagnostic; it now matches the reference, including the explicitly
annotated recursive declarations. Across all 13 changed or new fixtures, the pre-port run had 11 diagnostic and
JavaScript/declaration failures and three type failures; every diagnostic, type, symbol and emit baseline passes
after the port. All 13 cases also match the emitted files of a Go compiler built at this revision (one produces
no files). The additional filesystem gate repairs and performance repair are described below.

Both checker histories retain every previous passing baseline and add the new case: 13,461 diagnostics and
12,782 types/symbols in Go history; 13,461 diagnostics and 12,781 types/symbols in canonical history. Full emit
passes 13,465 diagnostics, 12,782 types/symbols, 13,395 JS/declaration outputs, 149 JS maps and 156 source maps.
All twelve complete passing sets retain every preceding pass, with no crashes or timeouts. The four existing
checker diagnostic differences and canonical `objectLiteralNormalization` difference remain unchanged. No
upstream cases were removed or renamed.

Workspace checks pass (1,066 passed, four existing ignored), as do native/WASM checks, Clippy, source checks,
formatting and all 36 regressions. Regeneration leaves all 1,168 crate files byte-identical to their pre-generation
snapshot. The full Go fourslash gate at this revision passes the same 4,074 exercised groups, with 472 skipped
and none failing; its complete passing group set is unchanged. An isolated metadata wrapper selects this source
without changing the published pin.

Two unrelated filesystem failures surfaced during revalidation and were repaired in this corresponding commit.
The FSEvents burst fixture required all 20 files in one callback, but received valid batches of one and 19 files:
its five-millisecond sleeps cannot guarantee the OS native batch shape. The real-filesystem fixture now waits
for the exact delivered path set. An isolated controlled callback case verifies that a single complete file
is immediate while one native multi-file batch holds the leading edge and delivers all files together.

A later parallel workspace run panicked in `ReadDir` cleanup with EBADF. Inspection found that kqueue cached raw
descriptors in subscriptions after an entry had been closed or replaced. A deterministic isolated regression
proved that both watch close and backend shutdown closed an unrelated file that reused the old descriptor,
leaked the replacement descriptor, and leaked one descriptor when overlapping watches were closed. The fix
removes redundant descriptor ownership from subscriptions: shutdown closes the current descriptor registry,
and watch close releases an entries map's current descriptors when its last subscription leaves. The regression
now passes all close, shutdown and overlap cases. The stale-descriptor bug also exists in the reference Go
algorithm; this is an explicit Rust I/O ownership repair. It is consistent with the observed parallel EBADF,
whose precise scheduling race was not reconstructed. Native filesystem and CLI watch integration gates pass.

The initial release failed the agreed performance gate on Compiler-Unions/single peak RSS. Its first ten-pair
median increased from 62,537,728 to 62,554,112 bytes (+16 KiB, +0.0262%); the independent reversed-order
confirmation increased from 62,521,344 to 62,545,920 bytes (+24 KiB, +0.0393%). Across both batches, 16 pairs
increase and four tie, with no decreases: pooled sign-test p = 0.0000152588. Compiler-Unions/default instructions
also screened initially but did not confirm (pooled p = 0.0576591492). The failed candidate and every raw sample
remain at their original paths. Initial medians for that candidate:

| Project / mode | Instructions, billions before → after | Wall seconds before → after | Peak RSS MiB before → after |
| --- | ---: | ---: | ---: |
| vscode / single | 79.531368 → 79.519333 | 7.362400 → 7.353514 | 1607.609 → 1607.609 |
| vscode / default | 91.335771 → 91.360507 | 0.997510 → 1.000161 | 2122.750 → 2116.070 |
| webpack / single | 10.566985 → 10.566332 | 0.895866 → 0.915597 | 311.328 → 311.328 |
| webpack / default | 14.617355 → 14.602850 | 0.205301 → 0.204599 | 496.203 → 496.938 |
| Compiler-Unions / single | 3.211780 → 3.211759 | 0.265617 → 0.263570 | 59.641 → 59.656 |
| Compiler-Unions / default | 3.882342 → 3.884291 | 0.148847 → 0.150356 | 83.258 → 83.242 |
| t3code-server / single | 38.155625 → 38.158179 | 3.122199 → 3.153594 | 728.703 → 728.703 |
| t3code-server / default | 121.885298 → 121.808789 | 1.346027 → 1.339640 | 2091.891 → 2090.469 |

A temporary entry probe showed neither changed branch executes in this workload. The release's data-section
sizes are unchanged; its text grows by 128 bytes, and page reclaims and physical footprint usually rise with RSS.
These observations suggest code generation affecting stack or other dirty-page residency; they do not establish
the exact mechanism. The existing rare-path and layout notes were reviewed before experimenting. Marking the new
helper `#[cold]` did not repair the focused RSS increase (+32 KiB), and plain `#[inline]` produced identical
executable sections, so both were rejected. Each measured experimental candidate received one focused batch; failed candidates were retained.

The repair keeps only `#[inline(never)]` on `create_cyclic_structure_placeholder`, preserving the Go helper and
control flow while preventing its recovery body from being inlined into callers. A focused ten-pair comparison
then gives identical peak RSS medians of 62,521,344 bytes; instructions change by +0.00512% and wall time by
+2.00303%, so this focused experiment is not an acceptance verdict. No cache, thread, allocator or algorithm
change is involved. Both checker histories, full emit, all 36 regressions and full fourslash were rerun for the repaired compiler.
The final filesystem repair changes no compiler inputs; its workspace, native/WASM, lint and generator gates
are rerun before full measurement.

The final release is compared with the same frozen source-3 baseline across all eight project/mode cells under
`paired-sign-test-v1`, with one warmup and ten alternating pairs per cell. Final initial-batch medians:

| Project / mode | Instructions, billions before → after | Wall seconds before → after | Peak RSS MiB before → after |
| --- | ---: | ---: | ---: |
| vscode / single | 79.500755 → 79.505384 | 7.289498 → 7.282193 | 1607.609 → 1607.578 |
| vscode / default | 91.285955 → 91.347801 | 0.984374 → 1.001522 | 2115.078 → 2111.188 |
| webpack / single | 10.562779 → 10.562005 | 0.906454 → 0.904980 | 311.328 → 311.305 |
| webpack / default | 14.650723 → 14.651033 | 0.205286 → 0.203064 | 497.578 → 498.086 |
| Compiler-Unions / single | 3.211742 → 3.211923 | 0.264454 → 0.266140 | 59.625 → 59.594 |
| Compiler-Unions / default | 3.884599 → 3.884684 | 0.150327 → 0.149670 | 83.133 → 82.984 |
| t3code-server / single | 38.134394 → 38.135493 | 3.110807 → 3.133931 | 728.688 → 728.672 |
| t3code-server / default | 122.154342 → 122.007797 | 1.372061 → 1.343244 | 2093.641 → 2091.094 |

Compiler-Unions/single instructions screen at +0.00564%; the sole confirmation gives +0.00272%, with five
positive and five negative pairs. Its pooled p = 0.0206947327 exceeds the 0.01 confirmation threshold.
t3code-server/single wall time screens at +0.74332%; its sole confirmation decreases by 0.37563%, with pooled
p = 0.4119014740. No RSS metric screens. The final verdict is **no detected regression**, not exact equality.
Every run preserves the baseline's output and exit status. No unchanged candidate was retried for a favorable
result; each screened cell received exactly the one prescribed confirmation.

The accepted frozen binary is `target/port-623/0004-834e7862c0/candidate-5/tsrs-release`, SHA-256
`36e55d4877ca18488bfddcecc7bf622278a1d843004e0ac3c78651a0f79a1af8`; all 113 runtime library files remain
byte-identical to the preceding snapshot. Artifacts under `target/port-623/0004-834e7862c0/` retain the upstream
patch, before/after reproductions, all complete passing sets, rejected immutable candidates and raw measurements,
controlled descriptor-reuse failure and repair, and final source/input identities. `candidate-4/` contains the
repaired compiler's full conformance, emit, oracle and fourslash evidence; `candidate-5/` records the final
filesystem, workspace, native/WASM, lint, formatting, generator and regression gates plus unchanged compiler
input evidence. Final performance data are in `perf-final/` and `perf-final-confirmation/`, with the acceptance
verdict in `noise-policy-final-verdict.json`. The published TypeScript pin remains unchanged.

## 5. Static properties and self-contextual typing — passed

Upstream: `619d485a639c83a2d82860b5797db4984cecfc91` — Avoid contextually typing static properties by their own
class to prevent spurious circularities (#64525).

`get_contextual_type_for_static_property_declaration` now rejects a contextual parent type whose symbol is the
class's own declaration symbol, preserving the Go short-circuit order. Before the port, the new
`classStaticPropertyNoSpuriousCircularityError1` fixture reports four spurious TS7022 diagnostics and infers `any`
instead of `42`, `number`, `number[]` and `{}`. All its diagnostics, types, symbols and emit baselines now pass.
A Go compiler built at this exact source revision produces byte-identical diagnostics and the same exit code;
the seven genuine circular-reference diagnostics remain. The ignored reference checkout advances; the published
TypeScript pin stays unchanged. The additional performance repair is described below.

Both checker histories retain every preceding passing baseline: 13,462 diagnostics and 12,783 types/symbols in
Go history; 13,462 diagnostics and 12,782 types/symbols in canonical history. Full emit passes 13,466 diagnostics,
12,783 types/symbols, 13,396 JS/declaration outputs, 149 JS maps and 156 source maps. All twelve complete passing
sets retain every preceding pass, with no crashes or timeouts. The four existing checker diagnostic artifacts and
canonical `objectLiteralNormalization` type/symbol difference remain unchanged. No upstream cases were removed.

The final candidate passes the full workspace gate (1,066 passed, four existing ignored), native/WASM checks,
Clippy, source checks, formatting and all 36 regressions. Regeneration leaves all 1,168 crate files byte-identical.
Full Go fourslash passes the same 4,074 exercised groups, with 472 skipped; its complete passing set is unchanged.
The initial sandboxed bridge check could not bind its Unix socket; its failure log is retained, and the gate passes
with authorized local-socket access. The isolated metadata wrapper selects this source without changing the pin.

The first release confirms single-threaded RSS regressions under `paired-sign-test-v1`: VS Code rises by 16 KiB
in both batches (pooled p = 0.0000381470); Compiler-Unions rises by 24 KiB initially and 16 KiB in confirmation
(p = 0.0000076294); t3code-server rises by 16 KiB in both batches (p = 0.0020904541). webpack/default wall time
and t3code-server/default RSS screen initially but do not confirm. Every failed raw sample and the original
immutable release remain at their original paths.

After reviewing the existing performance notes and source-4 diagnosis, temporary entry probes find that
Compiler-Unions never calls this helper, t3code-server calls it seven times without reaching the new guard, and
VS Code calls it 331 times and reaches the guard 12 times. The added guard cannot explain the Compiler-Unions
increase through executed work. The release's text grows by 64 bytes, with unchanged segment and data-section
sizes. Mach-O function boundaries identify two bodies growing by 48 and 16 bytes; their stack prologues remain
112 and 80 bytes. Symbols are stripped, so their identities are not established. This does **not** demonstrate
larger stack frames; code generation affecting page residency remains a hypothesis, not an established mechanism.

The repair adds only `#[inline(never)]` to this existing static-property helper, preserving the Go function and
control flow and the preceding cyclic-recovery helper's attribute. It adds no cache, thread or allocator change.
This narrow experiment responds to the newly confirmed port regression; it does not repeat the broad contextual
member outlining rejected in earlier notes. A focused ten-pair Compiler-Unions comparison changes peak RSS from
62,488,576 to 62,472,192 bytes (-16 KiB), instructions by -0.003004%, and wall time by -0.709589%. That focused
batch is diagnostic evidence, not acceptance. The temporary probes are removed, and all correctness gates above
are rerun on the repaired candidate.

The repaired release is compared against source 4's accepted `candidate-5/tsrs-release`, with one warmup and ten
alternating pairs in all eight configurations. No builds or correctness workloads overlap measurement. Final
initial-batch medians:

| Project / mode | Instructions, billions before → after | Wall seconds before → after | Peak RSS MiB before → after |
| --- | ---: | ---: | ---: |
| vscode / single | 79.586344 → 79.590718 | 7.339378 → 7.313885 | 1607.578 → 1607.562 |
| vscode / default | 91.496305 → 91.472670 | 0.992566 → 0.989575 | 2107.539 → 2108.641 |
| webpack / single | 10.563613 → 10.561301 | 0.899224 → 0.904487 | 311.297 → 311.266 |
| webpack / default | 14.541304 → 14.593499 | 0.203591 → 0.203820 | 494.219 → 495.250 |
| Compiler-Unions / single | 3.211762 → 3.211699 | 0.264822 → 0.268189 | 59.594 → 59.578 |
| Compiler-Unions / default | 3.884001 → 3.886338 | 0.149279 → 0.149961 | 83.281 → 83.062 |
| t3code-server / single | 38.142110 → 38.142063 | 3.150723 → 3.115938 | 728.672 → 728.641 |
| t3code-server / default | 121.745705 → 122.217984 | 1.357603 → 1.362288 | 2093.336 → 2097.469 |

Only Compiler-Unions/single wall time screens (+1.271514%, eight positive pairs of ten). Its sole independent
reversed-order confirmation increases by 1.666293%, with six positive and four negative pairs; pooled
p = 0.0576591492 exceeds the 0.01 threshold. No RSS or instruction metric screens. The final verdict is **no
detected regression**, not exact equality. Every measured run preserves the predecessor's output and exit
status. No unchanged candidate was retried for a favorable result.

Artifacts are under `target/port-623/0005-619d485a63/`: the upstream patch, before/after reproduction and Go oracle,
initial complete conformance sets, rejected release and raw `perf-initial/` / `perf-confirmation/` datasets,
`noise-policy-verdict.json`, and causal probes/disassembly under `diagnosis/`. `candidate-2/` contains the final
full correctness evidence, complete pass-set comparisons, unchanged source-input snapshots, frozen release and
113 byte-identical runtime library files. Final performance samples are in `perf-final/` and
`perf-final-confirmation/`, referenced by SHA-256 in `noise-policy-final-verdict.json`. The accepted release is
`candidate-2/tsrs-release`, SHA-256 `9c6bba2d437088e45963d774a9f6e2fa7d54614df33b3d49345ab3860634a46a`.
