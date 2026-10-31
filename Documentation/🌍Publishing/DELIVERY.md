# DATATYPE.wireless.PDF delivery

- **Repository:** https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.PDF (private), a
  submodule of the APK.audio monorepo at `PODS/POD.THIN/Softapps/DATATYPE/DATATYPE.wireless.PDF`.
- **Build:** the Python needs no build. Its text extraction needs `oaRustCore`, which is built from
  `PODS/POD.THICK/NODE.BareMetal/SRC/Core/CORE:engine:RUST` and is not part of this repository.
- **Images:** `PODS/POD.THIN/WEB.Frontend-Assets/Docker/Dockerfile` copies the whole
  `PODS/POD.THIN/Softapps` tree to `/srv/www/Softapps` (web-static). The root `.dockerignore`
  drops `PODS/POD.THIN/Softapps/**/*.py` (`.dockerignore:81`), so `Entry.py` reaches no image; only
  `display.json` and the wiki do. `PODS/POD.THICK/NODE.BareMetal/Docker/Dockerfile` copies the
  Softapps tree into its build stage too, under the same rule. Storage-Portal
  (`DATABASE.server.SQL`, :8080) no longer carries a copy: its nginx proxies `/APK:Softapps/` and
  `/Softapps/` to web-static (PLAN-3421.01,
  `PODS/POD.database/DATABASE.server.SQL/Docker/Dockerfile:35`).
- **Where it lands:** `/APK:Softapps/DATATYPE/DATATYPE.wireless.PDF/` on web-static, and the same
  path through :8080; web-static's nginx would answer a `.py` with 404 in any case
  (`PODS/POD.THIN/WEB.Frontend-Assets/Docker/nginx.conf:46`). The importer runs only from a checkout,
  under the NODE.BareMetal `Core` Python path.
- **Tested:** by hand — `python3 -m pytest -q Tests` in `IMPORT:file:PDF/` (9 passed 2026-10-30);
  NODE.BareMetal's `TEST:runner:PYTHON/test_handler_status.py` is the consumer check.
- **After an edit:** run the tests above, then
  `PODS/POD.manager/MANAGER.Docktor/K8/up-stack.sh WEB.Frontend-Assets` if `display.json` changed.
  A `DATABASE.server.SQL` rebuild is no longer needed for Softapps since PLAN-3421.01. Commit in
  this repository, then bump the submodule pointer in APK.audio.

## Delivered from the 2026-10-30 audit

- **One shared header row and lifecycle** — `MARKER_HEADERS` and `start`/`stop`/`status`/`run_tests` come from `apk_wireless_import` (DATATYPE.wireless.CSV/SRC/python, package `apk-wireless-import`), the one copy the IAS, PDF, Shure and CSV edges import; the three verbatim copies are gone. Verified by `Tests/test_entry.py` (the header row *is* the shared object).
- **run_tests() no longer lies** — `Lifecycle(STATUS)`: a stub's `run_tests()` is `False` (it returned `True` unconditionally). Pinned by `test_a_stub_never_passes_run_tests`.
- **Tests at home** — `IMPORT:file:PDF/Tests/` with a one-page fixture PDF: STATUS is "stub", both converters raise on it, text extraction needs the engine (9 passed). NODE.BareMetal's `test_handler_status.py` stays as the consumer check (handoff filed so it puts the shared package on its path).
- **No PYTHONPATH into NODE.BareMetal** — `Entry.py` no longer imports `importer_logging`; logging is `apk_wireless_import.log` (oaLogging when present, else nothing). The dependency is declared as a package by rev in `requirements.txt`.
- **Doc drift and comment noise** — ARCHITECTURE's `display.json` row says what the file holds (no `app`), the `# Inline comment: Logic for …` lines are gone, and the header's moved paths are corrected.
