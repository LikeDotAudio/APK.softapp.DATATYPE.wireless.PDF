# DATATYPE.wireless.PDF principles

DATATYPE.wireless.PDF exists to read a SoundBase PDF frequency report into the same channel records
the SoundBase CSV export gives: group, model, band, name, preset, spacing and frequency, one row per
frequency. Since LOOSE-workers-convert-rest it does that as a worker on the bus,
`Worker-WIRELESS_PDF` (module `wireless_pdf`, pod `PODS/POD.workers`): the parser is the Rust crate
`SRC/importer/` (`apkaudio-wireless-soundbase-pdf`), behind the runner's job loop in `SRC/`. The old
Python edge, `IMPORT.file.PDF/Entry.py`, is kept as the declared stub it was.

## The rules it keeps

1. **One record out, whatever came in.** A row is DATATYPE.wireless.SOUNDbase's `SoundbaseChannel`
   and the result its `SoundbaseParseResult` (that crate by git rev), so the PDF and the CSV export
   of one report read the same. `the_pdf_reads_as_its_csv_export` (`SRC/importer/tests/fixture.rs`)
   holds every field of all 20 rows of `fixtures/SB PDF.pdf` against `fixtures/SB PDF.csv`;
   `the_result_shape_is_pinned` (`SRC/src/lib.rs`) holds the wire keys.
2. **Rows come from the report's own layout, not from guessed text.** Glyphs keep their page
   positions; the header row names the columns; a row is anchored on its frequency and wrapped
   cells join their row by distance (`SRC/importer/src/lib.rs` header). Only text in the header
   row's font size is table text, so a running title over the Frequency column is never a row.
3. **A file that is not a report is an error, never an empty table.** Not a PDF, an unreadable PDF,
   or a PDF with no header row comes back `ok: false` with the reason
   (`what_is_not_a_pdf_is_answered_not_ok`, `a_bad_job_is_answered_not_dropped`). A malformed font
   that panics the extractor is caught and answered (`catch_unwind` in `glyphs`).
4. **The band rule is the kernel's.** A frequency counts only inside 100–3000 MHz
   (`apkaudio_wireless_kernel::usable_mhz_text`); the status word is `import_status`.
5. **A job is answered, always, under its id, and nothing is retained.** The envelope is the
   wireless kernel's `job` module, shared with the CSV, IAS, SoundBase and WWB workers.
6. **The self test runs the real thing.** `SRC/src/self_test.rs` sends the fixture PDF, compiled
   into the binary, through `handle_job` and passes only on 20 channels, 0 skipped.
7. **A stub says so.** The Python edge keeps `STATUS = "stub"`, raises rather than returning an
   empty table, and its `run_tests()` is `False` (`IMPORT.file.PDF/Tests/test_entry.py`).

## Where it breaks them

- **The dialect is one fixture deep.** The parser is written against one SoundBase report
  (`fixtures/SB PDF.pdf`). Name and Preset are empty in it, so their columns are mapped by header
  position but not yet seen filled.
- **No page posts a PDF yet.** The Data Migration *Soundbase* frame sends the CSV export as text to
  `wireless_soundbase`; nothing in the GUI sends a PDF as `content_b64` to `wireless_pdf`.
- The Python edge is still a stub that nothing calls at run time, excluded from every image
  (`.dockerignore:81`).
