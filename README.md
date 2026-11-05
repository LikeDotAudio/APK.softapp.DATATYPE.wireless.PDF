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
| `IMPORT.file.PDF/Entry.py` | the old Python edge, still a declared stub (`STATUS = "stub"`) |

```bash
bash SRC/check.sh
APK_DEPS_TOKEN=$(gh auth token) docker compose -f PODS/POD.workers/Docker/docker-compose.yml up -d --build wireless-pdf
```

### Its carriers on the bus (ROAD-3519)

Every carrier an imported file names is also said as a `Device/` row, with PLUGIN.WIFI's leaf
names, so EDITORS.WirelessBuddy's Discovered panel lists it beside the Wi-Fi radios:

```text
APK.audio/System/Protocols/wireless_pdf/Device/<id>/{name, device_class, frequency_mhz, channel, band,
    model, manufacturer, device, zone, group, source_file}     unkept, again every restate_s (30 s)
APK.audio/Discovery/wireless_pdf/<deviceId>                           presence, the runner's (presence::seen)
```

`device_class` is `Wireless Transmitter` (the transmitter on that frequency); a field the file
does not carry is `-`. Identity is never an address: one table row is one carrier. A SOUNDbase PDF carries no serial (and no units, only channels and their frequencies), so the id is formed from `source file | group | model | band | name`; `channel` is the row name, `band` and `model` the report's own. That text, numbered `#n` where it
repeats, is hashed — `deviceId = wireless_pdf:plan:<fnv1a64>`, `<id>` = the same as one topic segment
(`wireless_pdf_plan_<hex>`) — so the same file imported again lands on the same ids. A file's carriers
last until the same file name is imported again (dropped carriers are said gone at once) or the
worker restarts; presence then says them offline after `presence_inactive_after_s` (90 s). Code:
`SRC/src/carriers.rs`, `carriers_of` in `SRC/src/lib.rs`. Measured 2026-11-04 on the bench: `SRC/importer/fixtures/SB PDF.pdf` sent as a job put 20 carriers under `wireless_pdf/Device/` (20 `frequency_mhz` leaves) and 20 online presence documents. The PDF does carry frequencies; what it lacks is units: no serial, maker or device column.

- [Principles](Documentation/🏛Philosophy/PRINCIPLES.md) — the rules it keeps, and where it breaks them.
- [Architecture](Documentation/🧩Manual and Artifacts/ARCHITECTURE.md) — how it is loaded, its parts, and what a conversion does.
- [Delivery](Documentation/🌍Publishing/DELIVERY.md) — the repository, what the images carry, and where it lands.

Remote: https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.PDF

<!-- wiki:smoke -->
**Smoke test** — ✅ PASS 1/1 since 2026-10-30 13:09 EDT — [results](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)
<!-- /wiki:smoke -->
