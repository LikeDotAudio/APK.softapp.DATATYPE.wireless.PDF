// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! The SoundBase PDF frequency report importer as a worker (LOOSE-workers-convert-rest).
//!
//! ```text
//!   browser ── APK.audio/System/Protocols/wireless_pdf/outgoing/job ──▶ worker
//!   worker  ── APK.audio/System/Protocols/wireless_pdf/incoming/result ─▶ browser
//! ```
//!
//! ONE JOB IN, ONE RESULT OUT, MATCHED BY `job_id`. Neither leaf retains: a
//! retained job would run again on every reconnect, and a retained result
//! would answer a question the next reader never asked.
//!
//! A PDF IS BINARY, so a job carries it as `content_b64`. The answer's
//! `result` is the SoundBase importer's own `SoundbaseParseResult`
//! (`source_format` = `Soundbase_PDF_Report`): the same channel record the
//! `wireless_soundbase` worker returns for the CSV export of the same report.
//!
//! [`handle_job`] is the whole tool and has no bus in it, so the tests below
//! run it on the fixture in `importer/fixtures/` (a copy of
//! DATATYPE.wireless.SOUNDbase's `test_data/`) with no broker anywhere near.
//! [`run_job_agent`] hands it to the runner's job loop
//! (`apk_plugin_runner::jobs::JobWorker`); the envelope is the wireless
//! kernel's (`apkaudio_wireless_kernel::job`), shared with the other wireless workers.

pub use apkaudio_wireless_soundbase_pdf as pdf_importer;
pub mod self_test;

use apkaudio_wireless_kernel::job;
use serde_json::Value;

/// The token the bus knows this worker by. The same word as `[wireless_pdf]`.
pub const MODULE: &str = "wireless_pdf";

/// The tool the result came from, for a reader that listens to several.
pub const TOOL: &str = "DATATYPE.wireless.PDF";

/// The compiled-in config, the same text the runner publishes at `/config`.
pub const CONFIG_INI: &str = include_str!("../config.ini");

/// Used when `config.ini` does not say.
const DEFAULT_MAX_JOB_BYTES: usize = 16 * 1024 * 1024;

/// `max_job_bytes` from `config.ini`, or the default.
pub fn max_job_bytes(config_ini: &str) -> usize {
    job::max_job_bytes(config_ini, MODULE, DEFAULT_MAX_JOB_BYTES)
}

/// Answer one job. Always returns a result document, never panics on input.
pub fn handle_job(payload: &[u8], max_bytes: usize) -> Value {
    match job::read_file_job(payload, TOOL, "upload.pdf", "the PDF", max_bytes) {
        Ok(file) => file.answer(TOOL, pdf_importer::parse_soundbase_pdf(&file.bytes, &file.filename)),
        Err(failure) => failure,
    }
}

/// The job loop: the runner's. Blocking; the runner starts it on its own
/// thread and exits the process if it ever returns, so Docker restarts it.
pub fn run_job_agent(host: &str, port: u16) {
    let max = max_job_bytes(CONFIG_INI);
    apk_plugin_runner::jobs::JobWorker::new(MODULE, TOOL, move |payload: &[u8]| {
        // One job taken off outgoing/job and run: this worker's packet (ROAD-3472).
        apk_plugin_runner::loopback::packet_received();
        handle_job(payload, max)
    })
        // A job is a file, and base64 grows it by a third.
        .max_packet(max / 3 * 4 + 64 * 1024)
        .describe(|a| format!("ok={} channels={}", a["ok"], a["result"]["total_channels"]))
        .run(host, port);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The manifest the binary compiles in parses, declares the loopback port
    /// and lists every key of config.ini's [wireless_pdf] that code reads (ROAD-3472).
    #[test]
    fn the_compiled_in_manifest_parses_and_reads_every_config_key() {
        let manifest = apkaudio_contracts::embedded_manifest!().expect("agent.toml parses");
        assert_eq!(manifest.name, MODULE);
        assert_eq!(manifest.kind, "worker");
        assert_eq!(manifest.telemetry_port, 8226);
        let config = apk_plugin_runner::ini_to_json(CONFIG_INI);
        let keys: Vec<&str> = config[MODULE].as_object().unwrap().keys().map(String::as_str).collect();
        assert_eq!(manifest.unread_config_keys(keys), Some(0), "a [wireless_pdf] key nothing reads");
    }
    use apkaudio_contracts::baremetal_io;
    use base64::Engine as _;
    use serde_json::json;

    const REPORT: &[u8] = include_bytes!("../importer/fixtures/SB PDF.pdf");

    fn job(body: Value) -> Vec<u8> {
        body.to_string().into_bytes()
    }

    fn b64(bytes: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(bytes)
    }

    #[test]
    fn a_pdf_job_is_parsed_and_answered_under_its_id() {
        let out = handle_job(&job(json!({"job_id": "j1", "filename": "SB PDF.pdf", "content_b64": b64(REPORT)})), 1 << 20);
        assert_eq!(out["job_id"], "j1");
        assert_eq!(out["ok"], true, "{out}");
        assert_eq!(out["result"]["total_channels"], 20);
        assert_eq!(out["result"]["status"], "success");
        assert_eq!(out["result"]["source_format"], "Soundbase_PDF_Report");
        assert_eq!(out["result"]["file_path"], "SB PDF.pdf");
        assert_eq!(out["result"]["channels"][0]["band"], "G57 - Narrowband");
        assert_eq!(out["result"]["channels"][0]["group"], "Simple Plan IEM's (8 frequencies)");
        assert_eq!(out["result"]["channels"][19]["frequency_mhz"], 603.325);
    }

    #[test]
    fn what_is_not_a_pdf_is_answered_not_ok() {
        let out = handle_job(&job(json!({"job_id": "j2", "filename": "SB PDF.csv", "text": "Group,Frequency\nA,566.6\n"})), 1 << 20);
        assert_eq!(out["job_id"], "j2");
        assert_eq!(out["ok"], false, "{out}");
        assert!(out["error"].as_str().unwrap().contains("not a PDF"));
    }

    #[test]
    fn a_bad_job_is_answered_not_dropped() {
        let out = handle_job(b"not json", 1 << 20);
        assert_eq!(out["ok"], false);
        assert!(out["job_id"].is_null());

        let out = handle_job(&job(json!({"content_b64": b64(REPORT)})), 1 << 20);
        assert_eq!(out["ok"], false, "no job_id");

        let out = handle_job(&job(json!({"job_id": "j3"})), 1 << 20);
        assert_eq!(out["job_id"], "j3");
        assert_eq!(out["ok"], false, "no content");

        let out = handle_job(&job(json!({"job_id": "j4", "content_b64": "%%%"})), 1 << 20);
        assert_eq!(out["ok"], false, "bad base64");

        let out = handle_job(&job(json!({"job_id": "j5", "content_b64": b64(&REPORT[..4000])})), 1 << 20);
        assert_eq!(out["job_id"], "j5");
        assert_eq!(out["ok"], false, "a truncated PDF: {out}");
    }

    #[test]
    fn an_oversized_job_is_refused_by_size() {
        let out = handle_job(&job(json!({"job_id": "j6", "content_b64": b64(REPORT)})), 100);
        assert_eq!(out["ok"], false);
        assert!(out["error"].as_str().unwrap().contains("at most 100"));
    }

    #[test]
    fn the_limit_comes_from_config_ini() {
        assert_eq!(max_job_bytes(CONFIG_INI), 16_777_216);
        assert_eq!(max_job_bytes("[wireless_pdf]\n"), DEFAULT_MAX_JOB_BYTES);
    }

    #[test]
    fn job_and_result_topics_are_this_modules() {
        let (job, retain) = baremetal_io::module_job(MODULE).unwrap();
        assert_eq!(job, "APK.audio/System/Protocols/wireless_pdf/outgoing/job");
        assert!(!retain);
        let (result, retain) = baremetal_io::module_result(MODULE).unwrap();
        assert_eq!(result, "APK.audio/System/Protocols/wireless_pdf/incoming/result");
        assert!(!retain);
    }

    /// The wire shape a browser reads: the SoundBase CSV worker's, field for
    /// field. Change it on purpose, together with `job::SCHEMA`.
    #[test]
    fn the_result_shape_is_pinned() {
        let out = handle_job(&job(json!({"job_id": "s", "filename": "SB PDF.pdf", "content_b64": b64(REPORT)})), 1 << 24);
        assert_eq!(out["ok"], true, "{out}");
        assert_eq!(out["schema"], job::SCHEMA);
        let keys = |v: &Value| {
            let mut k: Vec<String> = v.as_object().unwrap().keys().cloned().collect();
            k.sort();
            k
        };
        assert_eq!(keys(&out["result"]), ["channels", "file_path", "skipped_rows", "source_format", "status", "total_channels"]);
        assert_eq!(keys(&out["result"]["channels"][0]), ["band", "frequency_mhz", "group", "model", "name", "preset", "spacing"]);
        assert_eq!(job::SCHEMA, 1);
    }
}
