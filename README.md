# DATATYPE.wireless.PDF

The SoundBase PDF frequency report importer among the wireless datatypes, as a worker on the bus:
`Worker-WIRELESS_PDF` (module `wireless_pdf`, pod `PODS/POD.workers`, no port). Post
`{"job_id", "filename", "content_b64"}` to `APK.audio/System/Protocols/wireless_pdf/outgoing/job`;
the answer on `.../wireless_pdf/incoming/result` carries the report's rows as the SoundBase channel
records (`group, model, band, name, preset, spacing, frequency_mhz`) — the same the
`wireless_soundbase` worker returns for the report's CSV export.

| Path | What |
| :-- | :-- |
| `SRC/importer/` | the parser crate `apkaudio-wireless-soundbase-pdf`: positioned glyphs → lines → the table by its header row; `tests/fixture.rs` holds `fixtures/SB PDF.pdf` to `fixtures/SB PDF.csv` field for field |
| `SRC/` | the worker `apk-worker-wireless-pdf` (runner job loop, wireless kernel envelope, self test, `config.ini` `[wireless_pdf]`) and `check.sh` |
| `Docker/` | the worker stack, included by `PODS/POD.workers/Docker/docker-compose.yml` |
| `IMPORT:file:PDF/Entry.py` | the old Python edge, still a declared stub (`STATUS = "stub"`) |

```bash
bash SRC/check.sh
APK_DEPS_TOKEN=$(gh auth token) docker compose -f PODS/POD.workers/Docker/docker-compose.yml up -d --build wireless-pdf
```

- [Principles](Documentation/🏛Philosophy/PRINCIPLES.md) — the rules it keeps, and where it breaks them.
- [Architecture](Documentation/🧩Manual and Artifacts/ARCHITECTURE.md) — how it is loaded, its parts, and what a conversion does.
- [Delivery](Documentation/🌍Publishing/DELIVERY.md) — the repository, what the images carry, and where it lands.

Remote: https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.PDF

<!-- wiki:smoke -->
**Smoke test** — ✅ PASS 1/1 since 2026-10-30 13:09 EDT — [results](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)
<!-- /wiki:smoke -->
