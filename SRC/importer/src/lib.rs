// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! The SoundBase PDF frequency report, read into SoundBase channel records.
//!
//! ```text
//!   PDF bytes ─▶ glyphs (page, x, y, size) ─▶ lines ─▶ words
//!            ─▶ sections (group heading + header row) ─▶ one row per frequency
//! ```
//!
//! THE REPORT'S SHAPE (fixture `fixtures/SB PDF.pdf`, expected rows
//! `fixtures/SB PDF.csv`): per group, a heading line ("Simple Plan IEM's (8
//! frequencies)"), then a header row `Model Band Name Preset Spacing
//! Frequency`, then one table row per frequency. A cell may wrap onto two
//! lines ("G57 -" / "Narrowband") set above and below the row's baseline, and
//! the table continues onto the next page with no header repeated.
//!
//! SO A ROW IS ANCHORED ON ITS FREQUENCY. Every word in the Frequency column
//! that reads as a usable MHz value (the kernel's 100–3000 MHz guard) starts a
//! row; every other table word joins the nearest anchor on its page, within
//! 1.5 font sizes vertically, in the column whose header starts at or left of
//! it. Table words are those set in the header row's font size, so the running
//! title over the Frequency column ("All Your Friends 2025") is never a row.
//! Wrapped cell lines join top to bottom with a space. The columns are read
//! from the header row by name, so a report whose columns move or carry an
//! extra one still maps right; a report with no header row is an error, not
//! an empty table.
//!
//! Pure: it reads bytes and returns; it publishes nothing. The
//! `wireless_pdf` worker (`SRC/src/lib.rs`) answers each job with the result.

use std::panic::{catch_unwind, AssertUnwindSafe};

use pdf_extract::{Document, MediaBox, OutputDev, OutputError, Transform};

pub use apkaudio_wireless_soundbase::{SoundbaseChannel, SoundbaseParseResult};

/// `source_format` of every result this crate returns.
pub const SOURCE_FORMAT: &str = "Soundbase_PDF_Report";

/// The header words this dialect knows, lower-case.
const COLUMNS: [&str; 6] = ["model", "band", "name", "preset", "spacing", "frequency"];

/// One glyph where the page put it: `y` grows downward from the page top.
#[derive(Debug, Clone)]
struct Glyph {
    page: u32,
    x: f64,
    end: f64,
    y: f64,
    size: f64,
    ch: String,
}

/// pdf-extract's output device, collecting positioned glyphs instead of text.
struct Collect {
    page: u32,
    flip: Transform,
    glyphs: Vec<Glyph>,
}

impl OutputDev for Collect {
    fn begin_page(&mut self, n: u32, mb: &MediaBox, _: Option<(f64, f64, f64, f64)>) -> Result<(), OutputError> {
        self.page = n;
        self.flip = Transform::row_major(1., 0., 0., -1., 0., mb.ury - mb.lly);
        Ok(())
    }
    fn end_page(&mut self) -> Result<(), OutputError> {
        Ok(())
    }
    fn output_character(&mut self, trm: &Transform, width: f64, _spacing: f64, font_size: f64, ch: &str) -> Result<(), OutputError> {
        let p = trm.post_transform(&self.flip);
        // The font size through the text matrix, as pdf-extract's own text output measures it.
        let (vx, vy) = (font_size * (trm.m11 + trm.m21), font_size * (trm.m12 + trm.m22));
        let size = (vx * vy).abs().sqrt();
        self.glyphs.push(Glyph { page: self.page, x: p.m31, end: p.m31 + width * size, y: p.m32, size, ch: ch.to_string() });
        Ok(())
    }
    fn begin_word(&mut self) -> Result<(), OutputError> {
        Ok(())
    }
    fn end_word(&mut self) -> Result<(), OutputError> {
        Ok(())
    }
    fn end_line(&mut self) -> Result<(), OutputError> {
        Ok(())
    }
}

/// A run of glyphs with no gap in it, on one line.
#[derive(Debug, Clone)]
struct Word {
    page: u32,
    x: f64,
    y: f64,
    size: f64,
    text: String,
}

/// The words of one visual line, left to right.
#[derive(Debug, Clone)]
struct Line {
    page: u32,
    y: f64,
    words: Vec<Word>,
}

impl Line {
    fn text(&self) -> String {
        self.words.iter().map(|w| w.text.as_str()).collect::<Vec<_>>().join(" ")
    }
}

fn glyphs(pdf: &[u8]) -> Result<Vec<Glyph>, String> {
    let mut doc = Document::load_mem(pdf).map_err(|e| format!("not a readable PDF: {e}"))?;
    if doc.is_encrypted() {
        doc.decrypt("").map_err(|e| format!("the PDF is encrypted: {e}"))?;
    }
    let mut out = Collect { page: 0, flip: Transform::identity(), glyphs: Vec::new() };
    // pdf-extract panics on some malformed fonts; a job is answered, not a crash.
    match catch_unwind(AssertUnwindSafe(|| pdf_extract::output_doc(&doc, &mut out))) {
        Ok(Ok(())) => Ok(out.glyphs),
        Ok(Err(e)) => Err(format!("PDF text extraction failed: {e}")),
        Err(_) => Err("PDF text extraction failed (the extractor panicked on this file)".to_string()),
    }
}

/// Glyphs into lines (same page, baselines within 0.4 font size) and lines
/// into words (split at a space glyph or a gap wider than 0.3 font size).
fn lines(mut glyphs: Vec<Glyph>) -> Vec<Line> {
    glyphs.retain(|g| g.size > 0.0 && g.x.is_finite() && g.y.is_finite());
    glyphs.sort_by(|a, b| a.page.cmp(&b.page).then(a.y.total_cmp(&b.y)).then(a.x.total_cmp(&b.x)));
    let mut rows: Vec<Vec<Glyph>> = Vec::new();
    for g in glyphs {
        match rows.last_mut() {
            Some(row) if row[0].page == g.page && (g.y - row[0].y).abs() <= 0.4 * g.size => row.push(g),
            _ => rows.push(vec![g]),
        }
    }
    rows.into_iter()
        .filter_map(|mut row| {
            row.sort_by(|a, b| a.x.total_cmp(&b.x));
            let (page, y) = (row[0].page, row[0].y);
            let mut words: Vec<Word> = Vec::new();
            let mut last_end = f64::NEG_INFINITY;
            let mut open = false;
            for g in row {
                if g.ch.trim().is_empty() {
                    open = false;
                    last_end = g.end;
                    continue;
                }
                if !open || g.x - last_end > 0.3 * g.size {
                    words.push(Word { page, x: g.x, y: g.y, size: g.size, text: String::new() });
                }
                words.last_mut().unwrap().text.push_str(&g.ch);
                open = true;
                last_end = g.end;
            }
            (!words.is_empty()).then_some(Line { page, y, words })
        })
        .collect()
}

/// The column starts of a header row, when this line is one: it must name
/// Frequency and at least one other known column.
fn header_columns(line: &Line) -> Option<Vec<(&'static str, f64)>> {
    let cols: Vec<(&'static str, f64)> = line
        .words
        .iter()
        .filter_map(|w| COLUMNS.iter().find(|c| w.text.eq_ignore_ascii_case(c)).map(|c| (*c, w.x)))
        .collect();
    (cols.len() >= 2 && cols.iter().any(|(c, _)| *c == "frequency")).then_some(cols)
}

/// "Simple Plan IEM's (8 frequencies)" — SoundBase's group heading.
fn is_group_heading(text: &str) -> bool {
    let t = text.trim().to_ascii_lowercase();
    let Some(open) = t.rfind('(') else { return false };
    let inner = t[open + 1..].trim_end_matches(')');
    let mut parts = inner.split_whitespace();
    matches!(
        (parts.next().map(|n| n.parse::<u32>().is_ok()), parts.next(), parts.next()),
        (Some(true), Some("frequency" | "frequencies"), None)
    ) && t.ends_with(')')
}

/// One section of the table: the group it belongs to and its columns.
struct Section {
    group: String,
    columns: Vec<(&'static str, f64)>,
    /// The header row's font size: table text is set in it, a page title is not.
    size: f64,
}

impl Section {
    /// The column a word starting at `x` falls in: the rightmost header that
    /// starts at or left of it (2 pt of slack for kerning).
    fn column_of(&self, x: f64) -> Option<&'static str> {
        self.columns.iter().filter(|(_, cx)| *cx <= x + 2.0).max_by(|a, b| a.1.total_cmp(&b.1)).map(|(c, _)| *c)
    }
}

/// A row in the making: its frequency word, then the words that join it.
struct Row {
    section: usize,
    page: u32,
    y: f64,
    size: f64,
    frequency: f64,
    cells: Vec<(&'static str, f64, f64, String)>, // column, y, x, text
}

/// Parse a SoundBase PDF report. No side effect; `source_name` is echoed back
/// as `file_path`.
pub fn parse_soundbase_pdf(pdf: &[u8], source_name: &str) -> Result<SoundbaseParseResult, String> {
    if !pdf.starts_with(b"%PDF-") {
        return Err("not a PDF (the file does not start with %PDF-)".to_string());
    }
    let lines = lines(glyphs(pdf)?);

    let mut sections: Vec<Section> = Vec::new();
    let mut group = String::new();
    // A heading seen since the last header row: this header's group.
    let mut heading_fresh = false;
    // (section, word) for every word that may belong to a row.
    let mut table: Vec<(usize, Word)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if let Some(columns) = header_columns(line) {
            // A heading not in the "(N frequencies)" form is still the line
            // right above its header row, on the same page.
            if !heading_fresh {
                if let Some(prev) = i.checked_sub(1).map(|p| &lines[p]).filter(|p| p.page == line.page && header_columns(p).is_none()) {
                    group = prev.text();
                    table.retain(|(_, w)| !(w.page == prev.page && w.y == prev.y));
                }
            }
            heading_fresh = false;
            sections.push(Section { group: group.clone(), columns, size: line.words[0].size });
            continue;
        }
        if is_group_heading(&line.text()) {
            group = line.text();
            heading_fresh = true;
            continue;
        }
        if let Some(s) = sections.len().checked_sub(1) {
            // Only text in the table's own size: the running title and date
            // ("All Your Friends 2025") sit over the Frequency column too.
            let size = sections[s].size;
            table.extend(line.words.iter().filter(|w| (w.size - size).abs() <= 0.15 * size).cloned().map(|w| (s, w)));
        }
    }
    if sections.is_empty() {
        return Err("no header row (Model … Frequency) in the PDF: not a SoundBase frequency report".to_string());
    }

    // Anchors: usable frequencies in the Frequency column.
    let mut rows: Vec<Row> = Vec::new();
    let mut rest: Vec<(usize, Word)> = Vec::new();
    for (s, w) in table {
        let freq = (sections[s].column_of(w.x) == Some("frequency"))
            .then(|| apkaudio_wireless_kernel::usable_mhz_text(&w.text))
            .flatten();
        match freq {
            Some(mhz) => rows.push(Row { section: s, page: w.page, y: w.y, size: w.size, frequency: mhz, cells: vec![("frequency", w.y, w.x, w.text)] }),
            None => rest.push((s, w)),
        }
    }

    // Every other word joins the nearest anchor of its section on its page.
    let mut skipped_rows = 0usize;
    for (s, w) in rest {
        let nearest = rows
            .iter_mut()
            .filter(|r| r.section == s && r.page == w.page && (r.y - w.y).abs() <= 1.5 * r.size.max(w.size))
            .min_by(|a, b| (a.y - w.y).abs().total_cmp(&(b.y - w.y).abs()));
        match (nearest, sections[s].column_of(w.x)) {
            (Some(row), Some(col)) => row.cells.push((col, w.y, w.x, w.text)),
            // A Frequency cell that is no usable carrier: a row the report
            // carried and the importer could not read.
            (None, Some("frequency")) if w.text.trim().parse::<f64>().is_ok() => skipped_rows += 1,
            _ => {} // title, date, footer: not table text
        }
    }

    rows.sort_by(|a, b| a.page.cmp(&b.page).then(a.y.total_cmp(&b.y)));
    let channels: Vec<SoundbaseChannel> = rows
        .into_iter()
        .map(|mut r| {
            r.cells.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.2.total_cmp(&b.2)));
            let cell = |col: &str| r.cells.iter().filter(|c| c.0 == col).map(|c| c.3.as_str()).collect::<Vec<_>>().join(" ");
            SoundbaseChannel {
                group: sections[r.section].group.clone(),
                model: cell("model"),
                band: cell("band"),
                name: cell("name"),
                preset: cell("preset"),
                spacing: cell("spacing"),
                frequency_mhz: r.frequency,
            }
        })
        .collect();

    Ok(SoundbaseParseResult {
        status: apkaudio_wireless_kernel::import_status(channels.len()),
        source_format: SOURCE_FORMAT.to_string(),
        file_path: source_name.to_string(),
        total_channels: channels.len(),
        skipped_rows,
        channels,
    })
}

/// Read a file and parse it.
pub fn parse_soundbase_pdf_file(path: impl AsRef<std::path::Path>) -> Result<SoundbaseParseResult, String> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
    parse_soundbase_pdf(&bytes, &path.to_string_lossy())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_group_heading_is_recognised_by_its_count() {
        assert!(is_group_heading("Simple Plan IEM's (8 frequencies)"));
        assert!(is_group_heading("Spare (1 frequency)"));
        assert!(!is_group_heading("Main Stage Direct Support Frequency Report"));
        assert!(!is_group_heading("Vox (lead)"));
    }

    #[test]
    fn what_is_not_a_pdf_is_refused() {
        assert!(parse_soundbase_pdf(b"Group,Model\n", "x.csv").unwrap_err().contains("not a PDF"));
        assert!(parse_soundbase_pdf(b"%PDF-1.4 truncated", "x.pdf").is_err());
    }
}
