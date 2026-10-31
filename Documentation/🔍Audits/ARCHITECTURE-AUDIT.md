# 🔍 Architecture audit — DATATYPE.wireless.PDF (2026-10-30)

Audited against the APK.audio multi-repo architecture standard: coupling, boundaries, duplication, contracts.
← [Audits](INDEX.md) · Improvements: [🛣Roadmap/IMPROVEMENTS.md](../%F0%9F%9B%A3Roadmap/IMPROVEMENTS.md)

## Bounded context
The SoundBase PDF report dialect, from report to the six-column wireless marker table. Python, 77 lines (`IMPORT.file.PDF/Entry.py`) plus `display.json`. Text extraction works through the Rust `oaRustCore.pdf_parser.PDFEngine`. The row parser is not written, the module declares `STATUS = "stub"`, and every conversion raises.

**Sampled:** every tracked file, plus the test that exercises it (`PODS/POD.THICK/NODE.BareMetal/SRC/Core/TEST.runner.PYTHON/test_handler_status.py`) and the sibling importers `DATATYPE.wireless.IAS` and `DATATYPE.wireless.SHUREwirelessWorkbench`.

## Context map
| Talks to | How (topic / endpoint / file / import) | Relationship |
|---|---|---|
| `oaRustCore` (built from `NODE.BareMetal/SRC/Core/CORE.engine.RUST`) | `from oaRustCore.pdf_parser import PDFEngine` (`Entry.py:30-34`) | Customer–Supplier (in-process) |
| `importer_logging` (in `NODE.BareMetal/SRC/Core`) | `from importer_logging import matrix_log` (`Entry.py:26`); needs that folder on `PYTHONPATH` | Conformist |
| DataMigration (caller) | `MARKER_HEADERS` and the `convert_*` / legacy `Marker_convert_*` functions | Published Language (marker table) |
| `NODE.BareMetal` status test | loads this file by path (`test_handler_status.py:49`) and asserts `STATUS` | Customer (test lives outside this repo) |
| OS launcher | `display.json` (name, icon, description) | Conformist |

## Scorecard
| Category | Score (0–3) | Finding |
|---|---|---|
| Coupling & Cohesion | 2 | Imports only the shared `apk_wireless_import` package (declared by rev) and the optional `oaRustCore` wheel; no PYTHONPATH into NODE.BareMetal |
| Boundary Integrity | 2 | The Rust engine sits behind `extract_text()` and a guarded import; the marker header row is the shared module's, not a local literal |
| Duplication Control | 3 | `MARKER_HEADERS` and the lifecycle are one copy, shared by the four Python edges |
| Testing & Contracts | 3 | `Tests/` here with a fixture PDF pins STATUS, the refusal and run_tests; NODE.BareMetal's status test is the consumer check |
| Change management | 2 | `STATUS = "stub"` is explicit and pinned; the legacy aliases are kept for callers |
Score key: 3 meets the standard · 2 minor gaps · 1 significant gaps · 0 absent / violated.

## Findings
1. **Good: refusal over an empty result.** Conversion raises `NotImplementedError` instead of returning `MARKER_HEADERS, []`, so a half-built importer cannot look like an empty file (`Tests/test_entry.py`).
2. **Good: the vendor engine sits behind one function.** `PDFEngine` is imported once, guarded, and reached only through `extract_text()`.
3. **STATUS does not degrade without the engine.** It reads `"stub"` even where `oaRustCore` is absent (the other edges say `"unavailable"`). Kept: "stub" is the truer word (the row parser is what is missing), and NODE.BareMetal's consumer test pins it.
