# DATATYPE.wireless.PDF architecture

## Using it

The OS lists Softapps from `PODS/POD.THIN/OS/system/project-apps.json`, which `walk_project()`
(`PODS/POD.THIN/WEB.OsApi/SRC/server.py:3842`) builds from the checkout. Under Softapps it reads
`name`, `icon` and `description` from `display.json` and opens `entry` or the first entry file it
finds (`APP_ENTRY_MARKERS`). This folder has no page, so its `display.json` declares no `entry` and
no `app`, and the walker finds no entry file either, so "Wireless PDF Datatype" is listed as a declared `folder` app and opens the folder.

The importer is a Python module. It needs one package, `apk-wireless-import`
(DATATYPE.wireless.CSV/SRC/python, pinned by rev in `IMPORT:file:PDF/requirements.txt`), and —
for text extraction only — the compiled `oaRustCore` (built from NODE.BareMetal
`Core/CORE:engine:RUST`). It no longer needs NODE.BareMetal on `PYTHONPATH`:

```sh
cd "PODS/POD.THIN/Softapps/DATATYPE/DATATYPE.wireless.PDF/IMPORT:file:PDF"
python3 -m pytest -q Tests        # 9 tests; conftest finds apk_wireless_import (installed, $APK_WIRELESS_IMPORT_DIR, or the sibling checkout)
```

```python
import importlib.util
spec = importlib.util.spec_from_file_location("Entry", ".../DATATYPE.wireless.PDF/IMPORT:file:PDF/Entry.py")
entry = importlib.util.module_from_spec(spec); spec.loader.exec_module(entry)
entry.extract_text("report.pdf")                        # page text, needs oaRustCore
entry.convert_soundbase_pdf_v1_to_markers("report.pdf") # raises NotImplementedError
```

## Parts

| Part | File | What it holds |
| :-- | :-- | :-- |
| Importer | `IMPORT:file:PDF/Entry.py` | `STATUS`, `extract_text`, the v1/v2 converters and their legacy aliases; `MARKER_HEADERS` and the `start`/`stop`/`status`/`run_tests` lifecycle come from `apk_wireless_import` (`Lifecycle(STATUS)`: a stub's `run_tests()` is False). |
| Tests | `IMPORT:file:PDF/Tests/` | `test_entry.py` (STATUS is "stub", conversion raises on the fixture, run_tests False, the shared header row) and `Assets/soundbase_report.pdf`, a one-page fixture. |
| Dependency | `IMPORT:file:PDF/requirements.txt` | `apk-wireless-import` at a DATATYPE.wireless.CSV rev. |
| Launcher manifest | `display.json` | Name "Wireless PDF Datatype", icon, description; no `app` and no `entry` (there is no page). |

Outside this repository: the PDF engine is
`PODS/POD.THICK/NODE.BareMetal/SRC/Core/CORE:engine:RUST/PARSER/pdf_parser.rs`, registered as
`oaRustCore.pdf_parser` (`CORE:engine:RUST/lib.rs:225`). A Rust SoundBase importer that reads CSV
exports of these reports is `DATATYPE.wireless.SOUNDbase/SRC/importer` (`apkaudio-wireless-soundbase`).

## Data flow

1. **Load.** On import, `Entry.py` tries `from oaRustCore.pdf_parser import PDFEngine` and keeps one
   engine, or `None` when the Rust build is absent.
2. **Extract.** `extract_text(pdf_path)` logs through `apk_wireless_import.log` and returns the
   page text from `PDFEngine.extract_text`, or raises when there is no engine.
3. **Convert.** `convert_soundbase_pdf_v1_to_markers(pdf_path)` (and v2, and the two legacy
   aliases) extracts the text, then raises `NotImplementedError`. No rows are produced.
4. **Report.** `status()` returns `STATUS`, `"stub"`.

It reads the PDF file named by the caller and nothing else. It publishes no bus topic and calls no
HTTP endpoint.
