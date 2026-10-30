# DATATYPE.wireless.PDF architecture

## Using it

The OS lists Softapps from `PODS/POD.THIN/OS/system/project-apps.json`, which `walk_project()`
(`PODS/POD.THIN/WEB.OsApi/SRC/server.py:3842`) builds from the checkout. Under Softapps it reads
`name`, `icon` and `description` from `display.json` and opens `entry` or the first entry file it
finds (`APP_ENTRY_MARKERS`, `server.py:3814`); `app` is inert there (`server.py:2758`). This folder
has neither, so "Wireless PDF Datatype" is listed as a declared `folder` app and opens the folder.

The importer is a Python module loaded by path, the way the status test does it. It needs
`PODS/POD.THICK/NODE.BareMetal/SRC/Core` on `PYTHONPATH` for `importer_logging`, and the compiled
`oaRustCore` (built from `Core/CORE:engine:RUST`) for text extraction:

```sh
cd PODS/POD.THICK/NODE.BareMetal/SRC/Core
PYTHONPATH=. python3 -m unittest "TEST:runner:PYTHON/test_handler_status.py"
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
| Importer | `IMPORT:file:PDF/Entry.py` | `MARKER_HEADERS`, `STATUS`, `extract_text`, the v1/v2 converters and their legacy aliases, and the `start`/`stop`/`status`/`run_tests` lifecycle the importers share. |
| Launcher manifest | `display.json` | Name "Wireless PDF Datatype", icon, description, `app: index.html` (no such file). |

Outside this repository: the PDF engine is
`PODS/POD.THICK/NODE.BareMetal/SRC/Core/CORE:engine:RUST/PARSER/pdf_parser.rs`, registered as
`oaRustCore.pdf_parser` (`CORE:engine:RUST/lib.rs:225`). A Rust SoundBase importer that reads CSV
exports of these reports is `DATATYPE.wireless.SOUNDbase/soundbase_importer.rs`.

## Data flow

1. **Load.** On import, `Entry.py` tries `from oaRustCore.pdf_parser import PDFEngine` and keeps one
   engine, or `None` when the Rust build is absent.
2. **Extract.** `extract_text(pdf_path)` logs through `importer_logging.matrix_log` and returns the
   page text from `PDFEngine.extract_text`, or raises when there is no engine.
3. **Convert.** `convert_soundbase_pdf_v1_to_markers(pdf_path)` (and v2, and the two legacy
   aliases) extracts the text, then raises `NotImplementedError`. No rows are produced.
4. **Report.** `status()` returns `STATUS`, `"stub"`.

It reads the PDF file named by the caller and nothing else. It publishes no bus topic and calls no
HTTP endpoint.
