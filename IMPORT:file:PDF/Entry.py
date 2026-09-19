# Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
# MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
# Reads a SoundBase PDF report toward the marker table DataMigration wants.
#
# ⚠️ STUB — text extraction works, marker parsing does not.
#
# Owns: the SoundBase PDF dialect. `oaRustCore.pdf_parser.PDFEngine`
# returns the page text; turning that text into rows is the part nobody has
# written, so this module declares itself rather than returning an empty table.
#
# Must not: return `MARKER_HEADERS, []`. That was the previous shape, and it is
# the exact failure DataMigration's lane comment names — "a header with no field
# on the record renders as a blank column: no error, no log line, and it reads
# as a broken parser." An empty result from a real file must raise.
#
# The import name is a contract with `src/lib.rs`; see the note in
# `oaFileImportHTML/Entry.py`. This module spent its life importing
# `oa_pdf_parser_rs` while the directory was named `XXXoa_pdf_parser_rs` and
# therefore outside the crate's module tree, so every call raised while
# `status()` reported "Running with Rust Engine".
#
# When the row parser lands: change STATUS to "live", implement
# `convert_soundbase_pdf_v1_to_markers`, and update `Tests/test_handler_status.py`
# in the same commit.

from importer_logging import matrix_log

MARKER_HEADERS = ["ZONE", "GROUP", "DEVICE", "NAME", "FREQ_MHZ", "PEAK"]

try:
    from oaRustCore.pdf_parser import PDFEngine
    rust_pdf_engine = PDFEngine()
except ImportError:
    rust_pdf_engine = None

# Greppable, and asserted by a test that fails the moment someone half-implements
# the row parser. Not "unavailable" — the engine loads; the parser is missing.
STATUS = "stub"


# Inline comment: Logic for extract_text
def extract_text(pdf_path):
    """The implemented half: raw page text out of the Rust engine."""
    if rust_pdf_engine is None:
        raise RuntimeError("Rust PDF parser required")
    matrix_log("ui", "importer", "extract_text", "🚀 Using HIGH-PERFORMANCE RUST PDF engine.", "DEBUG")
    return rust_pdf_engine.extract_text(str(pdf_path))


# Inline comment: Logic for convert_soundbase_pdf_v1_to_markers
def convert_soundbase_pdf_v1_to_markers(pdf_path):
    extract_text(pdf_path)
    raise NotImplementedError(
        "SoundBase PDF text extracts, but no marker parser turns it into rows. "
        "See STATUS in this file."
    )


# Inline comment: Logic for convert_soundbase_pdf_v2_to_markers
def convert_soundbase_pdf_v2_to_markers(pdf_path):
    return convert_soundbase_pdf_v1_to_markers(pdf_path)


Marker_convert_SB_PDF_File_report_to_csv = convert_soundbase_pdf_v1_to_markers
Marker_convert_SB_v2_PDF_File_report_to_csv = convert_soundbase_pdf_v2_to_markers


# Inline comment: Logic for start
def start(): pass
# Inline comment: Logic for stop
def stop(): pass
# Inline comment: Logic for status
def status(): return STATUS
# Inline comment: Logic for run_tests
def run_tests(): return True

__all__ = ["extract_text", "convert_soundbase_pdf_v1_to_markers", "convert_soundbase_pdf_v2_to_markers", "Marker_convert_SB_PDF_File_report_to_csv", "Marker_convert_SB_v2_PDF_File_report_to_csv", "MARKER_HEADERS", "STATUS", "start", "stop", "status", "run_tests"]
