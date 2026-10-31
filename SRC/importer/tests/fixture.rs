// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! The SoundBase fixture: `fixtures/SB PDF.pdf` must read as exactly the rows
//! of `fixtures/SB PDF.csv` (SoundBase's own CSV export of the same report),
//! in the same order, every field.

use std::path::PathBuf;

use apkaudio_wireless_soundbase_pdf::{parse_soundbase_pdf_file, SOURCE_FORMAT};

fn data(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures").join(name)
}

#[test]
fn the_pdf_reads_as_its_csv_export() {
    let r = parse_soundbase_pdf_file(data("SB PDF.pdf")).unwrap();
    assert_eq!((r.status.as_str(), r.source_format.as_str()), ("success", SOURCE_FORMAT));

    let mut csv = csv::Reader::from_path(data("SB PDF.csv")).unwrap();
    let want: Vec<csv::StringRecord> = csv.records().map(Result::unwrap).collect();
    assert_eq!(r.total_channels, want.len());
    assert_eq!(r.total_channels, 20);
    assert_eq!(r.skipped_rows, 0);
    for (got, want) in r.channels.iter().zip(&want) {
        let got_row = [&got.group, &got.model, &got.band, &got.name, &got.preset, &got.spacing];
        let want_row: Vec<&str> = want.iter().take(6).collect();
        assert_eq!(got_row.map(|s| s.as_str()).to_vec(), want_row, "row {want:?}");
        assert_eq!(got.frequency_mhz, want[6].parse::<f64>().unwrap(), "row {want:?}");
    }
}
