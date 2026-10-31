# Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
# MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
"""Entry.py declares what it is, and its lifecycle cannot claim more."""
import apk_wireless_import
import Entry


def test_status_is_a_word_and_status_reports_it():
    assert Entry.STATUS in apk_wireless_import.STATUS_WORDS
    assert Entry.status() == Entry.STATUS


def test_the_header_row_is_the_shared_one():
    assert Entry.MARKER_HEADERS is apk_wireless_import.MARKER_HEADERS


def test_run_tests_is_false_unless_live():
    if Entry.STATUS != "live":
        assert Entry.run_tests() is False


def test_start_and_stop_are_no_ops():
    assert Entry.start() is None and Entry.stop() is None


from pathlib import Path

import pytest

FIXTURE = Path(__file__).resolve().parent / "Assets" / "soundbase_report.pdf"


def test_the_pdf_is_still_declared_a_stub():
    """When the row parser lands: STATUS "live", and change this test with it."""
    assert Entry.STATUS == "stub"


def test_a_stub_never_passes_run_tests():
    assert Entry.run_tests() is False


def test_the_fixture_is_a_pdf():
    assert FIXTURE.read_bytes().startswith(b"%PDF-")


def test_conversion_refuses_rather_than_returning_no_rows():
    """A header with no field renders as a blank column: an empty result must raise."""
    with pytest.raises((NotImplementedError, RuntimeError)):
        Entry.convert_soundbase_pdf_v1_to_markers(FIXTURE)
    with pytest.raises((NotImplementedError, RuntimeError)):
        Entry.Marker_convert_SB_v2_PDF_File_report_to_csv(FIXTURE)


def test_text_extraction_needs_the_engine():
    if Entry.rust_pdf_engine is None:
        with pytest.raises(RuntimeError):
            Entry.extract_text(FIXTURE)
    else:
        assert "470.125" in Entry.extract_text(FIXTURE)
