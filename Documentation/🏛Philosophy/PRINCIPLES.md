# DATATYPE.wireless.PDF principles

DATATYPE.wireless.PDF exists to read a SoundBase PDF frequency report into the marker table that
the wireless data migration wants: zone, group, device, name, frequency and peak, one row per
microphone or IEM channel. It is a Python importer, `IMPORT:file:PDF/Entry.py`, with the same shape
as its CSV, IAS (HTML) and Shure Workbench (SHOW) siblings. Text extraction is done by the Rust
`PDFEngine` in `oaRustCore`; turning that text into rows is not written yet, and the module says so
rather than pretending.

## The rules it keeps

1. **A stub says so in a constant, and a test asserts it.** `STATUS = "stub"`
   (`IMPORT:file:PDF/Entry.py:38`), and `status()` returns exactly that constant
   (`IMPORT:file:PDF/Entry.py:73`). `test_pdf_is_still_declared_a_stub` in
   `PODS/POD.THICK/NODE.BareMetal/SRC/Core/TEST:runner:PYTHON/test_handler_status.py:70` holds it.
2. **An unimplemented parser raises; it never returns an empty table.**
   `convert_soundbase_pdf_v1_to_markers` extracts the text and then raises `NotImplementedError`
   (`IMPORT:file:PDF/Entry.py:53`), because a header with no rows reads as a broken parser with no
   error. The status test checks the raise (`test_handler_status.py:78`).
3. **One header row for every wireless importer.** `MARKER_HEADERS`
   (`IMPORT:file:PDF/Entry.py:28`) is the same six columns as the other importers;
   `test_marker_headers_agree_across_importers` (`test_handler_status.py:88`) compares them.
4. **The PDF is read by Rust, with no silent Python fallback.** The engine is imported from
   `oaRustCore.pdf_parser` (`IMPORT:file:PDF/Entry.py:31`); when it is missing, `extract_text`
   raises `RuntimeError("Rust PDF parser required")` (`IMPORT:file:PDF/Entry.py:45`).
5. **The v2 entry point does not invent a second parser.** `convert_soundbase_pdf_v2_to_markers`
   calls v1 (`IMPORT:file:PDF/Entry.py:60`), and the two legacy names are aliases
   (`IMPORT:file:PDF/Entry.py:64`-`IMPORT:file:PDF/Entry.py:65`).

## Where it breaks them

- **The row parser does not exist.** Every conversion raises. This is declared, not hidden, but it
  is the module's whole purpose.
- `IMPORT:file:PDF/Entry.py:26` imports `importer_logging`, which lives only in
  `PODS/POD.THICK/NODE.BareMetal/SRC/Core/`. The file cannot be imported from this repository, or
  from anywhere without that directory on `PYTHONPATH`.
- `run_tests()` returns `True` unconditionally (`IMPORT:file:PDF/Entry.py:75`), so a caller asking
  the module to test itself is told it passed.
- When `oaRustCore` is missing, `STATUS` still reads `"stub"`. The status test's own header says
  the importers degrade to `"unavailable"` on a machine without the Rust build; this one does not.
- The header comment names paths that moved: `oaFileImportHTML/Entry.py` is now
  `DATATYPE.wireless.IAS/IMPORT:file:HTML/Entry.py`, `src/lib.rs` is now
  `PODS/POD.THICK/NODE.BareMetal/SRC/Core/CORE:engine:RUST/lib.rs`, and
  `Tests/test_handler_status.py` is now `Core/TEST:runner:PYTHON/test_handler_status.py`.
- Nothing in the monorepo calls this importer at run time; only the status test loads it. The
  Softapp Python is also excluded from every image (`.dockerignore:81`).
- `display.json:5` names `"app": "index.html"`, and there is no `index.html`. Under Softapps the
  launcher ignores `app`, finds no entry file, and lists the folder itself.
