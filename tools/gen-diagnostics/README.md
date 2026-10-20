# Diagnostic localization data

`crates/tsrs_diagnostics/loc/` contains the 13 source-32 TypeScript locale
handbacks and matching runtime gzip archives, copied byte for byte from
`cb72b2e17b75557db7e3286d835544aef5d85f6c`. The English diagnostic
catalog still comes from `gen.py`.

`import_localization.py` mirrors the upstream OneLoc handback importer. Run it
with `--source <English JSON> --handback <OneLoc JSON> --out <locale.generated.json>`;
it also reads `<OneLoc JSON>.lct`. It keeps previous translations until a
replacement has `Stat="Loc"`, removes obsolete keys and English-equivalent
translations, and writes sorted CRLF JSON. This repository does not run the
upstream OneLoc pipeline.

`check_localizations.py` checks each catalog's canonical formatting, diagnostic
placeholders and gzip payload. `tools/gen-check.sh` runs it and the importer
unit cases. The CLI's `--locale` option remains ignored: these source catalogs
are retained for a future locale runtime port, not loaded by the compiler.
