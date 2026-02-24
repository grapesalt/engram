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
    let mut media_paths = Vec::new();
    let mut srt_paths = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            match path.extension().and_then(|e| e.to_str()) {
                Some(ext) if ext.eq_ignore_ascii_case("srt") => {
                    srt_paths.push(path.to_path_buf());
                }
                Some(ext) if exts.contains(&ext.to_lowercase().as_str()) => {
                    media_paths.push(path.to_path_buf());
                }
                _ => {}
            }
        }
    }

    let media_files = media_paths
        .into_iter()
        .map(|media| {
            let srt = media.with_extension("srt");
            let subtitles = if srt_paths.contains(&srt) {
                Some(srt)
            } else {
                None
            };

            MediaFile { media, subtitles }
        })
        .collect();

    Ok(media_files)
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
