# DATATYPE.wireless.PDF

The SoundBase PDF importer among the wireless datatypes: `IMPORT:file:PDF/Entry.py` extracts a
report's text through the Rust `PDFEngine` in `oaRustCore`, and is meant to turn it into the shared
six-column marker table. The row parser is not written; the module declares `STATUS = "stub"` and
every conversion raises instead of returning an empty table.

- [Principles](Documentation/🏛Philosophy/PRINCIPLES.md) — the rules it keeps, and where it breaks them.
- [Architecture](Documentation/🧩Manual and Artifacts/ARCHITECTURE.md) — how it is loaded, its parts, and what a conversion does.
- [Delivery](Documentation/🌍Publishing/DELIVERY.md) — the repository, what the images carry, and where it lands.

Remote: https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.PDF

<!-- wiki:smoke -->
**Smoke test** — ✅ PASS 1/1 since 2026-10-30 13:09 EDT — [results](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)
<!-- /wiki:smoke -->
