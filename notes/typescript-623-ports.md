# TypeScript ports for issue 623

Port [issue 623](https://github.com/oxc-project/tsrs/issues/623) in upstream chronological order, one tsrs commit per
upstream commit, including evidence-only commits for changes that need no Rust code. Each corresponding commit has
an `Upstream-Commit` trailer. Assign each upstream commit to a dedicated subagent, and finish its validation and
corresponding commit before starting the next subagent. Start at `b85298b6a81f772d080b0455de0ca9d744cd6fd6`; finish at
`5237fd46890956e4bb8177a1a05dcd69e4166560` (112 commits). Keep the published TypeScript pin until the final entry.

Each entry must retain every passing checker baseline in both histories and pass the emit baselines, workspace
checks, Clippy, source checks, and generated-code verification. Run the affected subsystem's additional gates.
Entries 1–10 used paired performance comparisons on `vscode`, `webpack`, `Compiler-Unions`, and `t3code-server`.
The initial entry 11 investigation expanded the planned cohort to the six TypeScript benchmark projects plus
`t3code-server`, in single-threaded and default modes. The user subsequently instructed us to continue the
migration without a performance-regression gate. From entry 11 onward, correctness remains mandatory; disclose
instruction, wall-time, and peak-RSS measurements when available, including confirmed regressions, but do not
withhold a correctness-valid corresponding commit for a performance result or an unmeasured cohort cell. Keep
measurement artifacts. For no-op ports, byte-identical compiler binaries and runtime assets establish that their
performance is unchanged. Reuse completed correctness checks only when their inputs are demonstrably unchanged;
compare complete passing sets rather than totals alone.

After the identical-binary control in entry 2 also failed the original zero-tolerance median rule, the user
instructed removal of that blocking rule and continuation without human guidance. Routine migration work requires
no further human approval. The historical replacement measurement policy, also used for focused entry 11
diagnoses, was `paired-sign-test-v1`; it is no longer a landing gate from entry 11:

- For a measured cohort, retain all three raw before/after medians and every pair in each measured cell (eight
  in the original cohort; fourteen in the extended cohort).
- For each metric, compute paired relative changes and the exact one-sided sign-test probability
  `P[Binomial(n_non_tied, 0.5) >= positive_pairs]`, excluding exactly equal pairs.
- A positive raw median change screens for confirmation when that probability is at most 0.10.
- Run exactly one independent ten-pair confirmation batch per screened cell against the same preceding binary,
  reversing the initial pair order. Evaluate every screened metric in that cell.
- A reproducible regression has positive raw median changes in both batches and a pooled paired sign-test
  probability at most 0.01. Retain and disclose that evidence. Do not repeat unchanged candidates until a
  favorable batch appears.
- Otherwise report **no detected regression**, not proof of exact equality. Keep old strict-rule records unchanged;
  new verdicts identify their input files by SHA-256. Binary/runtime identity still suffices for a no-op port.

The reusable local evaluator and confirmation collector are `target/port-623/noise_gate.py`; its local manifest
records the earlier measurement rules. Evaluator sanity checks cover identical samples, consistent
increases, exact binomial tails, excluded ties and mixed samples. The explicit cohort
`typescript-six-plus-t3-v1` requires all fourteen cells; legacy reports retain their eight-cell requirement.
The cohort extension preserves the equations and thresholds and reproduces the earlier accepted and rejected
verdicts unchanged. Its evaluator rejects missing, duplicate, unknown and mismatched cells when that cohort is
requested; running it is not required to land entries 11 onward.

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


## 6. String literal module import attribute types — passed with build-identity adjudication

Upstream: `792ffccb90f548cd3bdd7fc0b44f0ca9e1b169a7` — fix: disallow NoSubstitutionTemplate in module import
attribute types (#64243).

`check_grammar_import_attributes_type` now requires a literal type node containing a string literal, preserving
the Go short-circuit order before accessing the literal. The updated `importAttributeType1` fixture fails before
the port because the no-substitution template `css` lacks TS1555; its interpolated template already errors.
After the port, all 33 import-attribute variants pass, including 28 applicable type/symbol and 32 emit baselines.
A Go compiler built at this exact source revision produces byte-identical diagnostics and the same exit status
for the materialized upstream case. The ignored reference checkout advances; the published TypeScript pin stays
unchanged. The measurement problem and its controlled resolution are described below.

All twelve complete passing sets exactly match source 5's accepted candidate: 13,462 diagnostics and 12,783
types/symbols in Go checker history; 13,462 diagnostics and 12,782 types/symbols in canonical history; full emit
passes 13,466 diagnostics, 12,783 types/symbols, 13,396 JS/declaration outputs, 149 JS maps and 156 source maps.
There are no crashes or timeouts. The four existing checker diagnostic artifacts and canonical
`objectLiteralNormalization` type/symbol difference remain unchanged.

The workspace passes 1,066 checks with four existing ignored. Native/WASM checks, Clippy, source checks,
formatting and all 36 regressions pass. All seven generators plus formatting leave all 1,168 crate files
byte-identical. Full Go fourslash retains every one of the preceding 4,074 exercised groups and 4,130 passing
events, with 472 groups skipped; its six Python gate checks and local-socket bridge checks pass. The isolated
metadata wrapper selects this source without changing the published pin. The initial Go oracle build could not
access the default Go cache in the sandbox; its retained failure log precedes the successful build using a cache
under `target/`.

Performance compares the frozen preceding `candidate-2/tsrs-release` with this release on all four public
projects, single-threaded and at the default checker count, under `paired-sign-test-v1`. No build or correctness
workload overlaps measurement. One warmup and ten alternating pairs per cell give these initial medians:

| Project / mode | Instructions, billions before → after | Wall seconds before → after | Peak RSS MiB before → after |
| --- | ---: | ---: | ---: |
| vscode / single | 79.709651 → 79.714917 | 7.367043 → 7.384303 | 1607.562 → 1607.594 |
| vscode / default | 91.435059 → 91.530841 | 1.064486 → 1.085740 | 2112.938 → 2109.008 |
| webpack / single | 10.563169 → 10.561039 | 0.905310 → 0.864459 | 311.266 → 311.312 |
| webpack / default | 14.576860 → 14.643088 | 0.207951 → 0.209863 | 494.555 → 497.656 |
| Compiler-Unions / single | 3.211774 → 3.211768 | 0.266543 → 0.268167 | 59.578 → 59.594 |
| Compiler-Unions / default | 3.884214 → 3.883863 | 0.149601 → 0.144370 | 83.266 → 83.070 |
| t3code-server / single | 38.149629 → 38.150524 | 3.126021 → 3.154399 | 728.641 → 728.672 |
| t3code-server / default | 122.045047 → 122.229939 | 1.315348 → 1.335166 | 2102.773 → 2099.227 |

The ordinary fixed-build verdict **confirms four single-thread RSS increases**: VS Code +32 KiB in both
batches (pooled p = 0.0000009537), webpack +48 KiB initially and +40 KiB in confirmation (p = 0.0000009537),
Compiler-Unions +16 KiB in both (p = 0.0000305176), and t3code-server +32 KiB in both (p = 0.0000019073).
Other screened instruction/wall metrics do not confirm. This verdict remains unchanged at
`noise-policy-verdict.json`; it is not relabeled passed. Every original and rejected candidate and raw sample
remains at its original path.

After reviewing earlier rejected work, temporary entry probes observe zero calls to this grammar helper in
VS Code, Compiler-Unions and t3code-server. Mach-O inspection finds unchanged text-section size and unchanged
192-byte parent stack frame. Three focused experiments fail: outlining the grammar helper gives +16 KiB
Compiler-Unions RSS; adding cold placement gives +64 KiB; outlining the parent reduces its frame to 160 bytes
but gives +24 KiB. All attributes are reverted. These observations do not establish a code-layout mechanism.
The source-4 cyclic-recovery and source-5 static-context helper attributes remain unchanged.

Equal-byte copies at the original short and long path lengths do not reproduce the RSS increase. Debugger and
external vmmap attempts yield no usable snapshot and their failure logs remain. A diagnostic injected
self-inspection library instead localizes a repeatable startup difference to system `MALLOC_SMALL` residency.
The equal-length-path controls have equal live allocation counts and bytes; the original short/long paths
differ by 48 live bytes. The residency difference also occurs with `--version`, before checker work. Injection changes the environment and exit residency need not equal peak residency, so these probes are
not acceptance measurements.

Strict reciprocal metadata controls then change only the 16-byte Mach-O UUID and its 32-byte signature hash
covering the header. All executable/data sections, remaining bytes, signature identifier/flags/hash-page format
and 113 runtime assets stay unchanged; all copies pass strict signature verification and use equal-length paths.
Normal non-injected measurements reproduce a metadata-only +16-KiB startup increase (ten positive pairs) and
+24-KiB Compiler-Unions median increase (eight positive, two tied pairs) for baseline code. Initial-code copies
also differ by +16 KiB at startup (ten positive pairs), though their Compiler-Unions medians tie (six positive,
four tied pairs). Every output and exit matches. `diagnosis/strict-control-summary.json` retains exact byte
locations, complete section hashes, signatures and all pairs. These controls demonstrate a benchmark nuisance;
they do not independently exclude a code memory effect.

The measurement problem is addressed with root-reviewed, preregistered **build-identity sampling**, described in
[the reusable evidence note](perf-build-identity.md). Apple's revision-pinned primary allocator documentation
supports entropy stable across executions of a binary within one boot, so repeated launches of one build do not
independently sample that nuisance. The complete installed allocator mechanism remains unproven. Before any new
measurement, `diagnosis/build-identity-protocol.json` freezes ten SHA-256-derived UUID identities, each applied
symmetrically to both original code variants, plus ten disjoint reserved confirmation identities. Every identity
counts; none is selected for favorable results. Each of the four previously confirmed single-thread cells gets
one warmup and one timed run per binary/identity in alternating order, with no injection or allocator override.
All three ordinary metrics are retained and screened using the existing exact sign-test thresholds. Only a
screened cell receives the single reserved confirmation, reversing pair order. No forced UUID or production
workaround ships, and the original frozen release remains the candidate. Controlled initial medians:

| Project / mode | Instructions, billions before → after | Wall seconds before → after | Peak RSS MiB before → after |
| --- | ---: | ---: | ---: |
| vscode / single | 79.487772 → 79.484529 | 7.269334 → 7.280619 | 1607.594 → 1607.594 |
| webpack / single | 10.562651 → 10.562622 | 0.887391 → 0.860729 | 311.305 → 311.297 |
| Compiler-Unions / single | 3.211773 → 3.211704 | 0.264461 → 0.269008 | 59.609 → 59.609 |
| t3code-server / single | 38.134651 → 38.134738 | 3.129247 → 3.106347 | 728.672 → 728.672 |

No controlled RSS or instruction metric screens. Compiler-Unions wall time screens at +1.719593%
(0.264461 → 0.269008 seconds, eight positive pairs, p = 0.0546875). Its sole confirmation decreases by
0.213206% (0.269771 → 0.269196 seconds), with five positive and five negative pairs; pooled p = 0.1315879822.
The controlled verdict is **no detected code-associated regression across the preregistered identities**,
not proof of equality. The original shipping binaries still measure +16–48 KiB RSS in the fixed-build initial
batch. `diagnosis/build-identity-final-verdict.json` links the frozen protocol and both complete datasets.
Its evaluator verifies exact coverage, hashes, UUIDs, run order, unchanged binaries/assets and output parity.
This source-6 adjudication does not automatically change later sources' policy.

After removing the experiments, all 1,608 original correctness inputs and all 1,640 broader build inputs match
the completed gate snapshots byte-for-byte, so those gates are reused with recorded proof. The dev CLI is rebuilt
to remove the temporary probe and again matches the exact-source Go diagnostic oracle. Final source lint,
formatting and diff checks pass. The frozen original release is `tsrs-release`, SHA-256
`9b0a663ce1fe83506e50952d0ec4827d33c6fd658a9a5f7d22985dc983864d7d`; its 113 runtime assets match the
preceding accepted snapshot. `target/port-623/0006-792ffccb90/` contains the upstream patch, reproduction,
full correctness and pass-set evidence, unchanged-input proofs, original eight-cell performance rejection,
all failed candidates, diagnostic controls, preregistration/review and final controlled verdict.

## 7. `c4d731aae17e` — constrain recursive return inference

Port the exact upstream condition in `get_inferred_type`: pure return-type inference is filtered by its
constraint even during recursive call resolution. The exact priority equality, left-to-right evaluation and
existing filtering/fallback stay intact. The new `recursiveTypeInferenceReturnTypeConstraint` fixture agrees
byte-for-byte with a Go compiler built at this source: two TS2345 diagnostics. The preceding compiler instead
produces four diagnostics, including spurious constraint errors, and misses the recursive-call error.
The ignored reference checkout advances; the published TypeScript pin remains unchanged.

All twelve complete passing sets retain every preceding pass. The new fixture is the sole addition in ten
sets; maps are unchanged. Go checker passes 13,463 diagnostics and 12,784 types/symbols; canonical checker
passes 13,463 diagnostics and 12,783 types/symbols. Full emit passes 13,467 diagnostics, 12,784 types/symbols,
13,397 JS/declaration outputs, 149 JS maps and 156 source maps. All 66 nonpassing lists/artifacts are
byte-identical, including the four existing checker diagnostic differences and canonical
`objectLiteralNormalization` type/symbol difference. There are no crashes or timeouts.

The final formatted candidate passes the isolated workspace (1,066 passed, four ignored), all 36 regressions,
native/WASM checks, Clippy, source lint and formatting. All seven generators plus formatting preserve all
1,168 crate files. Full Go fourslash retains all 4,074 exercised passing groups and 4,130 passing events,
with 472 groups skipped; its gate and socket-bridge checks pass. Its isolated metadata wrapper selects the
exact source without modifying the published pin. All 1,608 correctness and 1,640 broader build inputs remain
byte-identical through the final gates. Root independently verified the complete passing sets, artifacts,
input hashes and performance verdict.

The first workspace run overlapped emit and Go compilation; the existing FSEvents burst-under-load fixture
reported one dropped batch. Its failed log is retained. One isolated full-workspace run passed, as did the
final candidate's isolated run. The workflow repair is to run the entire workspace without unrelated heavy
work; no filesystem-watcher code changes. Initial evidence setup failures are also retained: the Go build
command was corrected from nonexistent `./cmd/tsgo` to `./cmd/tsc`, and snapshot handling now hashes symlink
target text instead of reading a directory symlink as a file.

The initial mechanical port **confirms a t3code-server single-thread wall regression** under
`paired-sign-test-v1`: 3.107099 → 3.159965 seconds (+1.701464%, ten positive pairs) initially, and
3.132455 → 3.146151 seconds (+0.437227%, six positive/four negative pairs) in its sole reversed-order
confirmation; pooled p = 0.0059089661. Instructions decrease slightly and median RSS ties in both batches.
Other screened metrics do not confirm. The original binary, complete eight-cell run, confirmation and
`noise-policy-verdict.json` retain their failed status.

After reading prior rejected experiments, diagnostic probes find only two `NoConstraintChecks` candidates
in t3code-server, neither pure `ReturnType`: the new condition enables zero extra constraint checks there.
Existing raw counters show cycles increasing in all 20 pairs, with median increases of 1.42967% and
0.59744%; CPU time increases while page-reclaim medians tie and involuntary switches decrease. Static codegen
shows the ordinary constraint branch changing from fallthrough to taken, jumping over the new priority test.
The helper grows by 12 bytes, with its 368-byte stack frame unchanged. This supports a branch/layout diagnosis;
it does not establish every microarchitectural cause. All probes are removed.

The targeted repair places `std::hint::cold_path()` on the rare right-hand side, before the exact priority
comparison. This preserves upstream control flow and restores ordinary-path fallthrough at the preceding
binary's addresses, with no stack-frame growth or new spills. The helper gains four rare-path instructions;
no new outlining attribute is added. One focused ten-pair diagnostic on a fresh candidate yields wall
3.115891 → 3.107622 seconds (-0.265367%), instructions 38.135097 → 38.130792 billion, RSS
728.672 → 728.656 MiB and cycles 11.598189 → 11.570484 billion (-0.238872%). It is diagnostic evidence,
not the final acceptance run.

That candidate's prebuild formatting check failed but an incorrectly sequenced shell command continued the
build; generation subsequently exposed the formatting change. Both failures and the superseded validation
attempts remain. The orchestration repair applies formatting and all generators before snapshots/builds,
checks every prerequisite exit, and runs final gates serially without source writes during Cargo reads.
The formatted final release is frozen at a new immutable path of the same length as the baseline path.
Its executable sections and all 17,781 functions match the preceding repair candidate exactly; only UUID,
eight source-location constants and signature hashes differ (183 bytes total). All correctness and final
performance evidence below belongs to these formatted inputs. No forced UUID or metadata override ships.

Final performance uses the accepted source-6 release and final source-7 release on all four installed public
projects, single-threaded and at the default checker count, with `noEmit`, incremental disabled and plain
output. No heavy workload overlaps timing. One warmup and ten alternating pairs per cell produce:

| Project / mode | Instructions, billions before → after | Wall seconds before → after | Peak RSS MiB before → after |
| --- | ---: | ---: | ---: |
| vscode / single | 79.550229 → 79.532649 | 7.369314 → 7.328296 | 1607.594 → 1607.578 |
| vscode / default | 91.297143 → 91.326799 | 1.003273 → 0.997858 | 2116.938 → 2110.109 |
| webpack / single | 10.569473 → 10.567291 | 0.907911 → 0.902916 | 311.312 → 311.297 |
| webpack / default | 14.515828 → 14.511265 | 0.202901 → 0.202121 | 493.711 → 491.625 |
| Compiler-Unions / single | 3.211806 → 3.211823 | 0.262375 → 0.265229 | 59.609 → 59.602 |
| Compiler-Unions / default | 3.882497 → 3.884607 | 0.149139 → 0.150228 | 82.727 → 82.953 |
| t3code-server / single | 38.138322 → 38.134795 | 3.134343 → 3.126166 | 728.672 → 728.656 |
| t3code-server / default | 121.490636 → 122.249147 | 1.323314 → 1.310708 | 2087.469 → 2098.672 |

The final verdict is **no detected regression**. No metric screens, so no confirmation is required or run.
This is not proof of equality: some medians rise, but their paired signs do not satisfy the preregistered
screen. The previously regressing t3code-server single-thread wall median decreases by 0.260882%.
Every output and exit matches the baseline. All original failures and rejected/superseded candidates remain;
there is no unchanged-candidate retry or build-identity adjudication for this source.

Final correctness lives in `target/port-623/0007-c4d731aae1/candidate-3/`; raw performance is in
`perf-final/`, with final decision `noise-policy-final-verdict.json` under the source artifact root.
That root also holds the source patch, exact-source oracle, immutable snapshots, all original failures,
codegen/probe/cycle diagnosis and focused measurements. The accepted frozen binary is
`target/port-623/0007-cand000003/tsrs-release`, SHA-256
`7d5ca9c24e62ce861072fce926c6d93fb33b6ee5e51a8c177a035ac26e2b17f5`; all 113 adjacent runtime
assets match source 6. The original rejected binary remains `0007-c4d731aae1/tsrs-release`, SHA-256
`7c52a06fe8c3fec705a3a73e0fbf31c3fcc5078f84452a8a851a6254a8c5e97d`.

## 8. `79feed291a5c` — disable upstream VSIX template publishing

Upstream `79feed291a5c6c1d42ec040c4b42e7c2e9a1e2c4` adds `mb.publish.enabled: false` to the
Validate and Publish jobs in `tools/pipelines/vscode-typescript-publish.yml`. tsrs has no corresponding
Azure/VSIX pipeline, so this is an evidence-only port. No capability or runtime behavior changes.
The ignored reference checkout advances to this source and is clean; the published pin stays unchanged.

All 66,843 other upstream entries are identical, including the complete `tsc` and `packages` trees,
fixtures, baselines, generator schemas and bundled libraries. All 1,608 correctness inputs, 1,640 broader
build inputs and 1,168 generated crate files match source 7's final snapshots. A complete local tracked-file
comparison also covers npm/API tooling and configuration outside those snapshots; only these two evidence
documents change. This permits inheritance of source 7's final compiler, workspace, native/WASM,
regression, generator and fourslash gates. All twelve complete passing sets and 66 nonpassing artifacts
are preserved byte-for-byte: Go checker 13,463 diagnostics / 12,784 types and symbols; canonical
13,463 / 12,783; emit 13,467 diagnostics / 12,784 types and symbols / 13,397 outputs / 149 JS maps /
156 source maps; workspace 1,066 passed and four ignored; 36 regressions; fourslash 4,074 passing groups,
472 skipped and 4,130 passing events. Fresh Clippy, source lint, formatting and diff checks pass.

Performance is unchanged by compiler/runtime identity, without another timing run. Both sides use
`target/port-623/0007-cand000003/tsrs-release`, SHA-256
`7d5ca9c24e62ce861072fce926c6d93fb33b6ee5e51a8c177a035ac26e2b17f5`, with all 113 adjacent
runtime assets unchanged. Source 7's instruction, wall-time and RSS measurements remain the applicable
record. The patch, input/tree identities, inherited result hashes and fresh lint logs are retained under
`target/port-623/0008-79feed291a/`; inherited gate logs are in `0007-c4d731aae1/candidate-3/`.

## 9. `64c5fa0b92ad` — exercise both custom export conditions

Upstream `64c5fa0b92ade9b5f14db616d351ffe1990ca7fd` removes the space in the fixture directive
`@customConditions: webpack,browser`, updates its exports-enabled type baseline from `webpack` to `browser`,
and updates two Go resolution-trace baselines. The existing Rust compiler already handles both conditions.
Both variants pass diagnostics, types, symbols and emit before and after the fixture change. An exact-source
Go build agrees on diagnostics and exit status; supplementary literal-assignment probes also agree byte-for-byte:
exports enabled selects `webpack` before and `browser` after, while exports disabled selects `index` in both.
Go traces independently confirm those module paths. Rust trace-output parity is not asserted by these gates.
The ignored reference checkout advances and is clean; the published TypeScript pin remains unchanged.

Fresh full compiler runs preserve all twelve complete passing sets exactly: Go checker 13,463 diagnostics /
12,784 types and symbols; canonical checker 13,463 / 12,783; emit 13,467 diagnostics / 12,784 types and symbols /
13,397 outputs / 149 JS maps / 156 source maps. All 66 known nonpassing lists/artifacts remain byte-identical,
including the four checker diagnostic differences and canonical `objectLiteralNormalization` difference.
There are no new failures, crashes or timeouts. Fresh Clippy, source lint, formatting and diff checks pass.

Only the four upstream fixture/baseline files change; all 66,840 other entries, including Go code, fourslash
fixtures/baselines, generator schemas and bundled libraries, are identical. All 1,608 local correctness inputs,
1,640 broader build inputs and 1,168 generated crate files match source 7's final snapshots; a complete comparison
of all 2,352 tracked paths changes only the compatibility page and this ledger. The changed conformance fixture
is covered by the fresh compiler runs and is not an input to the inherited workspace, native/WASM, generator,
regression or fourslash gates. Those retain source 7's 1,066 workspace passes with four ignored, all 36 regressions,
and 4,074 passing fourslash groups / 472 skipped / 4,130 passing events. An initial evidence setup command named
a nonexistent fourslash tree; the recorded failure was repaired to use the actual Go fixture and baseline trees.
No compiler repair was needed.

Performance is unchanged by compiler/runtime identity, without another timing run. Both sides use the same
accepted `target/port-623/0007-cand000003/tsrs-release`, SHA-256
`7d5ca9c24e62ce861072fce926c6d93fb33b6ee5e51a8c177a035ac26e2b17f5`, and all 113 adjacent
runtime assets remain byte-identical. Source 7's instruction, wall-time and RSS measurements remain the applicable
record. The source patch, before/after fixtures, exact-source oracle, fresh compiler results, all passing sets,
input/tree identities and inherited-gate hashes are under `target/port-623/0009-64c5fa0b92/`.

## 10. `5f5dee5b6105` — switch upstream signing to MSBuild

Upstream `5f5dee5b610526cf3c597457cca336fde65e5f11` replaces DDSignFiles with MSBuild/MicroBuild
in `Herebyfile.mjs`, updates two Azure signing pipelines and adds a signing project and NuGet configuration.
tsrs's Cargo/Node/pnpm release tooling has no corresponding Microsoft signing integration, so this is an
evidence-only port. No capability or runtime behavior changes and no compiler repair is needed. The ignored
reference checkout advances and is clean; the published TypeScript pin remains unchanged.

All 66,841 other upstream entries are identical, including complete compiler, fixture, baseline, package,
generator and library trees. All 1,608 correctness inputs, 1,640 broader build inputs and 1,168 generated
crate files match the accepted snapshots. Comparing all 2,352 local tracked paths changes only this ledger
and the compatibility page. Source 9's full compiler results and source 7's other gates are therefore inherited:
all twelve complete passing sets and 66 known nonpassing artifacts remain byte-identical. Go checker retains
13,463 diagnostics / 12,784 types and symbols; canonical 13,463 / 12,783; emit 13,467 diagnostics / 12,784
types and symbols / 13,397 outputs / 149 JS maps / 156 source maps. Workspace retains 1,066 passes and four
ignored; all 36 regressions and 4,074 fourslash groups / 472 skipped / 4,130 passing events remain applicable.
Fresh Clippy, source lint, formatting and diff checks pass.

Performance is unchanged by compiler/runtime identity, without another timing run. Both sides use
`target/port-623/0007-cand000003/tsrs-release`, SHA-256
`7d5ca9c24e62ce861072fce926c6d93fb33b6ee5e51a8c177a035ac26e2b17f5`, with all 113 adjacent
runtime assets unchanged. Source 7's instruction, wall-time and RSS measurements remain the applicable
record. The patch, scope review, input/tree identities, inherited result hashes and fresh checks are under
`target/port-623/0010-5f5dee5b61/`.

## 11. `c8e9b7d25926` — cache inference from type arguments

Upstream `c8e9b7d259268e30605ed14910cb8d118929ed63` puts same-alias and same-reference type-argument
inference under `invokeOnce` and extends the visited key with inference priority, contravariance and bivariance.
The Rust port mirrors those branches and their recursion state. Its owned, pooled visited map ends its borrow before
recursive inference and updates the original key afterward; the full key retains separate states that the old
two-TypeId key would conflate. Rust-specific repairs keep priority in a checked two-byte encoding, reuse borrowed
variance slices and candidate buffers, and avoid a nesting walk for stacks shorter than two. The added
`inference-type-argument-variance` regression checks the upstream correlated-union TS2345 diagnostic against Go,
the dev CLI and the frozen release CLI.

The exact candidate-20 compiler rerun retains all twelve complete passing sets and all 66 known nonpassing
artifacts from entry 10. Go-history checking passes 13,463 diagnostics and all 12,784 type/symbol baselines;
canonical history has the same diagnostic passes and 12,783 type/symbol passes, with its known differences
unchanged. Emit passes 13,467 diagnostics, 13,397 JavaScript outputs, 149 JS maps and 156 source maps. Workspace
tests pass 1,067 with four ignored, all 37 regressions pass, and fourslash has 4,074 passing groups, 472 skipped,
and 4,130 passing events. Twenty-seven memo/deep-shadow runs preserve output and exit status. Native and WASM
checks, warnings-denied Clippy, source lint, formatting, and generator verification pass; all 1,168 generated
crate files remain unchanged. The serial-workspace fixtures were repaired to parse libtest's same-line child PID
prefix and accept either root or child kqueue descriptor exhaustion, which depends on map order. The initial
sandboxed workspace and fourslash runs could not bind local Unix sockets; their preserved failures were followed
by passing runs with socket access. The full current-source evidence and input proofs are under
`target/port-623/0011-c8e9b7d259/candidate-20/`.

Performance is disclosed under the user's entry-11 nonblocking policy. Against entry 10's byte-identical
accepted compiler, the frozen candidate-20 release (`3edc24074ab2ce687196eb816f114115c6292f3d27bf06a5e918fdfaee07a1dd`)
has a confirmed `webpack` single-threaded peak-RSS increase of **90,112 bytes** in each ten-pair batch
(20/20 paired increases; `candidate-20/formal-webpack-rss/run/results.json`). `t3code-server` single-threaded
instructions retired increase **0.182604%** in the initial ten pairs and **0.178883%** in the reversed
confirmation (19/20 paired increases; `diagnosis/c20-formal-t3/run/results.json`). The full fourteen-cell
performance cohort was not an entry-11 acceptance requirement after the user's policy change. These measured
regressions are retained in the record; they do not change the correctness verdict or hold the corresponding port.

## 12. `43521c80ec5e` — completion symbols remain in their snapshot

Upstream `43521c80ec5ebe4bc2948f1df3747773e48d8312` adds `userPreferences` and `prepareAutoImports` to
`createSnapshot` and `updateSnapshot`. The Rust API now passes both into the same clone, verifies the requested
file's default project and prepared auto-import registry before registering the result, and dereferences an
invalid clone. The requested filename is also passed to the registry's ancestor-directory walk, so a fresh
snapshot can index dependencies below a mixed-case project directory even when the file is not open. Completion
uses the active source filename. If auto-imports are needed, `includeSymbol: true` now returns the upstream
client error for an unprepared snapshot; without symbols, the existing transient-clone retry still works.
This prevents returning symbol handles owned by another snapshot. Focused Rust tests cover those paths,
per-snapshot preferences, creation/update with synthetic programs and a dependency auto-import, and an
unprepared dependency auto-import retry without symbols.

The public sync/async/protocol types export the new preference interfaces and snapshot fields. The new
`userPreferences.generated.ts` is byte-identical to upstream (SHA-256
`07d16cbb3a300152a1533628529baa0df9861815f8d0b82943acfde087fd4e3c`). Rust's existing tag-table
decoder now accepts all 83 raw names in source 12's generated Go decoder and matches its tested enum/default
edge cases. The Go schema/generator and reflection-to-generated parser refactor are implementation tooling,
not an additional runtime component to port; tsrs retains its equivalent Rust table and the exact generated
public TypeScript artifact. Go Fourslash fixture changes only rename preference fields to the generated shape;
the full exact-source Fourslash gate covers their behavior. LSP-derived API snapshots remain outside tsrs's
standalone Node API scope, as documented in `docs/LSP.md`. Upstream Go linter scoping and preference-generator
task registration do not apply to Cargo/pnpm builds; their generated public artifact is carried above.

The full dev compiler suite preserves all twelve complete source-11 passing sets and all known nonpassing
artifacts byte-for-byte, excluding timing summaries: Go-history checker 13,463 diagnostics and 12,784
types/symbols; canonical checker 13,463 and 12,783; emit 13,467 diagnostics, 12,784 types/symbols, 13,397
JavaScript/declaration outputs, 149 JS maps and 156 source maps. There are no new failures, crashes or timeouts.
The exact-source Go API, project, auto-import registry and preference tests pass. Workspace tests pass 1,072
with four existing ignored;
the public sync/async API suites pass 691 tests, including the new snapshot tests; all 37 regressions pass.
The full source-12 Go Fourslash gate passes 4,074 groups with 472 skipped, zero failed, and an identical
4,602-event outcome set to source 11. Native and WASM checks, warnings-denied Clippy, source lint, formatting,
and generated-code verification pass; regenerating leaves every tracked crate file byte-identical to its
pre-generation snapshot. The initial sandboxed workspace run failed only because a local Unix socket could not
be created; the complete rerun with socket access passed. An initial Clippy run found an assigning-clones lint
in the new snapshot preference copy; `clone_from` fixed it before the passing run.

No source-12 release performance measurements were collected. Under the user's nonblocking regression policy,
the source-11 measured regressions above remain disclosed, and no claim of source-12 performance equality or
absence of further regressions is made. Evidence is under `target/port-623/0012-43521c80ec/`, including the
exact-source Go diff, full compiler results and pass-set comparison, Go/API/fourslash/workspace and lint logs,
and generated-source hashes. The ignored reference checkout advances to this revision; the published
`Cargo.toml` TypeScript pin remains unchanged until the full source upgrade.

## 13. `edf7da4e93ab6` — preserve reverse-mapped declaration types

Upstream `edf7da4e93ab65e1f7766deae156c35132c0cde3` keeps reverse-mapped properties and index signatures
in declaration emit instead of replacing them with display placeholders. The Rust node builder follows all three
Go changes: placeholders remain for display only; reverse-mapped types without symbols enter the visited-type
guard during declaration serialization; and a separate mapped-declaration node identity limits expansion to 100
levels, restoring its depth on exit. The upstream `reverseMappedDeclarationSerialization` fixture covers indexed,
nested, named, and recursive/growing types. It passes its diagnostic, type, symbol, and JavaScript/declaration
baselines against this exact Go revision. The new `TestReverseMappedDeclarationQuickInfo` also passes through the
Rust LSP subprocess, preserving display behavior.

All twelve full compiler passing sets retain every source-12 pass and known nonpassing outcome; only the new
compiler case is added. Go-history checker: 13,464 diagnostics and 12,785 types/symbols; canonical checker:
13,464 diagnostics and 12,784 types/symbols; emit: 13,468 diagnostics, 12,785 types/symbols,
13,398 JavaScript/declaration outputs, 149 JS maps, and 156 source maps. The full source-13 Go Fourslash gate
passes 4,075 groups with 472 skipped and zero failures: the complete source-12 outcome set is unchanged, with
the new quick-info group added as a pass.

The upstream `reverse mapped declaration consumption` incremental scenario is dumped from the exact Go source
and passes the Rust `tsctests` harness. An isolated direct Go/Rust comparison exercises initial build, no-change
rebuild, comment-only producer edit, and another no-change rebuild. Both compilers report the same three TS2322
consumer errors and emit identical producer `.d.ts` and JavaScript in each phase. The consumer build info is
byte-identical. Producer build info differs only in Rust's existing optional `tsrsCanonicalCheckSignature` field.
The exact-source Go compiler, Fourslash, and incremental tests pass independently. All 37 existing regression
projects pass. The full workspace has 1,072 passing tests and four existing ignored tests; the public Node API
suite passes 910 tests. Native and WASM checks, warnings-denied Clippy, source lint, formatting, and
generated-source verification pass. Regeneration leaves every tracked crate file byte-identical to its
pre-generation snapshot. No performance measurement was collected for source 13 under the user's nonblocking
policy; the known source-11 regressions remain disclosed above.

Evidence is under `target/port-623/0013-edf7da4e93/`, including all compiler pass sets and their complete
comparison, exact-source Go oracle and four-phase build comparison, focused scenario dump and Rust result,
Fourslash raw events/outcome comparison, and workspace, API, lint and build logs. The ignored reference checkout
advances to this revision; the published TypeScript pin remains unchanged until the full source upgrade.

## 14. `02c1c9a6ab49` — preserve a removed CommonJS `with` body

Upstream `02c1c9a6ab49334a5ca9c1d7852647b7a3fad18a` fixes a CommonJS transformer crash when a top-level
`with` statement's nested visitor removes its body. Rust had the same unchecked unwrap. The port visits that
body first, substitutes an empty statement only when the visitor returns none, then visits the expression and
updates the `with` statement, matching the Go operation order. The new upstream
`withStatementCommonJsEmit` compiler case has four forms: the removed `export let a` emits `;`, while an
initialized export, a local variable, and an export in a block keep their expected output. All eight expected
strict-mode and unsupported-`with` diagnostics match. The exact source-14 Go compiler passes all nine fixture
subtests, and Rust passes the diagnostic, type, symbol, and JavaScript baselines.

Full Go-history and canonical checker runs and the full emit run preserve every source-13 passing case and all
known nonpassing outcomes. The new fixture adds one pass to every applicable checker and emit list: Go-history
checker has 13,465 diagnostic and 12,786 type/symbol passes; canonical checker has 13,465 diagnostic and
12,785 type/symbol passes; emit has 13,469 diagnostic, 12,786 type/symbol, and 13,399 JavaScript/declaration
output passes, with the existing 149 JavaScript map and 156 source-map passes unchanged. All 37 existing
regression projects pass. The full workspace has 1,072 passing tests and four existing ignored tests. Native
and WASM checks, warnings-denied Clippy, source lint, and formatting pass. Regenerating with all seven generators
leaves every tracked crate file byte-identical. A direct `tools/gen-check.sh` on the uncommitted transformer edit
reported that intended edit as a diff; the generator write and full before/after crate hash comparison verify
that no generated file changes.

The full exact-source Go Fourslash gate passes 4,075 groups with 472 skipped and zero failures. All 4,603 named
outcomes match source 13 exactly, including its 4,131 passing events. The first sandboxed launch could not bind
the bridge's local Unix socket; the complete rerun with socket access passed.

No source-14 release performance measurements were collected under the user's nonblocking regression policy.
The measured source-11 regressions remain disclosed above; no source-14 performance-equivalence claim is made.
Evidence is under `target/port-623/0014-02c1c9a6ab/`. The ignored reference checkout advances to this revision;
the published TypeScript pin remains unchanged until the full source upgrade.

## 15. `237b14a5d98f` — stabilize merged-declaration diagnostic ownership

Upstream `237b14a5d98f8e8f748ff36002fdd99786230d90` anchors merged-interface TS2320 and TS2430
diagnostics to the symbol's first interface declaration even when a checker first visits another declaration.
The Rust checker now uses that anchor in both Go-history and canonical modes. Canonical mode retains its per-file
execution for index-constraint checks; the earlier virtual four-checker rule no longer chooses the interface
diagnostic site. The enum const-consistency check compares against the first enum declaration. The merged-export
check now runs once per symbol per checker when there are at least two declarations, regardless of which
declaration kind a checker visits first. Existing merged-interface regression expectations are updated from the
exact source-15 Go compiler. The older diagnostic-history investigation has a supersession note.

The exact-source Go `TestLocal` oracle passes all six newly added interface fixture variants and four existing
merged/enum fixtures. Rust passes the new `mergedInterfaceAugmentationDiagnostics`,
`mergedInterfaceInheritanceDiagnostics`, and four `mergedInterfaceDefaultLibraryDiagnostics` configurations.
Full Go-history and canonical checker runs each retain all source-14 passes and add these six diagnostic cases;
Go history has 13,471 diagnostic and 12,786 type/symbol passes, canonical history 13,471 diagnostic and 12,785
type/symbol passes. The full emit run retains all previous passes and adds the same six diagnostic cases plus
two JavaScript cases: 13,475 diagnostics, 12,786 type/symbols, 13,401 JavaScript/declaration outputs, 149 JS maps,
and 156 source maps. No previously nonpassing set changes and there are no new crashes or timeouts.

The exact-source Go CLI produces identical ordered diagnostics for both updated public regression fixtures at
one, four, and eight checkers; reversing the first two declaring roots moves the sole TS2320 to the new first
declaration. The Rust CLI, API, and incremental tests now assert that ownership while retaining the per-file
index-constraint and cache-reuse checks. All 37 regression projects pass. The full workspace passes 1,072 tests
with four existing ignored. The full source-15 Go Fourslash gate passes 4,075 groups with 472 skipped and zero
failures, retaining all 4,603 named source-14 outcomes (4,131 passes and 472 skips). The native dev build, WASM
check, warnings-denied Clippy, source lint, formatting, and diff checks pass. Regeneration with all seven
generators leaves every one of the 1,168 tracked crate files byte-identical to its pre-generation snapshot.
No source-15 release performance measurement was collected under the user's nonblocking regression policy. The
confirmed source-11 regressions remain disclosed above, and no performance-equivalence claim is made for this port.
Evidence is under `target/port-623/0015-237b14a5d9/`. The ignored reference checkout advances to this revision;
the published TypeScript pin remains unchanged until the full source upgrade.

## 16. `688d86d7f647` — resolve binder symbols from remote declarations

Upstream `688d86d7f647781565dc1f6438a4fb09b5179160` adds `getSymbol(decl)` to the sync and async Node APIs.
The Rust core server now implements `getSymbolOfDeclaration`: it acquires the exact cached ordinary source file,
resolves the declaration's encoder index, rejects index zero, out-of-range and non-declaration nodes, rejects
declarations without binder symbols, and returns a file-owned symbol response. The source-file lease is released
on success and error. The protocol inventory and uint32 request decoder include the new method. A Rust
integration test checks file-owned identity against the checker response and validates zero, out-of-range, and
malformed indices. The exact-source Go `TestCreateSourceFile` suite passes.

The five changed public client source files (`async/api.ts`, `sync/api.ts`, `node/node.ts`, `sourceFileCache.ts`,
and `proto.generated.ts`) are byte-identical to this upstream revision. The changed upstream sync, async, and
generator tests are carried with the existing tsrs package-name and earlier snapshot-test edits preserved.
Remote source files retain their owning API and symbol-cache record. Successful declaration lookups are interned
by both binder ID and declaration index; concurrent async lookups share one request, while failures are not
cached. The full public npm API suite passes 925 tests, including the upstream checks for checker symbol identity,
synthetic and configuration nodes, cache clearing, lease release, stale/recreated files, and generator behavior.

All twelve full compiler passing sets and their known nonpassing outcomes match source 15 exactly. Go-history
checker has 13,471 diagnostic and 12,786 type/symbol passes; canonical checker has 13,471 diagnostic and
12,785 type/symbol passes. Emit has 13,475 diagnostic, 12,786 type/symbol, 13,401 JavaScript/declaration,
149 JavaScript-map, and 156 source-map passes. The full exact-source Go Fourslash gate passes 4,075 groups with
472 skipped and zero failures; its 4,603 named outcomes are identical to source 15. All 37 regression projects
pass. The workspace has 1,073 passing tests and four existing ignored. Native and WASM checks, warnings-denied
Clippy, source lint, formatting, and the npm SDK build pass. All 1,168 tracked crate files remain byte-identical
after running the seven code generators. Initial sandboxed workspace and Fourslash runs could not bind their
local Unix sockets; complete reruns with socket access passed.

No source-16 release performance measurements were collected under the user's nonblocking regression policy.
The measured source-11 regressions remain disclosed above, and no performance-equivalence claim is made for
this port. Evidence is under `target/port-623/0016-688d86d7f6/`, including the compiler pass-set and
Fourslash outcome comparisons, exact-source Go API test, full npm API and workspace suites, regression fixtures,
lint and build logs, and generator hashes. The ignored reference checkout advances to this revision; the
published `Cargo.toml` TypeScript pin and `npm/tsrs/UPSTREAM.json` base manifest remain unchanged until the full
source upgrade.

## 17. `e8fd69fc1e19` — generate compiler options and publish config schemas

Upstream `e8fd69fc1e1974f7f7410f79a04adad692dff0dc` moves option definitions to a shared generator and
publishes `tsconfig` and `jsconfig` JSON schemas. No compiler, build or type-acquisition option field is added or
removed. The observable parser change is deletion of seven watch-tuning flags: `watchInterval`, `watchFile`,
`watchDirectory`, `fallbackPolling`, `synchronousWatchDirectory`, `excludeDirectories`, and `excludeFiles`.
The Rust CLI and build parser now reject each one while preserving `--watch`; the exact Go baseline pair for
each flag is TS5023 in compiler mode and TS5072 in build mode. The former watch-option types and parsed fields
are removed. Rust's watcher uses the new parsed-option equality semantics, including type-acquisition's
nil/empty-list equivalence and ordered compiler `paths` comparison. The Rust compiler fixture runner now uses
the exact 91-name generated Go variation inventory, rather than deriving 72 names from declaration flags.

The public npm package contains upstream's byte-identical generated `CompilerOptions` and `proto` files, the
changed enum source files, and both byte-identical JSON schemas. It exports both schemas; its SDK sync inventory
will hash them when the final TypeScript pin advances. The local `gen_decls.py` now accepts the combined
`declarations_generated.go` source and preserves Rust metadata fields that upstream moved out of declarations.
Dry-run regeneration keeps the same order for all 126 compiler and seven build declarations, with compiler
field values unchanged. Go's generated option parsing, build-info, transpile, and declaration metadata keep
their existing Rust behavior; the exact Go `tsoptions`, `transpile`, and `tsctests` packages pass.

All twelve full compiler passing sets and every known nonpassing outcome match source 16 exactly. Go-history
checker retains 13,471 diagnostic and 12,786 type/symbol passes; canonical checker retains 13,471 diagnostic
and 12,785 type/symbol passes. Emit retains 13,475 diagnostic, 12,786 type/symbol, 13,401 JavaScript/declaration,
149 JavaScript-map, and 156 source-map passes. The full exact-source Go Fourslash gate passes 4,075 groups with
472 skipped and zero failures; all 4,603 named outcomes match source 16. The Rust workspace passes 1,076 tests
with four existing ignored, including the new watch-option and equality cases. The public npm API suite passes
925 tests, the SDK typechecks, both schemas appear in the npm package dry run, and all 37 regression projects
pass. Native and WASM checks, warnings-denied Clippy, source lint, formatting, and upstream artifact hashes
pass. Regeneration leaves all 1,166 retained tracked crate files byte-identical.

`--showConfig` and full `--help --all` output were already outside tsrs's supported CLI scope, so source 17's
new show-config scenarios and removed help section do not change those capabilities. Rust's internal parsed
configuration representation also does not distinguish nil from empty file-name/content-mapper collections
or nil entries inside them, unlike Go's pointer/slice equality; ordinary parsed configurations and the
observable conformance sets above retain parity. No source-17 release performance measurement was collected
under the user's nonblocking regression policy; the measured source-11 regressions remain disclosed above,
and no performance-equivalence claim is made. Evidence is under `target/port-623/0017-e8fd69fc1e/`. The ignored
reference checkout advances to this revision; the published TypeScript pin and base SDK manifest remain
unchanged until the full upgrade.

## 18. `15dee00a1c65` — detect array and tuple serialization cycles

Upstream `15dee00a1c6549a4953ab17ccac4c56d252ba4cc` routes array and tuple references through
`visitAndTransformType` so recursive containers use the same visited-type cycle check as other serialized
types. The Rust node builder now separates array/tuple serialization from ordinary type-reference
serialization and canonicalizes deferred and regular container references to one type identity before
checking for a cycle. It does not apply the shared `Array` symbol or tuple-origin depth limit to finite
container nesting. The visited-type check runs before cache lookup, and the serialized-type cache key
includes the current `infer` type parameters, matching the Go control flow.

The new `recursiveContainerDeclarationSerialization` compiler fixture passes diagnostics, types, symbols,
JavaScript and declaration output. Four unnameable recursive results report TS5088; named recursive array
and tuple aliases serialize without errors, and finite 13-level tuple and array containers remain expanded.
The changed `conditionalTypes1` and `recursiveTypeInference2` baselines also pass. The exact source-18 Go
compiler passes those fixtures, the nine-edit incremental recursive tagged-tuple scenario, and the changed
quick-info truncation test. The incremental case keeps the consumer's recursive `Doc` type across comment,
union and no-change rebuilds, then reports TS2322 for an invalid number assignment even after deleting
build info. The corresponding Rust `tsctests` replay passes byte-for-byte across all nine edits. The full
exact-source Go Fourslash gate passes 4,075 groups, skips 472 and fails none; all
4,603 named outcomes match source 17.

All twelve full compiler passing sets retain every source-17 pass and every known nonpassing outcome.
Go-history checker passes 13,472 diagnostic and 12,787 type/symbol baselines; canonical checker passes
13,472 diagnostic and 12,786 type/symbol baselines. Emit passes 13,476 diagnostic, 12,787 type/symbol,
13,402 JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines. The 37 existing
regression projects pass. The full workspace passes 1,076 tests with four existing ignored. Native and
WASM builds, warnings-denied Clippy, source lint, formatting and generated-source verification pass;
regeneration leaves all 1,166 tracked crate files byte-identical. No source-18 release performance
measurement was collected under the user's nonblocking migration policy; the source-11 regressions remain
disclosed above, and no performance-equivalence claim is made for this port.

Evidence is under `target/port-623/0018-15dee00a1c/`, including complete compiler pass-set comparisons,
the exact-source Go oracle and incremental logs, Fourslash outcome comparison, workspace and lint logs,
and generator hashes. The ignored reference checkout advances to this revision; the published TypeScript
pin and base SDK manifest remain unchanged until the full source upgrade.

## 19. `af36d532b012` — follow merged export-alias chains for declaration visibility

Upstream `af36d532b0120413a8b6d2157a7f5be7f8d61f03` extends `isAccessible` when a direct
symbol match fails: it follows a still-aliased `resolvedAliasSymbol` through `resolveAlias` and
`getMergedSymbol`, comparing each target with the requested merged symbol. The Rust checker mirrors
that loop and uses a set of seen symbol identities to stop circular alias chains. Its ordinary
qualification and external-module checks still run after a match. This preserves accessible class
types through `export =` with separate type exports and through merged re-export aliases, including
classes with private members.

The exact source-19 Go testrunner passes all five new compiler cases across 13 module configurations
and all seven existing cases whose type or symbol baselines changed. Rust matches diagnostics,
`.types`, `.symbols`, and JavaScript/declaration output on the same 13 new configurations. The
circular merged-alias case intentionally reports two TS2484 conflicts; the two-merged-alias case
reports TS2484 for `Factory` in each module mode. The changed existing baselines also pass in the
full suite.

All twelve full compiler passing sets retain every source-18 pass and every known nonpassing
outcome. Go-history checker passes 13,485 diagnostic and 12,800 type/symbol baselines; canonical
checker passes 13,485 diagnostic and 12,799 type/symbol baselines. Emit passes 13,489 diagnostic,
12,800 type/symbol, 13,415 JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines.
The full exact-source Go Fourslash gate passes 4,075 groups, skips 472, and fails none; all 4,603
named outcomes match source 18. All 37 regression projects pass. The workspace has 1,076 passing
tests and four existing ignored tests. Native and WASM builds, warnings-denied Clippy, source lint,
formatting, and generated-source verification pass; regeneration leaves all 1,166 tracked crate
files byte-identical. Initial sandboxed Fourslash and workspace runs could not bind local Unix
sockets; complete reruns with socket access passed.

No source-19 release performance measurement was collected under the user's nonblocking migration
policy; the source-11 measured regressions remain disclosed above, and no performance-equivalence
claim is made for this port. Evidence is under `target/port-623/0019-af36d532b0/`, including the
compiler pass-set and Fourslash outcome comparisons, exact-source Go oracle, workspace and regression
logs, build and lint logs, and generator hashes. The ignored reference checkout advances to this
revision; the published TypeScript pin and base SDK manifest remain unchanged until the full source
upgrade.

## 20. `bf00f213ffd6` — add keyed Promise methods to the ESNext library

Upstream `bf00f213ffd67ce4cc765c0db341e86c7057be44` gives `esnext.promise` its own
library file, referenced by the `esnext` aggregate, rather than aliasing
`lib.es2025.promise.d.ts`. The new library still references ES2020 and ES2025 promise
declarations, then adds `Promise.allKeyed` and `Promise.allSettledKeyed`. Both methods
preserve the input object's string and symbol keys, remove property `readonly`, and map
each value through `Awaited`; the latter wraps each result in `PromiseSettledResult`.
The Rust bundled-library generator includes the file in its name and embedded-content
inventories, and the ordered `--lib` option map now resolves `esnext.promise` to it after
the ES2026 feature aliases. The new library, changed `lib.esnext.d.ts`, and both npm
configuration schemas are byte-identical to upstream. The npm package dry run contains
and exports both parsed JSON schemas.

The exact source-20 Go `TestLocal/promiseAllKeyed.ts` case and its bundled/compiler/options
package tests pass. The new Rust compiler case matches diagnostics, `.types`, `.symbols`,
and JavaScript output. Its four TS2345 errors intentionally reject `number`, `string`,
`null`, and `undefined` inputs under the `object` constraint. `Promise.try` remains
available through the referenced ES2025 promise library.

All twelve full compiler passing sets retain every source-19 pass and known nonpassing
outcome. Go-history checker passes 13,486 diagnostic and 12,801 type/symbol baselines;
canonical checker passes 13,486 diagnostic and 12,800 type/symbol baselines. Emit passes
13,490 diagnostic, 12,801 type/symbol, 13,416 JavaScript/declaration, 149 JavaScript-map,
and 156 source-map baselines. Each gained only the new `promiseAllKeyed` case where that
baseline exists. The full exact-source Go Fourslash gate passes 4,075 groups, skips 472,
and fails none; all 4,603 named outcomes match source 19. All 37 regression projects
pass. The workspace passes 1,076 tests with four existing ignored. Native and WASM
builds, warnings-denied Clippy, source lint, formatting, and npm schema/package checks
pass. Regeneration leaves all 1,167 tracked and new crate files byte-identical.

No source-20 release performance measurement was collected under the user's nonblocking
migration policy. The measured source-11 regressions remain disclosed above, and no
performance-equivalence claim is made for this port. Evidence is under
`target/port-623/0020-bf00f213ff/`. The ignored reference checkout advances to this
revision; the published TypeScript pin and base SDK manifest remain unchanged until the
full source upgrade.
