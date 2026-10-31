# DATATYPE.wireless.PDF architecture

## Using it — the worker

`Worker-WIRELESS_PDF` (image `worker-wireless-pdf:local`, project `workers`, no port) answers jobs
on the bus. Subscribe to `APK.audio/System/Protocols/wireless_pdf/incoming/result`, then publish,
not retained, to `APK.audio/System/Protocols/wireless_pdf/outgoing/job`:

```json
{"job_id": "<yours>", "filename": "SB PDF.pdf", "content_b64": "<the PDF, base64>"}
```

The answer carries `schema: 1`, the same `job_id`, `tool: "DATATYPE.wireless.PDF"`, `ok`, and under
`result` a `SoundbaseParseResult`: `status` (`success` | `empty`), `source_format:
"Soundbase_PDF_Report"`, `file_path`, `total_channels`, `skipped_rows`, `channels[]` of `{group,
model, band, name, preset, spacing, frequency_mhz}` — the record the `wireless_soundbase` worker
returns for the CSV export. A job over `max_job_bytes` (16 MiB, `SRC/config.ini`), not a PDF, or a
PDF with no header row comes back `ok: false` with an `error`.

```bash
docker compose -f PODS/POD.workers/Docker/docker-compose.yml up -d --build wireless-pdf   # APK_DEPS_TOKEN set
bash SRC/check.sh                                                                          # cargo test --workspace --locked, + pytest
```

## How the parser reads a report (`SRC/importer/src/lib.rs`)

```text
PDF bytes ─▶ pdf-extract OutputDev "Collect": every glyph with page, x, y (top-down), font size
          ─▶ lines (baselines within 0.4 size) ─▶ words (split at a space glyph or a 0.3-size gap)
          ─▶ a header row (names Frequency and another of Model Band Name Preset Spacing) opens a
             section: its columns by x, its group = the last "(N frequencies)" heading, else the
             line above the header
          ─▶ anchors: a word in the Frequency column, in the header's font size, that is a usable MHz
          ─▶ every other table word joins the nearest anchor on its page within 1.5 sizes, in its column;
             wrapped cell lines ("G57 -" / "Narrowband") join top to bottom
          ─▶ SoundbaseChannel rows, in page and y order
```

The table may continue on the next page with no header: the last section carries over.

## Using it — the Python edge

The OS lists Softapps from `PODS/POD.THIN/OS/system/project-apps.json`, which `walk_project()`
(`PODS/POD.THIN/WEB.OsApi/SRC/server.py`) builds from the checkout from `display.json`.

`IMPORT:file:PDF/Entry.py` is the old desktop edge, kept a declared stub. It needs one package,
`apk-wireless-import` (DATATYPE.wireless.CSV/SRC/python, pinned by rev in
`IMPORT:file:PDF/requirements.txt`), and — for text extraction only — the compiled `oaRustCore`
(built from NODE.BareMetal `Core/CORE:engine:RUST`):

```sh
cd "PODS/POD.THIN/Softapps/DATATYPE/DATATYPE.wireless.PDF/IMPORT:file:PDF"
python3 -m pytest -q Tests        # 9 tests; conftest finds apk_wireless_import (installed, $APK_WIRELESS_IMPORT_DIR, or the sibling checkout)
```

## Parts

| Part | File | What it holds |
| :-- | :-- | :-- |
| Parser crate | `SRC/importer/` | `apkaudio-wireless-soundbase-pdf`: `parse_soundbase_pdf(bytes, name)`, `parse_soundbase_pdf_file(path)`; pure. `tests/fixture.rs` holds `fixtures/SB PDF.pdf` against `fixtures/SB PDF.csv` (copies of DATATYPE.wireless.SOUNDbase's `test_data/`, inside `SRC/` so the image builds from `SRC/` alone). |
| Worker | `SRC/` | `apk-worker-wireless-pdf`: `handle_job`, `run_job_agent` (the runner's `JobWorker`), `self_test.rs`, `config.ini` (`[wireless_pdf]`), `check.sh`, the two bins. |
| Stack | `Docker/` | `Dockerfile` (build context `SRC/`, private git deps by BuildKit secret `apk_deps_token`) and `docker-compose.yml` (service `wireless-pdf`, container `Worker-WIRELESS_PDF`), included by `PODS/POD.workers/Docker/docker-compose.yml`. |
| Python edge | `IMPORT:file:PDF/Entry.py` | `STATUS`, `extract_text`, the v1/v2 converters and their legacy aliases; `MARKER_HEADERS` and the `start`/`stop`/`status`/`run_tests` lifecycle come from `apk_wireless_import` (`Lifecycle(STATUS)`: a stub's `run_tests()` is False). |
| Tests | `IMPORT:file:PDF/Tests/` | `test_entry.py` (STATUS is "stub", conversion raises on the fixture, run_tests False, the shared header row) and `Assets/soundbase_report.pdf`, a one-page fixture. |
| Dependency | `IMPORT:file:PDF/requirements.txt` | `apk-wireless-import` at a DATATYPE.wireless.CSV rev. |
| Launcher manifest | `display.json` | Name, icon, description, category `DATATYPE`, `entry` = the parser crate, `requires` the `wireless_pdf` worker and the broker (the SoundBase sibling's shape). |

Outside this repository: the PDF engine is
`PODS/POD.THICK/NODE.BareMetal/SRC/Core/CORE:engine:RUST/PARSER/pdf_parser.rs`, registered as
`oaRustCore.pdf_parser` (`CORE:engine:RUST/lib.rs:225`). The SoundBase importer that reads CSV
exports of these reports, and owns the channel record this crate returns, is `DATATYPE.wireless.SOUNDbase/SRC/importer` (`apkaudio-wireless-soundbase`).

## Data flow (Python edge)

1. **Load.** On import, `Entry.py` tries `from oaRustCore.pdf_parser import PDFEngine` and keeps one
   engine, or `None` when the Rust build is absent.
2. **Extract.** `extract_text(pdf_path)` logs through `apk_wireless_import.log` and returns the
   page text from `PDFEngine.extract_text`, or raises when there is no engine.
3. **Convert.** `convert_soundbase_pdf_v1_to_markers(pdf_path)` (and v2, and the two legacy
   aliases) extracts the text, then raises `NotImplementedError`. No rows are produced.
4. **Report.** `status()` returns `STATUS`, `"stub"`.

The Python edge reads the PDF file named by the caller and nothing else; it publishes no bus topic.
The worker reads only the bytes in the job and publishes only its answer.
