# Build identity and small macOS RSS differences

Issue 623 source 6 exposed a benchmark nuisance: changing only a Mach-O UUID and the ad-hoc signature hash
covering that header changed startup and Compiler-Unions peak RSS, with every executable/data section unchanged.
The ordinary fixed-binary comparison remains valid evidence for those particular builds; it does not by itself
separate code effects from build-dependent allocator placement. Do not respond to a page-sized RSS difference
with repeated inline/cold attribute experiments before checking this possibility.

Evidence is under `target/port-623/0006-792ffccb90/diagnosis/`. The strict reciprocal controls preserve the
original signature identifier, flags, 4-KiB hash-page format and all bytes except the 16-byte `LC_UUID` field and
its 32-byte signature hash. Every copy passes `codesign --verify --strict`. Equal-length executable paths and
113 identical runtime assets remove the observed path-length allocation difference. Normal, non-injected runs
show a metadata-only +16-KiB startup difference and +24-KiB Compiler-Unions median difference for baseline code.
The reciprocal initial-code copies also change startup RSS by 16 KiB, though their Compiler-Unions medians tie.
All raw pairs and complete section hashes are in `strict-control-summary.json` and its linked inputs.

A separate injected self-inspection probe localizes the startup difference to system `MALLOC_SMALL` residency,
with equal live allocation counts and bytes in equal-length-path controls; original short/long paths differ by
48 live bytes. Injection changes the observation environment, and exit residency
is not necessarily peak residency; these probes are diagnostic evidence only. Path-only controls did not
reproduce the original difference. Helper outlining, cold placement and parent outlining all failed focused
comparisons and were reverted. A smaller parent stack frame did not reduce RSS.

[Apple's libmalloc documentation](https://github.com/apple-oss-distributions/libmalloc/blob/c49dafa25f1efe8607701ae6014a663ad2ee437f/doc/xzone_malloc.md#type-isolation)
(revision `c49dafa25f1efe8607701ae6014a663ad2ee437f`, libmalloc-812.100.31) describes type bucketing with entropy
stable for a binary within one boot. This supports build-dependent placement as an explanation for persistent
metadata-only differences. The installed allocator's complete causal chain was not established. Repeated
launches of one binary do not independently sample this nuisance.

For source 6, root reviewed a bounded protocol before measurement: ten preregistered SHA-256-derived UUIDs
applied symmetrically to both frozen code variants, equal-length paths, one warmup and one timed run per
binary/identity, alternating order, and no allocator override or injection. A disjoint reserved set of ten
identities is used once, in reversed order, only for screened cells. The existing paired sign-test thresholds
remain unchanged. All identities count; no favorable UUID is selected and no forced UUID ships. The frozen
`build-identity-protocol.json`, separate review, raw results and validating evaluator retain the complete record.

This is a source-6 adjudication, not an automatic replacement for future performance gates. Before applying it
elsewhere, preserve the ordinary comparison, demonstrate the relevant metadata nuisance with controls, and
preregister the sampling unit, identities, coverage and decision rule. Root sets each later source's policy.
A controlled result of no detected regression does not establish equal RSS for the original shipping binaries.
