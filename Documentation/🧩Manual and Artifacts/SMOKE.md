# Smoke test — DATATYPE.wireless.PDF

Is `DATATYPE.wireless.PDF` deployed? Written by `.apk.scripts/smoke.py` from the module's own shape; what each check means is in that script's header.

← [Wiki home](../README.md)

**❌ FAIL** — 2/3 checks, since 2026-11-12 00:16 EST

| Check | Target | Result | Detail |
|---|---|---|---|
| pushed | [APK.softapp.DATATYPE.wireless.PDF](https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.PDF.git) | ❌ | 1 commit(s) not pushed |
| container | `Worker-WIRELESS_PDF` | ✅ | running, healthy |
| web | [SRC/importer/src/lib.rs](https://apk.audio/APK%3ASoftapps/DATATYPE/DATATYPE.wireless.PDF/SRC/importer/src/lib.rs) | ✅ | HTTP 200 |

Re-run: `python3 .apk.scripts/smoke.py PODS/POD.THIN/Softapps/DATATYPE/DATATYPE.wireless.PDF`

<!-- smoke:sig bca5d5b91993 since 2026-11-12 00:16 EST -->
