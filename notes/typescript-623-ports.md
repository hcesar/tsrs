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

## 21. `82f054616325` — clarify security information

Upstream `82f054616325c1550b184f26a64f26b44ce6b99d` adds Workspace Trust/content-mapper
and Go-compiler CVE guidance only to `.github/skills/security-report-check/SKILL.md`.
`tsrs` has a user-facing `SECURITY.md` rather than an equivalent security-report skill.
Its Language Service section now says that editor integrations decide workspace trust,
direct LSP clients must make their own trust decision, and a trusted workspace may run
project tools from `node_modules`. It also states the current tsrs boundary: the language
server does not start content mappers, while compiler/build mode can start configured
mappers only with `--runExternalCode`. The upstream Go-compiler CVE and `govulncheck`
advice does not apply to this Rust compiler, so no Go-specific claim or new release
policy was copied.

The upstream source-20-to-21 diff contains only that skill file; the `tsc` tree is
`639627690565280fef5ece5479386bf63db835f1` at both revisions. The source and fixture
directories used by the tsrs conformance runner are unchanged. The ignored reference
checkout advances to source 21, while the published TypeScript pin remains unchanged.
Only `SECURITY.md` and this port record change in tsrs; there is no compiler, library,
runtime-asset, test, or capability change.

Full checker conformance in Go and canonical histories and full emit conformance were
rerun against the final dev build. All twelve complete passing sets and every known
nonpassing set match source 20 exactly: Go-history checker has 13,486 diagnostic and
12,801 type/symbol passes; canonical checker has 13,486 diagnostic and 12,800
type/symbol passes. Emit has 13,490 diagnostic, 12,801 type/symbol, 13,416 JavaScript/
declaration, 149 JavaScript-map, and 156 source-map passes. No new crash or timeout
occurred. The source-20 Fourslash, workspace, regression, native/WASM, npm and generator
results are reused because their product inputs and upstream fixture trees are unchanged;
they were not independently rerun for this documentation-only entry. The dev build,
warnings-denied Clippy, source lint, formatting, and whitespace checks pass.

The release compiler binary is byte-identical before and after this port (SHA-256
`3edc24074ab2ce687196eb816f114115c6292f3d27bf06a5e918fdfaee07a1dd`), and no
runtime assets change. No new release performance measurements were collected; the
source-11 measured regressions remain disclosed above. Source identity, exact passing-set
comparison, full gate logs, and build/lint evidence are under
`target/port-623/0021-82f0546163/`.

## 22. `59f5b0233f9d` — typed path prep bugfixes

Upstream `59f5b0233f9d2411aacb77c7342ddb31002dd3dd` changes typed path preparation across
the API, compiler, resolver, source-map reader/writer, incremental build, project watcher, and
language server. Most of its 282 changed paths are regenerated source-map, build and Fourslash
baselines. The Rust port encodes non-file document URIs without collapsing exceptional path
segments, preserves their case-sensitive identity, and resolves imports, package exports and
root-directory candidates through encoded dynamic paths. Project-reference symlink and
auto-import paths retain their original spelling. The upstream npm path implementation and
its changed API tests are carried into `npm/tsrs`.

Source-map emit omits an empty `sourceRoot`; reverse mapping handles explicit empty/null
sources, URL roots, invalid indices and duplicate source paths. Incremental snapshots resolve
nested triple-slash references from the referring source file. Content-mapper manifest watches
compare canonical paths. Project watchers retain original path spelling, and closing a project
session cancels and drains background tasks, idle cache work and its current snapshot. The API
rejects empty `createSourceFile` names and resolves relative names from its session directory;
auto-import preparation uses the program's resolved source file. LSP completion resolution
rejects relative or malformed dynamic file names. The existing Rust project builder already
clones API state and rejects failed snapshots before adoption, matching the upstream rollback
change without new code.

The source commit also adds cancellation of npm installs for automatic type acquisition and
closes LSP-attached API sessions on server shutdown. Neither operation has a tsrs counterpart:
automatic type acquisition and attached API sessions remain unported, while the standalone
Node API server is supported. The Go test harness changes do not alter product behavior.

The exact-source Go path, module, symlink, source-map, LSP-conversion, project, API,
incremental and VFS package suites pass. The new Rust resolver and source-mapper scenarios,
dynamic URI/path/glob and symlink cases, API empty-name case, project watcher/shutdown cases,
and LSP completion validation pass. Full checker conformance in Go and canonical histories and
full emit conformance pass, with every passing and nonpassing set identical to source 21:
13,486 diagnostic, 12,801 Go-history `.types` and `.symbols`, 12,800 canonical `.types` and
`.symbols`, 13,416 JavaScript/declaration, 149 JavaScript-map, and 156 source-map passes. No
new crash or timeout occurred.

The exact-source-22 Fourslash run passes 4,075 groups, skips 472 and fails none; all 4,603
named outcomes match source 20 (4,131 passes and 472 skips). The first full run used a stale
`fourslash_server` binary and showed a quick-info truncation mismatch; rebuilding that binary
made the unchanged upstream case pass, and the full run above uses the fresh binary. The
`DocumentUri.FileName` differential fixture was regenerated from source-22 Go for its new
non-file URI encoding. The full workspace passes 1,103 tests with four existing ignored; the
Node transport round trip passes 15 client tests, the npm API suite passes 933, and all 37
regression projects pass. The native dev build, WASM target check, warnings-denied Clippy,
source lint and formatting pass. All seven generators plus formatting preserve all 1,169
crate files byte-for-byte. A malformed absolute-looking completion filename now returns an
error instead of panicking in the Rust LSP server.

No source-22 release performance measurements were collected under the user's nonblocking
migration policy. The measured source-11 regressions remain disclosed above; no performance
equivalence claim is made for this port. Full gate logs, passing-set comparisons, exact-source
Go oracle and Fourslash results, and generator integrity evidence are under
`target/port-623/0022-59f5b0233f/`.

## 23. `09b1db061731` — elide erased named imports beside a default import

Upstream `09b1db0617311f3b39f165bb3480c0ce2f718b27` changes the type eraser under
`verbatimModuleSyntax`: when every named import was type-only and was erased, a remaining
default import keeps the import declaration, so the generated empty `{}` is omitted. A
source-written `{}` stays in the output; a named list that retains a value import is also
unchanged. The Rust transformer mirrors the Go import-clause branch. The exact new upstream
`verbatimModuleSyntaxDefaultImportTypeOnlyNamed` fixture passes the Go reference's nine
baseline/structural checks and matches tsrs diagnostics, `.types`, `.symbols`, and emitted
JavaScript. In particular, its `A1`, `A5`, and `A6` cases become plain default imports,
while `import A4, {} from "./a"` retains its braces.

All twelve full compiler passing sets retain every source-22 pass and known nonpassing
outcome, adding only the new fixture where its baseline exists. Go-history checker passes
13,487 diagnostic and 12,802 type/symbol baselines; canonical checker passes 13,487
diagnostic and 12,801 type/symbol baselines. Emit passes 13,491 diagnostic, 12,802
type/symbol, 13,417 JavaScript/declaration, 149 JavaScript-map, and 156 source-map
baselines. No new crash or timeout occurred. The exact-source Fourslash gate passes 4,075
groups, skips 472, and fails none; all 4,603 named outcomes match source 22. The full
workspace passes 1,103 tests with four existing ignored tests, and all 37 regression
projects pass. The dev build, WASM target check, warnings-denied Clippy, source lint and
formatting pass. All seven generators plus formatting leave all 1,169 tracked crate
files byte-identical. The initial sandboxed workspace run could not create local Unix
sockets; the complete rerun with socket access passed.

No source-23 release performance measurement was collected under the user's nonblocking
migration policy. The measured source-11 regressions remain disclosed above; no
performance-equivalence claim is made for this port. Evidence is under
`target/port-623/0023-09b1db0617/`, including exact-source Go output, full compiler
pass-set and Fourslash outcome comparisons, workspace, lint, and generator logs. The ignored
reference checkout advances to source 23; the published TypeScript pin and base SDK
manifest remain unchanged until the full source upgrade.

## 24. `2f9fd09a104e` — source-phase imports

Upstream `2f9fd09a104ef14268944a0aeb9cdc4d80abb6a6` adds static
`import source binding from "module"` and dynamic `import.source("module")`.
The Rust port follows its `SourceKeyword` scanner/parser/AST additions, grammar
and TS18111–TS18115 diagnostics, `AbstractModuleSource` and
`Promise<AbstractModuleSource>` types, source-phase module-resolution elision,
program-reuse and package-name rules, printing and CommonJS transform behavior,
and language-service auto-import, organization and source-definition behavior.
The `esnext` library now includes `lib.esnext.modulesource.d.ts`, and the
`esnext.modulesource` option selects that file. The npm AST, syntax-kind,
encoder and schema changes were copied from the matching upstream revision.

All 31 new `importSource` conformance variants pass diagnostics, `.types`,
`.symbols` and JavaScript emit. The three full compiler gates retain every
source-23 pass and known nonpass, with no crashes or timeouts. Go-history and
canonical checker diagnostic passes rise to 13,518; Go-history type and symbol
passes rise to 12,833, canonical to 12,832. Emit has 13,522 diagnostic, 12,833
type and symbol, 13,448 JavaScript/declaration, 149 JavaScript-map and 156
source-map passes. The exact-source Go parser, scanner, compiler and API
encoder suites pass. A Rust program test verifies source-phase specifiers never
resolve or retain files, while changing their phase prevents stale program
reuse. Parser, scanner, printer and codec focused tests pass.

The exact-source Fourslash gate passes 4,083 groups, skips 472 and fails none;
all 4,615 named outcomes match or add to source 23 (12 new passes, no changed
or lost outcomes). The full Rust workspace passes 1,106 tests with four
existing ignored tests; all 37 regression projects pass. The npm API suite
passes 935 tests, including the new sync and async source-phase cases. The
native dev build, WASM target check, warnings-denied Clippy, source lint and
formatting pass. The seven generators are run and checked for idempotence.

The new `SourceKeyword` shifts numeric `SyntaxKind` values in the API wire
format. The source-23 binary codec goldens could no longer decode under the
source-24 enum, and the static Go API-shape fixture likewise contained old
kind numbers. Both sets of fixtures were regenerated from the exact source-24
Go server and updated local JavaScript client; codec byte parity, print
parity and all decoder tests now pass. The `AbstractModuleSource` conformance
errors in the first focused run exposed a missing `esnext.modulesource` option
mapping, which was added before the full gate. The published TypeScript pin
and base SDK manifest remain unchanged until the whole source upgrade.

No source-24 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above; no performance-equivalence claim is made for this port.
Evidence is under `target/port-623/0024-2f9fd09a10/`, including pass-set and
Fourslash outcome comparisons, exact-source Go and codec goldens, workspace,
regression, npm, lint and generator logs.

## 25. `a4b1410230c7` — correct `Array.at` parameter documentation

Upstream `a4b1410230c7299957ec3f8f8dee4cccc55a4798` replaces “code unit” with
“item” in the `index` parameter documentation for `Array.at`, `ReadonlyArray.at`,
and eleven typed-array `at` signatures. The Rust bundled
`lib.es2022.array.d.ts` is copied byte-for-byte from the exact source-25 Go
reference; declaration signatures and executable code do not change. The
generator's embedding list remains byte-identical after formatting. The
ignored reference checkout advances to source 25, while the published
TypeScript pin and base SDK manifest remain unchanged until the whole upgrade.

All twelve full compiler pass and nonpass sets are identical to source 24,
without crashes or timeouts. Go-history checker passes 13,518 diagnostic and
12,833 type/symbol baselines; canonical checker passes 13,518 diagnostic and
12,832 type/symbol baselines. Emit passes 13,522 diagnostic, 12,833 type and
symbol, 13,448 JavaScript/declaration, 149 JavaScript-map, and 156 source-map
baselines. The exact `compiler/indexAt` fixture passes all three variants in
diagnostics, types, symbols and JavaScript emit. Exact-source Fourslash passes
4,083 groups, skips 472 and fails none; all 4,615 named outcomes match source
24. The full Rust workspace passes 1,106 tests with four existing ignored tests;
the npm API suite passes 935 tests, and all 37 regression projects pass. The
dev build, WASM target check, warnings-denied Clippy, source lint and formatting
pass. All seven generators leave 1,172 tracked crate files byte-identical.
The first workspace attempt was denied a local Unix socket by the sandbox;
the complete rerun with socket access passed. The npm shutdown test must run
from the npm package directory for its child process to resolve the package;
the full suite passed there.

The compatibility table and README summaries need no change: this port alters
documentation wording without changing a capability or a cited pass count.
No source-25 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above; no performance-equivalence claim is made for this port.
Evidence is under `target/port-623/0025-a4b1410230/`, including exact-source
library hashes, full compiler pass-set and Fourslash outcome comparisons,
workspace, npm, regression, lint, WASM and generator logs.

## 26. `efbf3b506754` — VS Code JSX auto-insert in content-mapped files

Upstream `efbf3b5067547539c9cf8331d51943543c8bc91a` changes only the
TypeScript VS Code extension: it moves the JSX auto-insert provider into the
selector-scoped registrations so content-mapped extensions are included, and
it disposes a conditional-registration listener when that selector changes.
tsrs ships the language server but not that VS Code client. Its existing
`_vs_onAutoInsert` request implementation is unchanged, and language-server
content mappers remain unported. There is therefore no tsrs production code
to change for this upstream client commit; JSX auto-insert in content-mapped
files remains unavailable through tsrs. The existing compatibility table
already records that language-server content-mapper gap. The ignored
reference checkout advances to source 26, while the published TypeScript
pin and base SDK manifest remain unchanged until the full source upgrade.

All twelve full checker and emit pass and nonpass sets match source 25
exactly, with no crashes or timeouts. Go-history checker passes 13,518
diagnostic and 12,833 type/symbol baselines; canonical checker passes 13,518
diagnostic and 12,832 type/symbol baselines. Emit passes 13,522 diagnostic,
12,833 type/symbol, 13,448 JavaScript/declaration, 149 JavaScript-map and
156 source-map baselines. The exact-source Fourslash gate passes 4,083 groups,
skips 472 and fails none; all 4,615 named outcomes match source 25. The dev
build, warnings-denied Clippy, source lint and formatting pass. The Rust
workspace and generators were not rerun because this port changes no Rust,
npm or generator input; source 25's full workspace and generator checks passed.

The first Fourslash attempt could not bind a local Unix socket in the sandbox.
An initial socket-enabled run used the gate helper's default Cargo.toml pin
(the unchanged published base), rather than source 26, and encountered that
older reference's known quick-info mismatch. The final run used a fresh
source-26 snapshot and the rebuilt server, and passed. No source-26 release
performance measurement was collected under the user's nonblocking migration
policy; no runtime code changes in this port. The measured source-11
regressions remain disclosed above. Evidence is under
`target/port-623/0026-efbf3b5067/`, including the exact-source snapshot
stamp, full compiler pass-set comparison and Fourslash outcome comparison.

## 27. `dca8e236a227` — bump `brace-expansion` to 5.0.12

Upstream `dca8e236a227e37183dcf1dd4e51b22571f5b558` changes only its
`package-lock.json`, moving `brace-expansion` from 5.0.9 to 5.0.12. tsrs uses
pnpm, and its existing lockfile already selects 5.0.12 with the exact upstream
SHA-512 integrity. This is a transitive development-tool dependency through
`void` and `minimatch`; there is no tsrs code or lockfile change to make. The
installed package reports 5.0.12, and `minimatch` successfully expands a
brace-pattern smoke case. The ignored reference checkout advances to source
27, while the published TypeScript pin and base SDK manifest stay unchanged.

All twelve complete checker and emit pass and nonpass sets match source 26,
with no crashes or timeouts. Go-history checker passes 13,518 diagnostic and
12,833 type/symbol baselines; canonical checker passes 13,518 diagnostic and
12,832 type/symbol baselines. Emit passes 13,522 diagnostic, 12,833
type/symbol, 13,448 JavaScript/declaration, 149 JavaScript-map and 156
source-map baselines. The exact-source Fourslash gate passes 4,083 groups,
skips 472 and fails none; all 4,615 named outcomes match source 26. The dev
build, warnings-denied Clippy, source lint and formatting pass. The Rust
workspace, npm API suite and generators reuse their last successful checks:
the upstream compiler, library, generator and SDK input trees, as well as
all tsrs product inputs, are byte-identical to source 26.

The frozen pnpm lock passes its supply-chain check, but an install attempt
exits with the repository's existing ignored-build-script policy for
`esbuild` and `workerd`; the exact package is installed and the focused
dependency smoke passes. This does not affect the compiler or Fourslash
gates. No compatibility table or README status changes are needed because
this port changes no user-visible capability or cited pass count.

No source-27 release performance measurement was collected under the user's
nonblocking migration policy. The rebuilt dev compiler, test runner and
Fourslash server binaries have the same SHA-256 hashes as before this port,
and the runtime source and assets are unchanged. The measured source-11
regressions remain disclosed above. Evidence is under
`target/port-623/0027-dca8e236a2/`, including source-tree and binary
identity, dependency resolution, complete compiler pass-set and Fourslash
outcome comparisons, and build and lint logs.

## 28. `0ad3777160ac` — VS Code LSP response middleware

Upstream `0ad3777160acc032a2063da0cb0edd02be8b23a9` adds
`registerLspMiddleware` to its VS Code extension API. The new client-side
registry transforms language-feature responses after the server replies,
with ordered callbacks, cloned values, cancellation and error fallback, and
registrations that survive server restarts. It also exports type-only
`@typescript/typescript/unstable/vscode` declarations and moves the LSP
protocol generator from `tsc/internal/lsp/lsproto/_generate/` to
`tools/scripts/lsp/`. The generated Go protocol bindings, Go compiler and
LSP-server behavior, and conformance fixtures are unchanged.

tsrs ships an LSP server, not a VS Code extension. Users can point the
upstream TypeScript extension at tsrs, so that extension's middleware can
operate on tsrs responses without a server change. tsrs has no extension
registration API to implement, and adding a `tsrs/unstable/vscode` export
would claim support for a client it does not provide. The corresponding
tsrs changes update the Rust protocol generator's two moved-source
attributions, the LSP guide, and the compatibility table's explicit
extension-API status. The ignored reference checkout advances to source
28; the published TypeScript pin and base SDK remain unchanged.

All twelve complete checker and emit pass and nonpass sets match source
27 exactly, with no crashes or timeouts. Go-history checker passes 13,518
diagnostic and 12,833 type/symbol baselines; canonical checker passes
13,518 diagnostic and 12,832 type/symbol baselines. Emit passes 13,522
diagnostic, 12,833 type/symbol, 13,448 JavaScript/declaration, 149
JavaScript-map and 156 source-map baselines. The exact-source Fourslash
gate passes 4,083 groups, skips 472 and fails none; all 4,615 named
outcomes match source 27. The LSP protocol generator is byte-identical
after formatting, and the generated-Go-to-Rust check finds all 3,686
names. The dev build, warnings-denied Clippy, source lint and formatting
pass. The full Rust workspace, npm API suite and unrelated generators
were not rerun: their runtime and generator inputs are unchanged from
source 27, whose preceding full checks passed.

No source-28 release performance measurement was collected under the
user's nonblocking migration policy. The rebuilt dev compiler, test
runner and Fourslash server binaries have the same SHA-256 hashes as
source 27; no runtime code changed. The measured source-11 regressions
remain disclosed above. Evidence is under
`target/port-623/0028-0ad3777160/`, including upstream source-tree and
binary identity, complete compiler pass-set and Fourslash outcome
comparisons, LSP generator identity, build and lint logs.

## 29. `d2f9dd68cfff` — checker-scoped merged symbol API methods

Upstream `d2f9dd68cfff50ca3c2fdf3152f808fd9d4ca2d6` adds four
checker requests: `getMergedSymbol`, `getSymbolOfNode`,
`getSymbolOfDeclarationForChecker`, and `getParentOfSymbolForChecker`.
The matching Rust checker exports and API handlers use the specified
snapshot/project's persistent checker, resolve node or symbol handles
through its registry, and return nullable symbols where the Go handlers
do. The sync and async SDKs, generated protocol types, generator methods,
and exact upstream tests advance with the source. A Rust real-program
test covers merged namespaces, missing node symbols, merged parents and
late-bound computed members. The existing file-owned `getSymbol(decl)`
continues to return the raw binder symbol; its no-symbol behavior remains
unchanged even though upstream removes redundant assertions for it.
The API coverage matrix now has 177 methods (58 core, 119 checker). The
ignored reference checkout advances to source 29; the published TypeScript
pin and base SDK manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass and nonpass sets match source
28 exactly, without crashes or timeouts. Go-history checker passes 13,518
diagnostic and 12,833 type/symbol baselines; canonical checker passes
13,518 diagnostic and 12,832 type/symbol baselines. Emit passes 13,522
diagnostic, 12,833 type/symbol, 13,448 JavaScript/declaration, 149
JavaScript-map and 156 source-map baselines. Exact-source Fourslash
passes 4,083 groups, skips 472 and fails none; all 4,615 named outcomes
match source 28. Go API and checker package tests pass. Four new upstream
sync/async client cases pass against both the source-29 Go server and tsrs;
the generator parity case and full npm API suite pass (939/939). The full
Rust workspace passes 1,108 tests with four existing ignored tests, and
the regression projects pass. The dev CLI/testrunner and Fourslash server
builds, WASM target check, warnings-denied Clippy, source lint and format
check pass. All seven generators leave 1,172 tracked crate files
byte-identical.

The first workspace and Fourslash attempts could not bind local Unix
sockets in the sandbox; complete reruns with socket access passed. An
initial full npm run from the repository root reached 938/939, with its
shutdown child unable to resolve the package from that working directory;
the full run from `npm/tsrs` passed. No source-29 release performance
measurement was collected under the user's nonblocking migration policy;
these changes add API methods and checker exports without altering the
compiler's existing call paths. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0029-d2f9dd68cf/`,
including exact-source Go and npm runs, full compiler pass-set and
Fourslash outcome comparisons, workspace, regression, WASM, lint and
generator logs.

## 30. `7e5d1c1c1de4` — retry Darwin FSEvents delivery tests

Upstream `7e5d1c1c1de408086e2dd74cb42d5b9c6ba77461` changes only
Go tests: nine Darwin-specific FSEvents event-delivery cases now run
through `runWithRetry`, with fresh temporary directories and scaled
timeouts on each attempt. Tsrs's generic watcher tests already call
`run_with_retry` through `run_for_each_watcher`, with the same three
attempts and 1×/5×/15× timeout scaling. None of the nine modified
Darwin-specific Go cases has a Rust counterpart yet, so this port aligns
the retry-helper description and records that test coverage gap; it does
not change production behavior or claim new coverage. The ignored
reference checkout advances to source 30. The published TypeScript pin
and base SDK manifest remain unchanged until the full upgrade.

The exact-source Go `fswatch` package passes with native FSEvents delivery
outside the sandbox. Its first sandboxed attempt could not start FSEvents
streams, so its failures were environmental; both logs are preserved.
All twelve complete checker and emit pass and nonpass sets match source
29 exactly, with no new crashes or timeouts: Go-history checker passes
13,518 diagnostic and 12,833 type/symbol baselines; canonical checker
passes 13,518 diagnostic and 12,832 type/symbol baselines; emit passes
13,522 diagnostic, 12,833 type/symbol, 13,448 JavaScript/declaration,
149 JavaScript-map and 156 source-map baselines. Exact-source Fourslash
passes 4,083 groups, skips 472 and fails none; all 4,615 named outcomes
match source 29. The dev CLI/testrunner and Fourslash server build,
warnings-denied Clippy, source lint and format check pass. The full
Rust `tsrs_fswatch` all-targets suite passes: 45 unit and three native
macOS integration tests. The full Rust workspace and unrelated npm,
WASM and generator checks were not rerun: their runtime and generator
inputs are unchanged from source 29, whose complete checks passed.
No source-30 release performance measurement was collected under the
user's nonblocking migration policy. No production runtime code changed;
the measured source-11 regressions remain disclosed above. Evidence is
under `target/port-623/0030-7e5d1c1c1d/`.

## 31. `ed4807212c28` — strongly typed file paths

Upstream `ed4807212c28c90777c1d7ef2bf8e47af5d08519` introduces
`RootedPath`, `RootedFilePath`, `RootedDirectoryPath`, `PathKey` and
`CaseSensitivity` across the Go compiler, project system, API, virtual
filesystem and test infrastructure. The corresponding Rust port focuses on
the observable semantics: a dynamic URI with no authority separator is not
treated as a valid root, and common-parent grouping uses canonicalized
components on case-insensitive filesystems. Auto-imports rebase unresolved
relative module augmentations from the importing file, retain extensionless
specifier text, and keep rooted ambient module names rooted. A trailing-slash NodeNext
directory import now reports TS2834, rather than TS2835. The test runner
preserves the parsed configuration's base directory and case sensitivity, so
TS6059 explains the matching include pattern and emits related TS1408.
The new empty/dot-stem filename and trailing-slash type-reference fixtures
also pass. Existing Rust watch-root handling already normalizes `..`.

The npm SDK advances its source-31 API, AST, enum, path, protocol and test
files, including the `unstable/path` export. The Rust API sends numeric
`caseSensitivity` in `initialize`, omits `configFileName` for inferred
projects, resolves raw compiler-option paths against the server directory,
rejects invalid document identifiers, and resolves relative callback
`realpath` results from the queried path's directory. The Go Fourslash
subprocess bridge now accepts upstream's typed virtual-filesystem paths and
project-state path keys. The Rust compiler continues to use its established
string and canonical `Path` storage internally; the Go-wide compile-time
newtype conversion is not mechanically mirrored in every Rust signature.
The ignored reference checkout advances to source 31; the published
TypeScript pin and base SDK manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass and nonpass sets match source 30,
with no lost passing cases or new failures, crashes or timeouts. The three
new compiler cases pass in both histories and emit. Go-history checker passes
13,521 diagnostics and 12,836 type/symbol baselines; canonical checker
passes 13,521 diagnostics and 12,835 type/symbol baselines. Emit passes
13,525 diagnostics, 12,836 type/symbol, 13,451 JavaScript/declaration, 149
JavaScript-map and 156 source-map baselines.
The exact-source Fourslash suite passes 4,084 groups, skips 472 and fails
none. All 4,615 named source-30 outcomes are preserved; the five new
empty/dot-stem rename outcomes pass. Its first run found a CSS-module
auto-import mismatch at the newly added extensionless and rooted cases;
the auto-import representation now preserves both kinds of module name,
and the full rerun passes all 4,620 named outcomes.
The source-31 npm SDK build and source-condition typecheck pass. All 18
changed API/AST/enum source files match upstream byte for byte; eight
changed test files match after the tsrs package-name edit, and reversing
the existing async connection-loss patch restores upstream's client file.
The full npm API suite passes 953/953, including the new typed-path and
relative callback-realpath cases. Exact-source Go path, options, AST,
encoder and request-filesystem package tests pass. The regression
projects and WASM target check pass.
The full Rust workspace passes 1,111 tests with four existing ignored
tests. The initial workspace run exposed three stored source-24 checker
oracle responses for document identifiers that source 31 now rejects.
An exact-source Go oracle confirmed all three new errors; a three-row
override updates those expectations while retaining the other 81
checker-response comparisons. The final workspace and focused API
configuration test pass. The dev CLI/testrunner and Fourslash-server
builds, warnings-denied Clippy, source lint and formatting pass. All seven
generators plus formatting leave 1,172 tracked crate files byte-identical.

No source-31 release performance measurement was collected under the
user's nonblocking migration policy. The measured source-11 regressions
remain disclosed above. Evidence is under
`target/port-623/0031-ed4807212c/`, including exact-source Go and npm
API results, full compiler pass-set and Fourslash outcome comparisons,
workspace, regression, WASM, lint and generator-integrity logs.

## 32. `cb72b2e17b75` — update localization files

Upstream `cb72b2e17b75557db7e3286d835544aef5d85f6c` refreshes the
OneLoc handbacks and deterministic runtime archives for 13 locales. The 26
JSON and gzip files in `crates/tsrs_diagnostics/loc/` match that commit byte
for byte; every handback has 2,220 sorted diagnostic keys, and every gzip
payload has the matching key/value map. The new
`tools/gen-diagnostics/import_localization.py` ports the upstream handback
rules to this repository's Python tooling: only completed `Loc` targets
replace existing translations, pending `Update` and unstated targets keep
their previous text, obsolete and English-equivalent entries are removed,
invalid metadata leaves the output unchanged, and the result is sorted CRLF
JSON. The locale checker validates canonical formatting, placeholders and
archive contents in `tools/gen-check.sh`; its importer cases mirror the new
upstream Go tests. The upstream Azure OneLoc pipeline is Go-repository
infrastructure and is not copied here.

Localized output remains a pre-existing gap: tsrs does not yet load these
catalogs, and `--locale` still leaves English messages unchanged. This data
port does not claim to implement that runtime subsystem. The ignored
reference checkout advances to source 32; the published TypeScript pin and
base SDK manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets match source 31 with
no lost passes, new failures, crashes or timeouts. Go-history checker passes
13,521 diagnostics and 12,836 type/symbol baselines; canonical checker passes
13,521 diagnostics and 12,835 type/symbol baselines. Emit passes 13,525
diagnostics, 12,836 type/symbol, 13,451 JavaScript/declaration, 149
JavaScript-map and 156 source-map baselines. Exact-source Fourslash passes
4,084 groups, skips 472 and fails none; all 4,620 named source-31 outcomes
are unchanged. The source-32 Go diagnostics and importer package tests pass,
as do the seven Rust diagnostics tests, importer cases, all 1,111 workspace
tests (four ignored), regression projects, WASM check, warnings-denied
Clippy, source lint, formatting and generator integrity. Workspace and
Fourslash needed escalated reruns only because the sandbox denied Unix-socket
binds.

No source-32 release performance measurement was collected under the user's
nonblocking migration policy. No production Rust or npm runtime code
changed; the staged catalogs are not embedded or loaded by tsrs. The measured
source-11 regressions remain disclosed above. Evidence is under
`target/port-623/0032-cb72b2e17b/`.

## 33. `f9f8d0129256` — fix formatter token-line comparison

Upstream `f9f8d01292562242b6e7c7142e46ed7e470926f9` corrects
`FormattingContext.TokensAreOnSameLine` to compare the start of the current
token with the **start** of the next token. A next token may extend over
multiple lines without changing whether the two token starts share a line.
The Rust formatter now makes that same comparison and retains the upstream
tri-state cache. Focused Rust regressions reproduce the source-32 failures
for template-head/middle spacing and an unterminated JSX attribute string,
then pass with the fix. The upstream commit also removes skips from two
template-formatting Fourslash tests and changes the JSX fixture's expected
spacing; those three exact source-33 Go Fourslash cases pass against the Rust
server. The ignored reference checkout advances to source 33; the published
TypeScript pin and base SDK manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets match source 32 with
no lost passes, new failures, crashes or timeouts. Go-history checker passes
13,521 diagnostics and 12,836 type/symbol baselines; canonical checker passes
13,521 diagnostics and 12,835 type/symbol baselines. Emit passes 13,525
diagnostics, 12,836 type/symbol, 13,451 JavaScript/declaration, 149
JavaScript-map and 156 source-map baselines. Exact-source Fourslash passes
4,086 groups, skips 470 and fails none; of all 4,620 named outcomes, only
`TestFormatNoSpaceAfterTemplateHeadAndMiddle` and
`TestFormatSpaceAfterTemplateHeadAndMiddle` change from skip to pass. The
source-33 Go formatter package, focused Rust formatter tests, full Rust
workspace, regression projects, warnings-denied Clippy, source lint,
formatting and generator integrity pass. Fourslash and workspace needed
escalated reruns only because the sandbox denied Unix-socket binds. The
capability table and language-server documentation now cite the measured
Fourslash counts.

No source-33 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0033-f9f8d01292/`.

## 34. `50d70a3f5f45` — stable inferred type-parameter order

Upstream `50d70a3f5f453a79a4323b263165da51f656a4e3` sorts the
`getInferTypeParameters` result with `CompareTypes` before the parameters are
used to construct conditional type mappers. The Rust function now uses its
existing `compare_types` comparator and Go-compatible `sort_func`, preserving
the upstream order without changing the local symbol table. The new
`stableTypeMapperOrdering` fixture passes for diagnostics, JavaScript,
`.types`, and `.symbols`; its result type retains `"m"` middleware in both
endpoint alternatives. It also passed before the Rust edit because the
source-33 local-symbol iteration happened to produce the expected order, so
the explicit sort is needed for algorithmic parity beyond that fixture. The
ignored reference checkout advances to source 34; the published TypeScript
pin and base SDK manifest remain unchanged until the full upgrade.

All existing cases retain their outcomes across the twelve complete checker
and emit pass/nonpass sets. The new fixture is added as a diagnostics,
type/symbol, and JavaScript pass and as a map-baseline skip. Go-history checker
passes 13,522
diagnostics and 12,837 type/symbol baselines; canonical checker passes 13,522
diagnostics and 12,836 type/symbol baselines. Emit passes 13,526 diagnostics,
12,837 type/symbol and 13,452 JavaScript/declaration baselines; the 149
JavaScript-map and 156 source-map passes are unchanged. No prior pass was lost
and no new failure, crash, or timeout appeared. Exact-source Fourslash passes
4,086 groups, skips 470, fails none, and all 4,620 named outcomes match
source 33. The source-34 Go checker package, 1,113 Rust workspace tests (four
ignored), regression projects, WASM check, warnings-denied Clippy, source
lint, formatting, and all seven generator checks pass. Fourslash and workspace
used the previously required Unix-socket sandbox escalation.

No source-34 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0034-50d70a3f5f/`.

## 35. `6b3da828d7f5` — remove legacy localization handbacks

Upstream `6b3da828d7f59669ba3b26e9a466b7bde9a5f6d4` deletes only the 13
legacy `tools/loc/*/diagnosticMessages.generated.json.lcl` handbacks. tsrs
never tracked `.lcl` files or referenced that format, so there is no source
asset or runtime behavior to change. The 13 source-32 imported JSON catalogs
and their 13 matching gzip archives are a separate, validated data format and
remain in place. The existing `--locale` gap is unchanged: tsrs still emits
English messages. The ignored reference checkout advances to source 35; the
published TypeScript pin and base SDK manifest stay unchanged until the full
upgrade.

All twelve complete checker and emit pass/nonpass sets match source 34 exactly.
Go-history checker passes 13,522 diagnostics and 12,837 type/symbol baselines;
canonical checker passes 13,522 diagnostics and 12,836 type/symbol baselines.
Emit passes 13,526 diagnostics, 12,837 type/symbol, 13,452 JavaScript/declaration,
149 JavaScript-map, and 156 source-map baselines. No passing case was lost and
no new failure, crash, or timeout appeared. Exact-source Fourslash passes
4,086 groups, skips 470, fails none, and all 4,620 named outcomes match source
34. The source audit confirms zero tsrs `.lcl` files and references. Locale
catalog validation, importer unit cases, seven Rust diagnostics tests, the dev
build, warnings-denied Clippy, source lint, formatting, and generator integrity
pass. The Fourslash subprocess gate used the previously required Unix-socket
sandbox escalation.

No source-35 release performance measurement was collected under the user's
nonblocking migration policy. This commit changes documentation only and
leaves production code and shipped locale data untouched. The measured
source-11 regressions remain disclosed above. Evidence is under
`target/port-623/0035-6b3da828d7/`.

## 36. `bcd7c422559d` — retain the idle cache clean timer for session shutdown

Upstream `bcd7c422559da26b7e60a46a41cf5503a360d106` stores the newly
scheduled idle cache clean timer on `Session` so `cancelIdleCacheClean` and
`closeIdleCacheClean` can stop it. Without that handle, closing a session may
wait for the full 30-second delay. tsrs already stores the timer in
`idle_cache_clean.timer` from the source-22 session port. Cancellation removes
and stops that handle; close also waits for any callback already in flight. The
matching Rust regression opens a file to schedule the timer and verifies that
`Session::close` returns within five seconds. This source commit needs no
production change and does not change any user-visible capability. The ignored
reference checkout advances to source 36; the published TypeScript pin and
base SDK manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets match source 35 exactly,
with no lost passes, new failures, crashes or timeouts. Go-history checker
passes 13,522 diagnostics and 12,837 type/symbol baselines; canonical checker
passes 13,522 diagnostics and 12,836 type/symbol baselines. Emit passes 13,526
diagnostics, 12,837 type/symbol, 13,452 JavaScript/declaration, 149
JavaScript-map and 156 source-map baselines. The exact upstream Go timer
regression, the full exact-source Go project package, and its Rust counterpart
pass, as do all 37 CLI regression projects.

Exact-source Fourslash passes 4,086 groups, skips 470, fails none, and all
4,620 named outcomes match source 35. The full Rust workspace passes 1,114
tests with four existing ignored, including the new timer case. The first
sandboxed workspace run could not bind a Unix socket; an escalated run made
that case pass but overlapped the 14-worker emit gate and saw one overflow in
the FSEvents all-core stress test. Both the isolated stress test and the
serial, escalated full workspace pass. The dev CLI/testrunner and fresh
Fourslash server builds, warnings-denied Clippy, source lint, formatting and
generator integrity pass.

No source-36 release performance measurement was collected under the user's
nonblocking migration policy. This port changes a test and documentation only;
the source-22 production implementation is untouched. The measured source-11
regressions remain disclosed above. Evidence is under
`target/port-623/0036-bcd7c42255/`.

## 37. `ca197b7b8586` — restart imports after a lower node_modules depth

Upstream `ca197b7b8586900c3b75c0365fac31cfcaaacca7` changes
`filesParser.start` to revisit a file's imports when a later path reaches that
file at a lower `node_modules` depth, even when its subtask list was already
started. The Rust parser now computes the same `lowered` condition and queues
the existing subtasks again at the file's new lowest depth. A symlinked package
first reached through a package import and then relatively no longer marks its
transitive `util.ts` as an external-library file, allowing it to be emitted.
The dedicated Rust virtual-filesystem regression puts the package-importing
root before the relative-importing root; its external-library assertion failed
before the change, while both classification and emit eligibility pass
afterward. The new exact-source Go conformance fixture and its Rust diagnostic
and JavaScript baselines also pass. The upstream fixture passed before the Rust
change under Rust's usual root order,
which is why the reversed-order regression is retained. The ignored reference
checkout advances to source 37; the published TypeScript pin and base SDK
manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets retain their source-36
outcomes. The new fixture adds one diagnostics and one JavaScript pass, with no
lost passing case or new failure, crash or timeout. Go-history checker passes
13,523 diagnostics and 12,837 type/symbol baselines; canonical checker passes
13,523 diagnostics and 12,836 type/symbol baselines. Emit passes 13,527
diagnostics, 12,837 type/symbol, 13,453 JavaScript/declaration, 149
JavaScript-map and 156 source-map baselines. Exact-source Fourslash passes
4,086 groups, skips 470, fails none, and all 4,620 named outcomes match source
36. The source-37 Go fixture, focused Rust regression, full Rust workspace
(1,115 passed, four existing ignored), CLI regression projects, WebAssembly
check, dev builds, warnings-denied Clippy, source lint and formatting pass.
All seven generators leave the preexisting crate diff byte-identical; the
generator script's clean-tree check is run after the corresponding commit.
Fourslash and the workspace socket test used the previously required Unix
socket sandbox escalation. The capability table's cited diagnostic and
JavaScript pass counts are updated.

No source-37 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0037-ca197b7b85/`.

## 38. `a1ef42b9ea70` — stabilize diagnostics for JS constructor properties

Upstream `a1ef42b9ea7032fa60df127d42b4c86fd2a110ee` eagerly resolves the
types of a JavaScript class's own constructor-declared `this` properties after
checking its members. The same step runs for class declarations and deferred
class expressions; inherited, typed, and method-only properties are excluded.
The Rust checker mirrors both call sites and the constructor-parent condition.
This makes implicit-any diagnostics on properties initialized only to
`undefined` or `null` independent of later symbol access. The tracked
`js-constructor-property-diagnostics` regression covers both class forms and
an annotated property: source-37 tsrs exited without diagnostics, while the
exact-source Go compiler and source-38 tsrs both report the same two TS7008
diagnostics. All four variants of upstream
`flakyDiagnosticsJsClassPropertyInitializedToUndefined` pass their diagnostic,
type, symbol, and JavaScript baselines. The ignored reference checkout advances
to source 38; the published TypeScript pin and base SDK manifest remain
unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets retain their source-37
outcomes. The new fixture adds four passes in each applicable baseline, with no
lost passing case, new failure, crash, or timeout. Go-history checker passes
13,527 diagnostics and 12,841 type/symbol baselines; canonical checker passes
13,527 diagnostics and 12,840 type/symbol baselines. Emit passes 13,531
diagnostics, 12,841 type/symbol, 13,457 JavaScript/declaration, 149
JavaScript-map, and 156 source-map baselines. Exact-source Fourslash passes
4,086 groups, skips 470, fails none, and all 4,620 named outcomes match source
37. The exact-source Go fixture, full Rust workspace (1,120 passed, four
existing ignored), all 38 CLI regression projects, WebAssembly check, dev
builds, warnings-denied Clippy, source lint, formatting, and generator output
comparison pass. The Fourslash Unix-socket test needed its previously required
sandbox escalation. The compatibility table's source revision and cited
diagnostic, type/symbol, and JavaScript pass counts are updated; the root and
npm README summaries contain no corresponding numbers.

No source-38 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0038-a1ef42b9ea/`.

## 39. `fc636a691f1d` — await async API disposal and suggest `await using`

Upstream `fc636a691f1dbdbc9ba678911891a9e85ec283bf` makes async
`Snapshot`, `Program`, and `BuildOrchestrator` expose
`[Symbol.asyncDispose](): Promise<void>` and return the `dispose()` Promise.
The corresponding npm SDK methods now match upstream byte for byte, while
their sync counterparts call synchronous `dispose()` directly. The two
upstream API test files are ported with only the established npm package-name
substitution; their `await using` and `using` cases run against the tsrs
server. The Rust checker also mirrors the upstream `using` assignment check:
when the initializer lacks `[Symbol.dispose]` but is assignable to
`AsyncDisposable | null | undefined`, it appends TS18116 suggesting
`await using`. The English catalog is regenerated from this source revision;
its exact-count test advances from 2,220 to 2,221 messages. The ignored
reference checkout advances to source 39. The published TypeScript pin and
base SDK manifest remain unchanged until the full upgrade.

The new `usingAsyncDisposable` fixture exposed the missing suggestion before
the change and passes after it; the exact-source Go fixture passes. All twelve
complete checker and emit pass/nonpass sets retain their source-38 outcomes
with no removed passes or changed nonpassing cases. The one new fixture adds
one diagnostics, type, symbol, and JavaScript pass in each applicable mode.
Go-history checker passes 13,528 diagnostics and 12,842 type/symbol baselines;
canonical checker passes 13,528 diagnostics and 12,841 type/symbol baselines.
Emit passes 13,532 diagnostics, 12,842 type/symbol, 13,458
JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines.
Exact-source Fourslash passes 4,086 groups, skips 470, fails none, and all
4,620 named outcomes match source 38. The changed sync/async npm API files
pass 721/721 tests, and the complete npm SDK suite passes 953/953. Full Rust
workspace all-targets passes 1,120 tests with four existing ignored; all 38
CLI regression projects, WebAssembly check, dev and npm SDK builds, SDK
source-condition typecheck, warnings-denied
Clippy, source lint, formatting, and seven generators pass. The generator
rerun leaves the tracked crate diff and untracked-file list byte-identical.
Fourslash and workspace Unix-socket tests required the previously observed
sandbox escalation. The capability table and Node API documentation record
the updated counts and async disposal behavior; no root or npm README
summary cites these counts.

No source-39 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0039-fc636a691f/`.

## 40. `de61e6962143` — update DOM and WebWorker typings

Upstream `de61e696214322f72d967bd17c58c50e44b2920d` updates bundled
`lib.dom.d.ts` and `lib.webworker.d.ts`, including CSS, DOM, Serial, and
WebTransport declarations. The Rust bundled copies match the source-40 Go
files byte for byte. The upstream commit also updates two type baselines;
`mappedTypeRecursiveInference` and `globalThisBlockscopedProperties` pass
their exact-source diagnostic, type, symbol, and JavaScript baselines. The
ignored reference checkout advances to source 40. The published TypeScript
pin and base SDK manifest remain unchanged until the full upgrade.

The new DOM declarations add 34 `string` references to the language-service
smoke fixture. Its one affected expected response was regenerated with a
Go compiler built from source 40; the Rust response matches the new Go JSON
byte for byte, and the other 38 responses remain unchanged. All twelve
complete checker and emit pass/nonpass sets retain their source-39 outcomes,
with no removed passes, new failures, crashes, or timeouts. Go-history checker
passes 13,528 diagnostics and 12,842 type/symbol baselines; canonical
checker passes 13,528 diagnostics and 12,841 type/symbol baselines. Emit
passes 13,532 diagnostics, 12,842 type/symbol, 13,458 JavaScript/declaration,
149 JavaScript-map, and 156 source-map baselines. Exact-source Fourslash
passes 4,086 groups, skips 470, fails none, and all 4,620 named outcomes
match source 39. Full Rust workspace all-targets, all 38 CLI regression
projects, WebAssembly check, dev builds, warnings-denied Clippy, source
lint, formatting, and all seven generators pass. The generator rerun leaves
the tracked crate diff byte-identical. Workspace and Fourslash Unix-socket
tests used the previously required sandbox escalation. The compatibility
table records the source revision and library update; no root or npm README
summary cites these counts.

No source-40 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0040-de61e69621/`.

## 41. `253bcd8623` — scope emit resolvers to one emit context and cache their node builders

Upstream `253bcd86231f2ceda21029eedf4c92c8b0d583f4` creates one emit
resolver and context per file, caches one node builder in that resolver, and
shares its declaration-visibility and JSX links through the checker. The Rust
checker now does the same for file emit and declaration diagnostics. Script
transforms, declaration transforms, and their deferred printers use the same
context and resolver for each file, resetting the context after both prints.
Declaration serialization no longer accepts a separate context, so its lazy
node builder cannot silently use a different factory. Language-service
completion, quick-info, reference, and signature-help paths also reuse a
context or builder at the scope introduced upstream. A focused compiler test
checks JS, declaration, and both map outputs across repeated emits with one
program. The ignored reference checkout advances to source 41. The published
TypeScript pin and base SDK manifest remain unchanged until the full upgrade.

The Go commit moves checker helper methods to `emitsupport.go` and removes its
checker-wide cached resolver. Rust keeps the existing helper methods and a
context-independent checker resolver for internal call sites; this avoids a
large mechanical module split while file emit always constructs a fresh
context-bound resolver. Rust's `EmitHost` still stores that per-file resolver,
because its scratch-region and deferred-printer architecture differs from Go's
host factory. The visibility/JSX caches remain checker-owned across those
per-file resolvers and scratch-keyed JSX links are cleared before the file's
scratch region is freed. These are Rust architectural deviations from the Go
patch, not intended output differences.

All twelve complete compiler checker and emit pass/nonpass sets retain their
source-40 outcomes, with no added or removed passes, failures, crashes, or
timeouts. Go-history checker passes 13,528 diagnostics and 12,842 type/symbol
baselines; canonical checker passes 13,528 diagnostics and 12,841 type/symbol
baselines. Emit passes 13,532 diagnostics, 12,842 type/symbol, 13,458
JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines.
Exact-source Fourslash passes 4,086 groups, skips 470, fails none, and all
4,620 named outcomes match source 40. The exact-source Go
`TestImportElision` changed by this commit passes. Full Rust workspace all-targets passes
1,121 tests with four existing ignored; all 38 CLI regression projects,
WebAssembly check, dev builds, warnings-denied Clippy, source lint,
formatting, and all seven generators pass. The generator rerun leaves tracked
crate changes and untracked crate files byte-identical. Workspace and
Fourslash Unix-socket tests used the previously required sandbox escalation.
The compatibility table records the source revision; the root and npm README
summaries contain no corresponding revision or counts.

Release performance was measured on public projects with one checker and
alternating source-40/source-41 release binaries. After one warm-up pair, VS
Code declaration-only emit (9,399 byte-identical output files, five measured
pairs) had median wall time 7.421 versus 7.418 seconds and median peak RSS
2,065,219,584 versus 2,068,709,376 bytes (+0.17%). Compiler emit (120
identical output files, 21 measured pairs) had 0.2655 versus 0.2672 seconds
and 109,936,640 versus 109,740,032 bytes (-0.18% RSS); Compiler-Unions emit
(105 identical output files, 21 measured pairs) had 0.3439 versus 0.3413
seconds and 111,230,976 versus 111,345,664 bytes (+0.10% RSS). All ordered
diagnostics and emitted bytes matched between revisions. The largest wall
difference was +0.63% on the short Compiler run, below the project's 2%
wall-time regression threshold and within timing noise. `bench/count.py`
reported retired instructions unavailable on macOS/arm64 (`no
perf_event_open`), so there is no instruction-count comparison. Evidence is under
`target/port-623/0041-253bcd8623/`.

## 42. `afd02f148639` — recognize typed JSDoc parameter tags as type annotations

Upstream `afd02f14863971d7a1b424f7c5a7e6a4eb2f92a7` adds
`JSDocParameterTag` to `ast.GetTypeAnnotationNode`. The Rust AST utility now
returns the tag's `typeExpression` through its existing `type_node()` accessor,
which is used by the language-service object-binding completion guard. A
focused parser test parses a typed `@param` tag, observes the accessor return
`None` before the port, and observes the same type-expression node as `type_node()`
after the port. This is a direct source-equivalent change without a Rust-specific
behavioral deviation. The ignored reference checkout advances to source 42;
the published TypeScript pin and base SDK manifest remain unchanged until the
full upgrade.

All twelve complete compiler checker and emit pass/nonpass sets retain their
source-41 outcomes, with no added or removed passes, failures, crashes, or
timeouts. Go-history checker passes 13,528 diagnostics and 12,842 type/symbol
baselines; canonical checker passes 13,528 diagnostics and 12,841 type/symbol
baselines. Emit passes 13,532 diagnostics, 12,842 type/symbol, 13,458
JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines.
Exact-source Fourslash passes 4,086 groups, skips 470, fails none, and all
4,620 named outcomes match source 41. Full Rust workspace all-targets passes
1,122 tests with four existing ignored; all 38 CLI regression projects,
WebAssembly check, dev builds, warnings-denied Clippy, source lint, formatting,
and all seven generators pass. The generator rerun leaves the tracked crate
diff and untracked crate files byte-identical. Workspace and Fourslash
Unix-socket tests required the previously observed sandbox escalation. The
compatibility table records the source revision and typed JSDoc completion
context; the root and npm README summaries contain no corresponding revision
or counts.

No source-42 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0042-afd02f1486/`.

## 43. `1a78786ca7` — ignore import-type qualifier names while marking emit references

Upstream `1a78786ca751559170857893fe3e01eba7c69997` excludes the
qualified names in `typeof import()` types from the emit resolver's
alias-reference walk. Rust applies the same guard in `mark_linked_references`
using its existing `is_import_type_qualifier_part` helper. Before the change,
the three new upstream compiler fixtures showed one incorrectly retained
import and two extra missing-name diagnostics added during emit; afterward,
all three pass their diagnostic, type, symbol, and JavaScript baselines. This
is a direct source-equivalent change without a Rust-specific behavioral
deviation. The ignored reference checkout advances to source 43. The
published TypeScript pin and base SDK manifest remain unchanged until the
full upgrade.

All twelve complete checker and emit pass/nonpass sets retain their source-42
outcomes, with exactly the three new upstream cases added to each applicable
passing set and no removed passes, new failures, crashes, or timeouts.
Go-history checker passes 13,531 diagnostics and 12,845 type/symbol baselines;
canonical checker passes 13,531 diagnostics and 12,844 type/symbol baselines.
Emit passes 13,535 diagnostics, 12,845 type/symbol, 13,461
JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines.
Exact-source Fourslash passes 4,086 groups, skips 470, fails none, and all
4,620 named outcomes match source 42. Full Rust workspace all-targets passes
1,122 tests with four existing ignored; all 38 CLI regression projects,
WebAssembly check, dev builds, warnings-denied Clippy, source lint,
formatting, and all seven generators pass. The generator rerun leaves the
tracked crate diff and untracked crate files byte-identical. Workspace and
Fourslash Unix-socket tests required the previously observed sandbox
escalation. The compatibility table records the source revision, new pass
counts, stable diagnostics, and import elision; the root and npm README
summaries contain no corresponding revision or counts.

No source-43 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0043-1a78786ca7/`.

## 44. `21b260b4e7` — exclude top-level imports from document symbols

Upstream `21b260b4e7e5123727f33d506fa686fecc258b1a` stops the document-symbol
visitor before descending into an import or import-equals declaration directly
under a source file. Rust adds the equivalent guard after collecting attached
JSDoc tags, so top-level imports disappear from outlines while their JSDoc
typedef and callback symbols remain. The new exact-source upstream Fourslash
case failed before the change with five extra import symbols and passed after
it; all three existing navigation baselines changed upstream by this commit
also pass against the new Go snapshots. This is a direct source-equivalent
change without a Rust-specific behavioral deviation. The ignored reference
checkout advances to source 44. The published TypeScript pin and base SDK
manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets are unchanged from
source 43, with no removed passes, new failures, crashes, or timeouts.
Go-history checker passes 13,531 diagnostics and 12,845 type/symbol baselines;
canonical checker passes 13,531 diagnostics and 12,844 type/symbol baselines.
Emit passes 13,535 diagnostics, 12,845 type/symbol, 13,461
JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines.
Exact-source Fourslash passes 4,087 of 4,557 groups, skips 470, fails none;
the only addition among its 4,621 named outcomes is the new upstream case,
and no existing outcome changes. Full Rust workspace all-targets passes
1,122 tests with four existing ignored; all 38 CLI regression projects,
WebAssembly check, dev builds, warnings-denied Clippy, source lint,
formatting, and all seven generators pass. The generator rerun leaves the
tracked crate diff and untracked crate files byte-identical. Workspace and
Fourslash Unix-socket tests required the previously observed sandbox
escalation. The compatibility table and LSP guide record the new Fourslash
count and document-symbol behavior; the root and npm README summaries contain
no corresponding source revision or count.

No source-44 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0044-21b260b4e7/`.

## 45. `c3f14c2fe8` — preserve object-literal parent during decorator metadata emit

Upstream `c3f14c2fe8971440d5a8bf1432d36bb26828c7c7` visits decorated object
literals with the literal as the current metadata parent, then restores the
enclosing parent. Rust now follows that control flow. This prevents a method or
accessor inside an object literal nested in a class from being treated as a
class member while visiting decorator metadata. The new exact-source compiler
fixture `decoratorMetadataObjectLiteralMethodNoCrash` passes its diagnostic,
type, symbol, and JavaScript baselines. It also passed before the Rust edit:
Rust's existing class-element guard already avoided the upstream Go crash, so
the new code closes a source-level parent-tracking gap without a measurable
baseline change on the fixture. The ignored reference checkout advances to
source 45. The published TypeScript pin and base SDK manifest remain unchanged
until the full upgrade.

All twelve complete checker and emit pass/nonpass sets preserve every source-44
pass and have no new failures, crashes, or timeouts. The new upstream fixture
adds one pass to each applicable diagnostic, type, symbol, and JavaScript set.
Go-history checker passes 13,532 diagnostics and 12,846 type/symbol baselines;
canonical checker passes 13,532 diagnostics and 12,845 type/symbol baselines.
Emit passes 13,536 diagnostics, 12,846 type/symbol, 13,462
JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines.
Exact-source Fourslash passes 4,087 of 4,557 groups, skips 470, fails none;
all 4,621 named outcomes are unchanged from source 44. Full Rust workspace
all-targets passes 1,122 tests with four existing ignored; all 38 CLI regression
projects, WebAssembly check, dev builds, warnings-denied Clippy, source lint,
and formatting pass. The generator rerun leaves the staged crate diff and
untracked crate files unchanged. Workspace and Fourslash Unix-socket tests
required the previously observed sandbox escalation. The compatibility table
records the source revision, new pass counts, and decorator metadata behavior;
the root and npm README summaries contain no corresponding revision or counts.

No source-45 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0045-c3f14c2fe8/`.

## 46. `09d4966578ae` — report reference diagnostics for configless API programs

Upstream `09d4966578aec90fdec11784559218392602a029` makes
`CreateDiagnosticAtReferenceSyntax` return no located diagnostic when an
API-created program has project references but no config file. Its caller then
reports an ordinary program diagnostic instead of dereferencing a missing
config file. Rust already had the equivalent `config.config_file?` guard and
caller fallback before this source commit; the new Rust API regression passed
against source 45 before the comment update. It now locks the upstream 6306
diagnostic and its absent file location (`pos` and `end` are `-1`). The exact
upstream Go API test also passes at source 46. The previous Rust comment
incorrectly described Go's pre-fix behavior and is now aligned with this
revision. The Rust API's pre-existing validation of relative reference paths
remains an intentional surface difference: it returns a structured client
error where Go can panic; absolute reference paths follow the new upstream
behavior. The ignored reference checkout advances to source 46; the published
TypeScript pin and base SDK manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets are identical to
source 45: no removed passes, new failures, crashes, or timeouts. Go-history
checker passes 13,532 diagnostics and 12,846 type/symbol baselines; canonical
checker passes 13,532 diagnostics and 12,845 type/symbol baselines. Emit passes
13,536 diagnostics, 12,846 type/symbol, 13,462 JavaScript/declaration, 149
JavaScript-map, and 156 source-map baselines. Exact-source Fourslash passes
4,087 of 4,557 groups, skips 470, fails none; all 4,621 named outcomes are
unchanged from source 45. The source-46 focused Rust and Go API tests, full
Rust workspace all-targets (1,123 passed, four existing ignored), dev builds,
all 38 CLI regression projects, WebAssembly check, warnings-denied Clippy,
source lint, formatting, and all seven generators pass. The generator rerun
leaves the staged crate diff and untracked crate files unchanged. Workspace
and Fourslash Unix-socket tests required the previously observed sandbox
escalation. The capability table records the source-46 API behavior; the root
and npm README summaries contain no corresponding revision or count.

No source-46 release performance measurement was collected under the user's
nonblocking migration policy. There is no production logic change in Rust;
the source edit corrects a comment and the new test guards existing behavior.
The measured source-11 regressions remain disclosed above. Evidence is under
`target/port-623/0046-09d4966578/`.

## 47. `d61a7d2359` — skip disk-layout import diagnostics for custom API resolutions

Upstream `d61a7d235906e78be9b111de0ba060e28e67ecfa` marks a static or
callback module resolution as custom and skips checker diagnostics that assume
the import's path follows the on-disk resolution layout. Rust adds the same
internal `ResolvedModule` flag in the shared API conversion and guards the
corresponding checker branch. The new Rust API regression covers both a static
entry and a callback with `rewriteRelativeImportExtensions`: before the port
it reported two spurious TS2876 diagnostics, one for each import; afterward it
reports none. The exact upstream Go API test passes. The marker is internal,
as in Go, and does not change the `resolveModuleName` response. There is no
Rust-specific behavior or callback deviation in this port. The ignored
reference checkout advances to source 47. The published TypeScript pin and
base SDK manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets are identical to
source 46: no removed passes, new failures, crashes, or timeouts. Go-history
checker passes 13,532 diagnostics and 12,846 type/symbol baselines; canonical
checker passes 13,532 diagnostics and 12,845 type/symbol baselines. Emit passes
13,536 diagnostics, 12,846 type/symbol, 13,462 JavaScript/declaration, 149
JavaScript-map, and 156 source-map baselines. Exact-source Fourslash passes
4,087 of 4,557 groups, skips 470, fails none; all 4,621 named outcomes are
unchanged from source 46. The full Rust workspace all-targets passes 1,124
tests with four existing ignored; all 38 CLI regression projects, WebAssembly
check, dev builds, warnings-denied Clippy, source lint, formatting, and all
seven generators pass. The generator rerun leaves the staged crate diff and
untracked crate files unchanged. Workspace and Fourslash Unix-socket tests
required the previously observed sandbox escalation. The capability table
and Node API guide record the new custom-resolution behavior; the root and
npm README summaries contain no corresponding revision or count.

No source-47 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0047-d61a7d2359/`.

## 48. `54f05ae22a` — keep malformed destructuring assignments from crashing emit

Upstream `54f05ae22afd5f0ec5ec3ae0333b03efa1156fe6` broadens three
transformer assertions: decorator array assignment elements accept any
expression, and class-field/decorator object assignment elements accept any
object-literal element. Rust mirrors those predicates. Parser recovery can
produce array expressions and object methods or accessors that are invalid
assignment targets; the normal visitor still needs to traverse them for
private-field, `super`, and decorator transforms. Against the source-48
fixtures before the Rust edit, all seven new variants crashed in emit. After
the edit, all seven match exact-source Go diagnostic and JavaScript baselines,
and the three variants with type/symbol baselines match those too. The four
variant cases explicitly disable type/symbol baselines upstream. The focused
Go compiler harness passes all seven variants. There is no Rust-specific
behavioral deviation. The ignored reference checkout advances to source 48;
the published TypeScript pin and base SDK manifest remain unchanged until
the full upgrade.

All twelve complete checker and emit pass/nonpass sets preserve every
source-47 pass with no new failure, crash, or timeout. The seven upstream
variants add seven diagnostic and JavaScript passes in each applicable mode,
and three type/symbol passes. Go-history checker passes 13,539 diagnostics
and 12,849 type/symbol baselines; canonical checker passes 13,539 diagnostics
and 12,848 type/symbol baselines. Emit passes 13,543 diagnostics, 12,849
type/symbol, 13,469 JavaScript/declaration, 149 JavaScript-map, and 156
source-map baselines. Exact-source Fourslash passes 4,087 of 4,557 groups,
skips 470, fails none; all 4,621 named outcomes are unchanged from source 47.
The full Rust workspace all-targets passes 1,124 tests with four existing
ignored; all 38 CLI regression projects, WebAssembly check, dev builds,
warnings-denied Clippy, source lint, and formatting pass. Workspace and
Fourslash Unix-socket checks required the previously observed sandbox
escalation. All seven generators pass and leave the staged crate diff
unchanged. The compatibility table records the new pass counts and
malformed-input emit behavior; the root and npm README summaries contain no
corresponding source revision or count.

No source-48 release performance measurement was collected under the user's
nonblocking migration policy. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0048-54f05ae22a/`.

## 49. `91521cf229` — prefilter compiler test files before reading them

Upstream `91521cf2299d54a40d46fcf500130d273ddfc947` adds a conservative
Go `test.run` filename filter before opening and expanding compiler fixtures.
The Rust runner now makes the corresponding early decision for its own
selection interface: a plain mixed-case `--filter` can reject an unrelated
filename because configuration descriptions are lowercase, and `--list`
can reject filenames with no matching stem or configuration prefix. Rust
`--filter` also accepts arbitrary substring or regex matches over a different
test ID shape, so lowercase, regex, escaped, and other ambiguous patterns
continue to read the file. This is the intentional runner-specific adaptation;
it preserves the selected test set rather than copying the Go heuristic onto
incompatible CLI syntax. The upstream Go filter test and three focused Rust
unit tests pass. Against the actual fixture corpus, source-48 and source-49
runner binaries select byte-identical IDs for the full 15,278-variant list,
eight filter/list cases, and the 2,000-case WebAssembly sample list. The
ignored reference checkout advances to source 49; the published TypeScript
pin and base SDK manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets are identical to
source 48: no removed passes, new failures, crashes, or timeouts. Go-history
checker passes 13,539 diagnostics and 12,849 type/symbol baselines; canonical
checker passes 13,539 diagnostics and 12,848 type/symbol baselines. Emit passes
13,543 diagnostics, 12,849 type/symbol, 13,469 JavaScript/declaration, 149
JavaScript-map, and 156 source-map baselines. Exact-source Fourslash passes
4,087 of 4,557 groups, skips 470, fails none; all 4,621 named outcomes are
unchanged from source 48. The full Rust workspace all-targets passes 1,127
tests with four existing ignored; all 38 CLI regression projects, WebAssembly
check, dev builds, warnings-denied Clippy, source lint, formatting, and all
seven generators pass. Workspace and Fourslash Unix-socket checks required
the previously observed sandbox escalation. No user-visible capability or
conformance count changed, so the compatibility table and README summaries
remain current.

For the dev runner's `list --suite all`, ten warm paired timing samples give
0.3162 -> 0.0316 s for `--filter flakyDiagnostic` and 0.2937 -> 0.0314 s for
the two-ID `--list`; unfiltered listing is 0.3204 -> 0.3146 s and the
lowercase fallback is 0.2904 -> 0.2918 s. The 2,000-case list selects the
same IDs in 0.2881 -> 0.0710 s (five warm samples). Production compiler code
does not change. No source-49 release instruction-count or peak-RSS measurement
was collected under the user's nonblocking migration policy. The measured
source-11 regressions remain disclosed above. Evidence is under
`target/port-623/0049-91521cf229/`.

## 50. `4529d07ab5` — expose API client modules for editor integrations

Upstream `4529d07ab5b7a2d2bbe0ef8b25edbed9f35b7b7f` adds a runtime
`unstable/vscode` API module loader to the TypeScript package and connects it
to its bundled VS Code extension's selected-SDK lifecycle. The Rust compiler
and Go test corpus do not change. The tsrs npm SDK copies upstream
`src/vscode/extensionApi.ts`, `moduleLoader.ts`, and `protocol.generated.ts`
byte for byte and exports `tsrs/unstable/vscode`. The loader takes the
selected package's manifest, resolves its real path and package name, checks
the full `typescript/unstable/*` specifier, and loads the module through that
package's own exports. It works with a selected `tsrs` package as well as
with a selected TypeScript package. Focused Node tests cover selected-package
isolation, module identity, export enforcement, invalid specifiers, and a
manifest without a name. The packed-package smoke test imports both API
clients through the loader, and its consumer type-checks the loader's public
declarations. This adds an SDK capability; the compatibility table, root
README, npm README, and Node API guide are updated.

tsrs does not bundle the upstream VS Code extension or build VSIX packages.
The upstream extension's selected-SDK event, API-pipe lifecycle, package
upgrade detection, restart prompt, client state handling, and VSIX packaging
therefore have no tsrs runtime counterpart. Its `ExtensionAPI` and
`TypeScriptSDK` interfaces are exported as upstream declaration types, but
no tsrs extension implements them. The loader itself does not guard Node's
module cache against an in-place package upgrade; upstream performs that
guard in its extension, which restarts the extension host when necessary.
The ignored reference checkout advances to source 50; the published
TypeScript pin and base SDK manifest remain unchanged until the full upgrade.

All twelve complete checker and emit pass/nonpass sets are identical to
source 49: no removed passes, new failures, crashes, or timeouts. Go-history
checker passes 13,539 diagnostics and 12,849 type/symbol baselines;
canonical checker passes 13,539 diagnostics and 12,848 type/symbol
baselines. Emit passes 13,543 diagnostics, 12,849 type/symbol, 13,469
JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines.
Exact-source Fourslash passes 4,087 of 4,557 groups, skips 470, fails none;
all 4,621 named outcomes are unchanged from source 49. The npm source suite
passes 956 tests, including the three new module-loader tests; the packed
package smoke test passes 25 steps, including selected-package module
loading and a declaration type-check. The full Rust workspace all-targets
passes 1,127 tests with four existing ignored; all 38 CLI regression projects,
WebAssembly check, dev builds,
warnings-denied Clippy, source lint, formatting, and all seven generators
pass. Workspace and Fourslash Unix-socket checks required the previously
observed sandbox escalation.

The Rust compiler and its bundled libraries do not change in source 50.
No source-50 release instruction-count or peak-RSS measurement was collected
under the user's nonblocking migration policy. The measured source-11
regressions remain disclosed above. Evidence is under
`target/port-623/0050-4529d07ab5/`.

## 51. `bea2e849e9` — require a namespace binding in deferred imports

Upstream `bea2e849e90b5ff2889f9e120e1e0fae5e17659b` parses a bare
`import defer "module"` as a deferred import clause and reports TS18117 because
it lacks a namespace binding. The Rust parser and checker now take the same
path, the plain-JavaScript grammar-error set retains TS18117 even without
`@ts-check`, and the default-import emit-helper predicate follows the Go
change. Diagnostic 18117 is generated from the new Go catalog entry; its
catalog-size assertion advances to 2,222. The dedicated CLI regression
`deferred-import-namespace-binding` checks bare deferred imports in TypeScript,
checked JavaScript, and unchecked JavaScript alongside valid namespace and
invalid named bindings. Its diagnostics and exit status match a Go compiler
built from this exact source commit. Before the port, both new upstream
`importDeferInvalidDefault` variants failed diagnostics and JavaScript emit;
afterward, both pass diagnostics, types, symbols, and emit.

All three full compiler gate modes pass. Source 51 replaces one former
`importDeferInvalidDefault` fixture with `commonjs` and `esnext` variants:
that one-for-two rename is the sole change across complete pass/nonpass sets.
Go-history checker passes 13,540 diagnostics and 12,850 type/symbol baselines;
canonical checker passes 13,540 diagnostics and 12,849 type/symbol baselines.
Emit passes 13,544 diagnostics, 12,850 type/symbol, 13,470 JavaScript,
149 JavaScript-map, and 156 source-map baselines. Exact-source Fourslash
passes 4,087 of 4,557 groups, skips 470, fails none, and all 4,621 named
outcomes are unchanged from source 50. The full Rust workspace passes 1,127
tests with four existing ignored; all 39 CLI regressions, WebAssembly check,
dev builds, warnings-denied Clippy, source lint, formatting, and all seven
generators pass. Fourslash and workspace Unix-socket tests required the
previously observed sandbox escalation.

The ignored reference checkout advances to source 51; the published
TypeScript pin remains unchanged until the full upgrade. No source-51
release instruction-count or peak-RSS measurement was collected under the
user's nonblocking migration policy. The measured source-11 regressions
remain disclosed above. Evidence is under `target/port-623/0051-bea2e849e9/`.

## 52. `0ba5d992db` — preserve duplicate private declarations during decorated-class emit

Upstream `0ba5d992db957fa444c3049e51243a056d3a9276` changes two
class-field declaration transforms to resolve a private name in the current
class environment, then leave a declaration unchanged when its private-name
entry is `Untransformed` or invalid. The Rust port mirrors those two lookups
and guards. The expression-use lookup still searches enclosing classes and
intentionally hides `Untransformed` entries; using it at declaration sites
caused a panic on the upstream fixture's ES2022 variant. Before this port,
that variant crashed at “Undeclared private name for property declaration”;
the ES2015 and ESNext variants passed. Afterward, all three pass diagnostics
and JavaScript emit. A Go compiler built from this exact commit and the Rust
CLI produce byte-identical emitted JavaScript, identical 12 diagnostics, and
the same exit code for each target. The compatibility table records this
closed emit gap; the root and npm README capability summaries do not change.

All three full compiler modes have exactly the source-51 pass/nonpass sets:
no removed passes, new failures, crashes, or timeouts. Go-history checker
passes 13,540 diagnostics and 12,850 type/symbol baselines; canonical checker
passes 13,540 diagnostics and 12,849 type/symbol baselines. Emit passes
13,544 diagnostics, 12,850 type/symbol, 13,470 JavaScript, 149 JavaScript-map,
and 156 source-map baselines. Exact-source Fourslash passes 4,087 of 4,557
groups, skips 470, fails none, and all 4,621 named outcomes are unchanged.
The full Rust workspace passes 1,127 tests with four existing ignored; all
39 CLI regressions, WebAssembly check, dev builds, warnings-denied Clippy,
source lint, formatting, and all seven generators pass. Fourslash and
workspace Unix-socket tests required the previously observed sandbox
escalation.

The ignored reference checkout advances to source 52; the published
TypeScript pin remains unchanged until the full upgrade. No source-52
release instruction-count or peak-RSS measurement was collected under the
user's nonblocking migration policy. The measured source-11 regressions
remain disclosed above. Evidence is under `target/port-623/0052-0ba5d992db/`.

## 53. `0bd74f77b2` — avoid CommonJS exports for shadowed names in function-local loops

Upstream `0bd74f77b276e659df9f63c3c63377cf11d1d86a` changes the
iteration-body visitors for the ordinary CommonJS `for`, `for..in`, and
`for..of` transforms from the top-level nested visitor to the normal
visitor. The Rust port mirrors those two calls. Before the port, the new
`commonjsExportShadowedInFunctionLoop` fixture passed diagnostics, types,
and symbols but emitted three spurious `exports.mount = mount` assignments
for local variables shadowing an exported function. Afterward, the fixture
passes JavaScript emit while file-level loops still assign to exports for
hoisted `var` declarations. A Go compiler built from this exact commit and
the Rust CLI emit byte-identical JavaScript with matching diagnostics and
exit status for the fixture.

All three full compiler modes pass. The new fixture is the only pass-set
addition from source 52, including diagnostics, types, symbols, and
JavaScript; no prior pass was removed and there are no new failures,
crashes, or timeouts. Go-history checker passes 13,541 diagnostics and
12,851 type/symbol baselines; canonical checker passes 13,541 diagnostics
and 12,850 type/symbol baselines. Emit passes 13,545 diagnostics, 12,851
type/symbol, 13,471 JavaScript, 149 JavaScript-map, and 156 source-map
baselines. Exact-source Fourslash passes 4,087 of 4,557 groups, skips 470,
fails none, and all 4,621 named outcomes are unchanged. The full Rust
workspace, all 39 CLI regressions, WebAssembly check, dev builds,
warnings-denied Clippy, source lint, and formatting pass. The workspace and
Fourslash Unix-socket checks required the previously observed sandbox
escalation. The generator check ran after the commit because it compares
its output against the whole working-tree diff.

The ignored reference checkout advances to source 53; the published
TypeScript pin remains unchanged until the full upgrade. No source-53
release instruction-count or peak-RSS measurement was collected under the
user's nonblocking migration policy. The measured source-11 regressions
remain disclosed above. Evidence is under `target/port-623/0053-0bd74f77b2/`.

## 54. `9055b58852` — refresh the inferred project after project-tree loading

Upstream `9055b588528600681abb8d95b3f1f39ef1707e4a` cleans the inferred
project after configured project trees are loaded, then updates its program
when it exists. The Rust port mirrors both steps in
`ProjectCollectionBuilder::did_request_project_trees`. The exact upstream
`TestWorkspaceSymbolNewInferredProject` fails against the pre-port Rust
server: after editing `a.ts` so it no longer imports open `e.ts`,
`workspace/symbol` cannot find `e` in the new inferred project. That first
assertion stops the pre-port test before its later crash case. With the port,
the same test passes both the symbol lookup and the subsequent request after
opening `b.ts`, which exercises the configured project's fast path. The
upstream Fourslash test is run unchanged through the Go subprocess runner;
there is no Rust-only test translation or protocol change.

All three full compiler modes have exactly the source-53 pass and nonpass
sets, with no removed passes, new failures, crashes, or timeouts. Go-history
checker passes 13,541 diagnostics and 12,851 type/symbol baselines;
canonical checker passes 13,541 diagnostics and 12,850 type/symbol baselines.
Emit passes 13,545 diagnostics, 12,851 type/symbol, 13,471 JavaScript,
149 JavaScript-map, and 156 source-map baselines. Exact-source Fourslash
passes 4,088 of 4,558 groups, skips 470, fails none, and adds only the new
upstream test to the 4,621 prior named outcomes; all prior outcomes are
unchanged. The full Rust workspace passes with four existing ignored tests;
all 39 CLI regressions, WebAssembly check, dev builds, warnings-denied
Clippy, source lint, and formatting pass. Fourslash and workspace socket
tests used the previously observed sandbox escalation.

The compatibility and language-server pages record the new Fourslash count
and closed workspace-symbol gap; the root and npm README summaries have no
matching count or capability distinction. The ignored reference checkout
advances to source 54; the published TypeScript pin remains unchanged until
the full upgrade. No source-54 release instruction-count or peak-RSS
measurement was collected under the user's nonblocking migration policy.
The measured source-11 regressions remain disclosed above. Evidence is under
`target/port-623/0054-9055b58852/`.

## 55. `a501b11f64` — index re-exporting modules for declaration emit

Upstream `a501b11f64592a18287664ecc478a09659ad3134` adds a checker-local
index of external modules by the resolved targets of their exports, preserving
program order and including a symbol's parent module even when it is not
exported. The Rust checker already has the equivalent reverse lookup in
`modules_exporting`, backed by `ModuleExportIndex` and
`get_symbol_target_for_alias_lookup`. It considers named, aliased, star, and
`export =` entries and inserts the parent module in program order. Its
per-checker index summarizes only final export tables, checks table sizes,
and falls back to the original scan during alias resolution and for modules
not yet summarized; fresh checkers begin with empty indexes. These guards
also cover reentrancy and invalidation where the Go change uses an incomplete
index fallback. The upstream `declarationEmitAlternativeContainingModules`
fixture passed diagnostics, types, symbols, and declaration output against
the unmodified Rust implementation, matching the exact-source Go baselines.
No redundant runtime cache or refactor was added. This is an evidence-only
port with the compatibility counts updated for the new fixture.

All three full compiler modes pass. Compared with source 54, the new fixture
is the sole pass-set addition (diagnostics, types, symbols, and JavaScript/
declaration output); all prior pass and nonpass sets remain unchanged, with
no new failures, crashes, or timeouts. Go-history checker passes 13,542
diagnostic and 12,852 type/symbol baselines; canonical checker passes 13,542
diagnostic and 12,851 type/symbol baselines. Emit passes 13,546 diagnostic,
12,852 type/symbol, 13,472 JavaScript/declaration, 149 JavaScript-map, and
156 source-map baselines. Exact-source Fourslash passes 4,088 of 4,558 groups,
skips 470, fails none, and all 4,622 named outcomes match source 54. The full
Rust workspace, all 39 CLI regressions, WebAssembly check, dev builds,
warnings-denied Clippy, source lint, formatting, and generator check pass.
The Fourslash and workspace socket checks required the previously observed
sandbox escalation.

The ignored reference checkout advances to source 55; the published
TypeScript pin remains unchanged until the full upgrade. Runtime source,
Cargo inputs, and bundled assets are unchanged from source 54, so this port
does not change the Rust compiler binary's behavior or performance. No
separate source-55 release performance batch was collected under the user's
nonblocking migration policy; retired-instruction collection is unavailable
on this macOS host. The measured source-11 regressions remain disclosed
above. Evidence is under `target/port-623/0055-a501b11f64/`.

## 56. `ec47d33c23` — check computed enum names despite grammar errors

Upstream `ec47d33c23e464a17cdf2475632cba629bee8763` checks the
expression in an enum member's computed name even though the computed name
itself is a grammar error. It also removes the special case that suppressed
reference resolution inside those names. The Rust port makes the same two
checker changes. Before the port, eight of ten new enum fixtures failed
their diagnostic baselines, one differed by error codes, and four existing
`parserComputedPropertyName` variants failed the revised baselines. After
the port, all ten new enum fixtures and all 41 parser variants match their
exact-source diagnostics, type/symbol baselines, and JavaScript output.
The exact upstream Go `TestLocal` runner passes all ten new fixtures and
all five changed parser fixture files, including their diagnostic and emit
subchecks.

All three full compiler modes pass. Compared with source 55, each gains
exactly the ten new fixtures and retains every prior passing and nonpassing
outcome, with no new failures, crashes, or timeouts. Go-history checker
passes 13,552 diagnostic and 12,862 type/symbol baselines; canonical
checker passes 13,552 diagnostic and 12,861 type/symbol baselines. Emit
passes 13,556 diagnostic, 12,862 type/symbol, 13,482 JavaScript/declaration,
149 JavaScript-map, and 156 source-map baselines. Exact-source Fourslash
passes 4,088 of 4,558 groups, skips 470, fails none, and preserves all
4,622 named source-55 outcomes unchanged. The full Rust workspace, all 39
CLI regressions, WebAssembly check, dev builds, warnings-denied Clippy,
source lint, and formatting pass. The Fourslash and workspace socket tests
required the previously observed sandbox escalation. The generator check
runs after the commit because it compares its output against the whole
working-tree diff.

The compatibility table records the new counts and enum diagnostic
behavior; neither README has a matching numerical summary to update.
The ignored reference checkout advances to source 56; the published
TypeScript pin remains unchanged until the full upgrade. No source-56
release instruction-count or peak-RSS measurement was collected under the
user's nonblocking migration policy. The measured source-11 regressions
remain disclosed above. Evidence is under
`target/port-623/0056-ec47d33c23/`.

## 57. `fed0bf2414` — attach hover documentation to its function

Upstream `fed0bf24149fb1ed36039212648bafdafc1ea10e` moves the
`getDocumentationForSymbol` docstring from above the
`documentationLocationMapper` type to the function it describes. The Rust
port makes the same comment move in `hover.rs` and updates the Go line
references for the mapper type and method. Rust groups the method in the
`LanguageService` implementation before the type alias, so their order
differs from Go; the docstring now directly precedes the corresponding Rust
function. No executable logic or public capability changes.

The exact upstream diff changes only `tsc/internal/ls/hover.go`; removing
whole-line comments gives byte-identical source before and after. The Rust
diff likewise changes only `hover.rs`, whose noncomment lines are
byte-identical. No compiler or Fourslash fixture, baseline, generated asset,
or runtime input changed. Thus the source-56 full compiler conformance sets
and exact-source Fourslash outcomes apply without rerunning those unchanged
inputs: Go-history checker 13,552 diagnostics and 12,862 type/symbol
baselines; canonical checker 13,552 diagnostics and 12,861 type/symbol
baselines; emit 13,556 diagnostics, 12,862 type/symbol, 13,482
JavaScript/declaration, 149 JavaScript-map, and 156 source-map baselines;
Fourslash 4,088 passing groups of 4,558, 470 skipped, no failures, and all
4,622 named outcomes unchanged. The source-56 full pass/nonpass comparisons
had no lost passes, new failures, crashes, or timeouts. The identity hashes
and both patches are retained in `identity.json`, `upstream.patch`, and
`rust.patch` under the source-57 evidence directory.

The dev CLI, test-runner and Fourslash-server builds, 80 language-server
unit tests, all 39 CLI regressions, WebAssembly check, warnings-denied
Clippy, source lint, formatting, and generator check pass. The ignored
reference checkout advances to source 57; the published TypeScript pin
remains unchanged until the full upgrade. No source-57 release
instruction-count or peak-RSS measurement was collected under the user's
nonblocking migration policy. Because only comments changed, no runtime
performance change is expected. The measured source-11 regressions remain
disclosed above. Evidence is under `target/port-623/0057-fed0bf2414/`.

## 58. `c0ca4744c9` — prepare the JavaScript API for beta

Upstream `c0ca4744c917430282a7de06c3e8f22d2cb87353` promotes the TypeScript 7
SDK subpaths from `unstable/*` to `sync`, `async`, `fs`, `path`, `proto`, `ast/*`, and
`vscode`. It removes deprecated snapshot and project aliases, moves
`formatNodeForInsertion` to the project language service, exposes `isRemoteNode`
and `API.debug`, strips internal declarations, and adds a declaration-checking
configuration. The tsrs package exports the matching stable subpaths and syncs
all 192 recorded SDK files, tests, generated protocol and AST sources at this
commit. The Rust project response drops the duplicate `rootFiles` and
`compilerOptions` fields; its 177-method inventory places
`formatNodeForInsertion` in the upstream protocol order. SDK, package-consumer,
and Node parity tests use the promoted names and replacement APIs. Recorded
Node goldens were refreshed against the exact source-58 Go oracle, including
its current AST kind numbers and source-map serialization.

The SDK source manifest now records source 58, and the sync tool accepts an
explicit `--source-commit` so the copied SDK can advance during the migration.
The `Cargo.toml` TypeScript pin and npm package version remain at their
published base until the final source commit. The source-58 VS Code extension
edits have no counterpart here because tsrs ships the `vscode` SDK helper,
not the upstream extension. The pre-existing tsrs async connection-loss fix
and vendored JSON-RPC adjustments remain explicit sync patches. These are the
deliberate differences from the selected SDK source.

All three full compiler modes retain the exact source-57 passing and
nonpassing sets, without new failures, crashes, or timeouts. Go-history
checker passes 13,552 diagnostic and 12,862 type/symbol baselines; canonical
checker passes 13,552 diagnostic and 12,861 type/symbol baselines. Emit passes
13,556 diagnostic, 12,862 type/symbol, 13,482 JavaScript/declaration, 149
JavaScript-map, and 156 source-map baselines. Exact-source Fourslash passes
4,088 of 4,558 groups, skips 470, fails none, and preserves all 4,622 named
source-57 outcomes. The SDK source suite passes all 956 tests. The packed npm
consumer completes all 25 smoke steps and the separate Node consumer passes.
The full Rust workspace, 39 CLI regressions, WebAssembly check, dev builds,
warnings-denied Clippy, source lint, formatting, SDK sync, and generated-code
checks pass. Exact-source Go API and protocol-generator tests pass. The
Fourslash and workspace socket checks used the previously observed sandbox
escalation.

In the additional Node API parity lane, both the exact source-58 Go oracle
and tsrs pass 48 of 55 tests, skip four, mark two TODO, and fail the same
async checker test on the upstream `signatureToSignatureDeclarationHelper`
panic. Rust's lone-surrogate TODO succeeds where the Go oracle reports the
known server exit. The three existing soft differences (CPU profiling and
two large-declared-payload transport cases) remain documented in the parity
summary. This lane identifies no new tsrs regression from source 58; the
upstream panic remains a known gap.

No source-58 release instruction-count, wall-time, or peak-RSS comparison was
collected under the user's nonblocking migration policy. The compiler
conformance sets are unchanged, but the SDK and API response do change; no
performance-equality claim follows. The measured source-11 regressions
remain disclosed above. Evidence is under
`target/port-623/0058-c0ca4744c9/`, with Node parity summaries under
`tools/node-api/.work/source58-oracle-parity/` and
`tools/node-api/.work/source58-parity-refreshed/`.

## 59. `6ad8c56f9b` — shared data symbols

Upstream `6ad8c56f9b5a9bb910046c56059296311adc24ba` makes symbol fields
private, routes their use through accessors, and gives instantiated symbols
the original symbol's live data (name, declarations, value declaration,
members, exports, parent, and export symbol). Flags, check flags, and identity
remain independent. The Rust symbol already had accessors and a compact
optional table tail, so the mechanical Go accessor rewrites do not require
matching Rust churn. The port switches the checker to a shared-data
constructor and converts remaining direct Rust name reads to `name()`.

Rust keeps its 32-byte compressed/40-byte plain `Symbol` instead of adopting
Go's separate `Symbol`/`SymbolWithData` types. An instantiated symbol stores
its data owner's arena key in its existing `parent_or_tables` word and a
reserved zero name word marks the shared-data representation. Nested instances
flatten to that one owner when constructed. Data accessors resolve the owner;
the marker is independent of mutable check flags, so even clearing
all of an instance's check flags cannot detach its data. Ordinary symbols
retain their prior packed name, declaration slice, and optional table tail.
The checker still counts instantiated symbols and assigns separate IDs in
seed mode; `clone_symbol` continues to copy its member and export tables.
The focused Rust unit test checks live two-way sharing, lazy table creation,
independent flags and IDs, and the check-flag overwrite case. The exact-source
Go symbol and API symbol-response tests pass. There is no new upstream
conformance fixture or user-visible capability change.

All three full compiler modes preserve the exact source-58 passing and
nonpassing sets, with no new failure, crash, or timeout. Go-history checker
passes 13,552 diagnostic and 12,862 type/symbol baselines; canonical checker
passes 13,552 diagnostic and 12,861 type/symbol baselines. Emit passes 13,556
diagnostic, 12,862 type/symbol, 13,482 JavaScript/declaration, 149
JavaScript-map, and 156 source-map baselines. Exact-source Fourslash passes
4,088 of 4,558 groups, skips 470, fails none, and preserves all 4,622 named
source-58 outcomes. The full Rust workspace, 39 CLI regressions, WebAssembly
check, dev builds, warnings-denied Clippy, source lint, formatting, and
generator check pass. The Fourslash and workspace socket checks used the
previously observed sandbox escalation.

The ignored reference checkout advances to source 59. The published
TypeScript pin and compatibility summaries remain unchanged; this source
changes internal storage without changing feature status or pass counts.

Paired release runs against the source-58 binary on the same Apple M3 Max
preserve diagnostics and exit codes in every public-project cell. Ten
interleaved runs per binary for xstate-main and webpack, and three for the
larger VS Code and MUI Docs projects, give these median wall-time changes:

| Project | 1 checker | 4 checkers | Largest peak-RSS change |
| --- | ---: | ---: | ---: |
| xstate-main | +2.6% | +1.2% | +0.3% |
| webpack | +1.4% | +1.4% | +0.1% |
| vscode | +2.4% | +1.8% | +0.2% |
| mui-docs | +3.9% | +0.1% | +0.1% |

The two smaller projects also pass the 16-checker poisoned-arena runs. The
one-hop data-owner lookup avoids repeated traversal of nested instances, but
the ordinary-symbol marker check still adds work: the measured wall-time
increase is disclosed under the user's nonblocking migration policy. Peak
memory is effectively unchanged because Rust already used a packed symbol.
`bench/count.py` cannot collect retired instructions on this macOS/arm64
host (`no perf_event_open on darwin/arm64`); the single-threaded peak-RSS
runs and diagnostic comparisons still pass. The paired logs and JSON are
under `target/port-623/0059-6ad8c56f9b/verify-optimized/` and
`verify-large/`.

## 60. `aad4c72bf2` — sync connection answers nested requests in stack order

Upstream `aad4c72bf2d22e1fddb2a07d9e6f131e9b2ade44` adds a call-depth turn
condition to `ipc.SyncConn`. A client callback can re-enter the API while an
older nested request handler is still running; concurrent server-to-client
calls may then be stacked. The older handler must wait until deeper calls
finish before writing its response, and a notification handler may itself
make a nested call. The Rust port puts `calls` and `reading` beside the
reader/writer in `SyncIo`, uses a condition variable for `lock_turn`, and
reserves the protocol reader for the deepest active call. Every request
response, including timing and recovered handler errors, waits for its turn.
An `ActiveCall` guard restores the call state and wakes waiters on success,
error, or unwind. This is the same ordering as Go despite Rust's separate
protocol reader and writer. No SDK source changes were needed: upstream
changed only the Go IPC connection. The published sync SDK uses MessagePack;
the JSON-RPC sync-connection notification path is covered at the transport
level.

The two new Rust pipe tests force a shallower handler to finish while a deeper
call is active and check that responses arrive deepest first, and verify that
a notification handler can call the client. Both pass. The exact-source Go
`TestSyncConn` cases pass. Source-mode Node API tests pass 956/956, the pinned
Node transport round trip passes 15/15, and the packed npm consumer passes
25/25 steps. All three full compiler modes have result lists byte-identical
to source 59 (18 Go-history, 18 canonical, 36 emit lists), without a removed
pass, changed known nonpass, crash, or timeout: Go-history checker passes
13,552 diagnostics and 12,862 type/symbol baselines; canonical checker
passes 13,552 diagnostics and 12,861 type/symbol baselines; emit passes
13,556 diagnostics, 12,862 type/symbol, 13,482 JavaScript/declaration, 149
JavaScript-map, and 156 source-map baselines. Exact-source Fourslash passes
4,088 of 4,558 groups, skips 470, fails none, and preserves all 4,622
named source-59 outcomes. The full Rust workspace, 39 CLI regressions,
WebAssembly target check, dev and release builds, warnings-denied Clippy,
source lint, formatting, and generator check pass. Fourslash and the
workspace/transport socket cases used the established sandbox escalation.
The API capability table records the new nested-call guarantee; existing
README summaries remain consistent. The ignored reference checkout advances
to source 60; the published TypeScript pin remains at the base revision.

Six alternating release-server pairs processed one million public sync
`getServerTiming` frames each. Source 59 to source 60 median instructions
retired were 21,035,871,909.5 to 21,061,756,554.5 (+0.123%); median wall
time was 1.275 to 1.275 seconds; peak RSS was 8,601,600 to 8,617,984 bytes
(+0.19%). Six alternating Node sync-client pairs of 100,000 requests each
measured 10.245 to 10.134 microseconds per request (-1.09%) at the median.
These are transport microbenchmarks, not the compiler project cohort or a
measurement of contended nested callbacks. The small instruction and RSS
increases in the framed-server run are disclosed under the user's
nonblocking migration policy; neither indicates a material throughput loss.
The scripts, raw pairs, and compiler/Fourslash outcome files are under
`target/port-623/0060-aad4c72bf2/`.

## 61. `fff11d8a7c` — update localization files

Upstream `fff11d8a7c18c6f3e23eef988406732a5a9294b2` refreshes all 13
OneLoc handbacks and their runtime gzip archives. The 26 files in
`crates/tsrs_diagnostics/loc/` match the source-61 files byte for byte.
The locale checker validates canonical CRLF JSON, diagnostic placeholders,
deterministic gzip payloads, and matching handback/runtime maps against the
2,222 current English diagnostics. The handbacks have 2,218–2,222 translated
keys each; the differences include entries now removed because their text
equals current English. Upstream also changes the handback importer to filter
all existing English-equivalent translations after processing. That rule and
the strengthened test assertions were already present in the source-32
Python port, so
this commit updates the data and source references without changing importer
behavior. Exact-source Go importer and diagnostics package tests pass.

The `--locale` gap is unchanged: the Rust compiler does not load these
catalogs and still emits English diagnostics. The compatibility table and
localization tooling documentation identify the current source-61 data; root
and npm README summaries require no change. The ignored `ts-ref` checkout
advances to source 61, while the published TypeScript pin remains at the base
revision until the full upgrade.

All three complete compiler modes have outcome lists byte-identical to source
60: 18 Go-history, 18 canonical, and 36 emit pass/nonpass lists, with no lost
passes, new failures, crashes, or timeouts. Go-history checker passes 13,552
diagnostics and 12,862 type/symbol baselines; canonical checker passes 13,552
diagnostics and 12,861 type/symbol baselines. Emit passes 13,556 diagnostics,
12,862 type/symbol, 13,482 JavaScript/declaration, 149 JavaScript-map and
156 source-map baselines. Exact-source Fourslash passes 4,088 of 4,558 groups,
skips 470, fails none, and preserves all 4,622 named source-60 outcomes.
The full Rust workspace, 39 CLI regressions, WebAssembly target check,
warnings-denied Clippy, source lint, formatting, diagnostics generator and
locale checks, and Python importer cases pass. Fourslash and workspace socket
checks used the established sandbox escalation after local bind denials.

No release instruction-count, wall-time, or peak-RSS comparison was collected
for source 61 under the user's nonblocking migration policy. The changed
catalogs are not read by the Rust compiler, and this port changes no Rust
runtime path, but no numerical performance-equality claim follows. The
measured source-11 regressions remain disclosed above. Evidence is under
`target/port-623/0061-fff11d8a7c/`.
