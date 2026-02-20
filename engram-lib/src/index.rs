use rayon::prelude::*;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::{EngramResult, errors::EngramError};

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub media: PathBuf,
    pub subtitles: Option<PathBuf>,
}

fn get_duration(path: &Path) -> EngramResult<f64> {
    let output = std::process::Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path.to_string_lossy().as_ref(),
        ])
        .output()
        .map_err(|e| {
            EngramError::FfmpegError(format!("FFprobe execution error: {e}"))
        })?;

    if !output.status.success() {
        return Err(EngramError::FfmpegError(format!(
            "FFprobe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let duration_str = String::from_utf8_lossy(&output.stdout);
    duration_str
        .trim()
        .parse::<f64>()
        .map_err(|e| EngramError::FfmpegError(format!("Invalid duration: {e}")))
}

pub fn get_files(dir: &Path, exts: &[&str]) -> EngramResult<Vec<MediaFile>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if exts.contains(&ext.to_lowercase().as_str()) {
                    if let Ok(duration) = get_duration(path) {
                        if duration >= 10.0 {
                            let srt_path = path.with_extension("srt");
                            let srt_path = if srt_path.exists() {
                                Some(srt_path)
                            } else {
                                None
                            };

                            files.push(MediaFile {
                                media: path.to_path_buf(),
                                subtitles: srt_path,
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(files)
}

pub fn get_files_par(
    dirs: &[PathBuf],
    exts: &[&str],
) -> EngramResult<Vec<MediaFile>> {
    let files = dirs
        .par_iter()
        .map(|dir| get_files(dir, exts))
        .collect::<EngramResult<Vec<Vec<MediaFile>>>>()?
        .into_iter()
        .flatten()
        .collect();
    Ok(files)
}
