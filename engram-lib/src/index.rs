use rayon::prelude::*;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::EngramResult;

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub media: PathBuf,
    pub subtitles: Option<PathBuf>,
}

pub fn get_files(dir: &Path, exts: &[&str]) -> EngramResult<Vec<MediaFile>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if exts.contains(&ext.to_lowercase().as_str()) {
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
