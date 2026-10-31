# DATATYPE.wireless.PDF principles

DATATYPE.wireless.PDF exists to read a SoundBase PDF frequency report into the marker table that
the wireless data migration wants: zone, group, device, name, frequency and peak, one row per
microphone or IEM channel. It is a Python importer, `IMPORT:file:PDF/Entry.py`, with the same shape
as its CSV, IAS (HTML) and Shure Workbench (SHOW) siblings. Text extraction is done by the Rust
`PDFEngine` in `oaRustCore`; turning that text into rows is not written yet, and the module says so
rather than pretending.

## The rules it keeps

1. **A stub says so in a constant, and a test asserts it.** `STATUS = "stub"` in
   `IMPORT:file:PDF/Entry.py`, and `status()` returns exactly that constant.
   `test_the_pdf_is_still_declared_a_stub` (`IMPORT:file:PDF/Tests/test_entry.py`) holds it here;
   NODE.BareMetal's `TEST:runner:PYTHON/test_handler_status.py` holds it as the consumer.
2. **An unimplemented parser raises; it never returns an empty table.**
   `convert_soundbase_pdf_v1_to_markers` extracts the text and then raises `NotImplementedError`,
   because a header with no rows reads as a broken parser with no error.
   `test_conversion_refuses_rather_than_returning_no_rows` runs it on `Tests/Assets/soundbase_report.pdf`.
3. **One header row for every wireless importer, one copy.** `MARKER_HEADERS` is
   `apk_wireless_import.MARKER_HEADERS` — the same object the IAS and Shure edges import
   (`test_the_header_row_is_the_shared_one`).
4. **A self test never passes for having run nothing.** `run_tests()` is the shared `Lifecycle`'s:
   `False` for a stub (`test_a_stub_never_passes_run_tests`).
5. **The PDF is read by Rust, with no silent Python fallback.** The engine is imported from
   `oaRustCore.pdf_parser`; when it is missing, `extract_text` raises `RuntimeError("Rust PDF parser required")`.
6. **The v2 entry point does not invent a second parser.** `convert_soundbase_pdf_v2_to_markers`
   calls v1, and the two legacy names are aliases.

## Where it breaks them

- **The row parser does not exist.** Every conversion raises. This is declared, not hidden, but it
  is the module's whole purpose.
- When `oaRustCore` is missing, `STATUS` still reads `"stub"` where the other edges say
  `"unavailable"`. Kept by design: the parser is what is missing either way, and "stub" is the
  word the consumer test pins.
- Nothing in the monorepo calls this importer at run time; only the tests load it. The
  Softapp Python is also excluded from every image (`.dockerignore:81`).
- There is no page: `display.json` names no `app` (it once named a missing `index.html`), the
  launcher finds no entry file, and lists the folder itself.
