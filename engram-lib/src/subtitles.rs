use crate::{EngramResult, errors::EngramError};
use encoding_rs::*;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Segment {
    pub start: i64,
    pub end: i64,
    pub text: String,
}

pub fn parse_srt_file(path: &Path) -> EngramResult<Vec<Segment>> {
    let bytes = fs::read(path)?;

    // Process different encodings
    let content = if let Ok(text) = std::str::from_utf8(&bytes) {
        text.to_string()
    } else {
        let encodings: &[&'static Encoding] = &[
            WINDOWS_1252, // Western European subtitles
            ISO_8859_2,   // Latin-2
            UTF_16LE,     // Windows UTF-16
            UTF_16BE,     // UTF-16 Big Endian
            WINDOWS_1251, // Cyrillic
            SHIFT_JIS,    // Japanese
            GBK,          // Chinese
        ];

        let mut decoded: Option<String> = None;
        for encoding in encodings.iter() {
            let (cow, _encoding_used, had_errors) = encoding.decode(&bytes);
            if !had_errors {
                decoded = Some(cow.to_string());
                break;
            }
        }

        decoded.ok_or_else(|| {
            EngramError::SubtitleParseError(format!(
                "Could not decode subtitle file with any known encoding: {}",
                path.display()
            ))
        })?
    };

    let content = content.replace("\r\n", "\n").replace('\r', "\n");

    let mut segments = Vec::new();
    let blocks: Vec<&str> = content.split("\n\n").collect();

    for block in blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        let lines: Vec<&str> = block.lines().collect();
        if lines.len() < 3 {
            continue;
        }

        let timestamp_line = lines[1];
        let parts: Vec<&str> = timestamp_line.split(" --> ").collect();
        if parts.len() != 2 {
            continue;
        }

        let start = parse_timestamp(parts[0])?;
        let end = parse_timestamp(parts[1])?;

        let text = lines[2..].join("\n");

        segments.push(Segment { start, end, text });
    }

    Ok(segments)
}

pub fn parse_timestamp(ts: &str) -> EngramResult<i64> {
    let parts: Vec<&str> = ts.split(':').collect();
    if parts.len() != 3 {
        return Err(EngramError::SubtitleParseError(format!(
            "Invalid timestamp: {ts}"
        )));
    }

    let hours: i64 = parts[0].trim().parse().map_err(|_| {
        EngramError::SubtitleParseError(format!("Invalid hours: {}", parts[0]))
    })?;

    let minutes: i64 = parts[1].trim().parse().map_err(|_| {
        EngramError::SubtitleParseError(format!(
            "Invalid minutes: {}",
            parts[1]
        ))
    })?;

    let sec_parts: Vec<&str> = parts[2].split(',').collect();
    if sec_parts.len() != 2 {
        return Err(EngramError::SubtitleParseError(format!(
            "Invalid seconds: {}",
            parts[2]
        )));
    }

    let seconds: i64 = sec_parts[0].trim().parse().map_err(|_| {
        EngramError::SubtitleParseError(format!(
            "Invalid seconds: {}",
            sec_parts[0]
        ))
    })?;

    let milliseconds: i64 = sec_parts[1].trim().parse().map_err(|_| {
        EngramError::SubtitleParseError(format!(
            "Invalid milliseconds: {}",
            sec_parts[1]
        ))
    })?;

    Ok(hours * 3_600_000 + minutes * 60_000 + seconds * 1000 + milliseconds)
}

pub fn format_timestamp(ms: i64) -> String {
    let hours = ms / 3_600_000;
    let minutes = (ms % 3_600_000) / 60_000;
    let seconds = (ms % 60_000) / 1000;
    let milliseconds = ms % 1000;
    format!(
        "{:02}:{:02}:{:02},{:03}",
        hours, minutes, seconds, milliseconds
    )
}

pub fn generate_srt(segments: &[Segment]) -> String {
    segments
        .iter()
        .enumerate()
        .map(|(i, seg)| {
            format!(
                "{}\n{} --> {}\n{}\n",
                i + 1,
                format_timestamp(seg.start),
                format_timestamp(seg.end),
                seg.text
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
