use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::{
    EngramResult, UserConfig, errors::EngramError, search::SearchResult,
};

pub struct FFmpeg {
    path: PathBuf,
}

impl FFmpeg {
    pub fn new(config: &UserConfig) -> EngramResult<Self> {
        let ffmpeg_path = if let Some(ref path) = config.ffmpeg_bin {
            path.clone()
        } else {
            which::which("ffmpeg").map_err(|_| EngramError::FfmpegError("Ffmpeg not found in system. Please install it or add it to your path before re-running the program".to_string()))?
        };

        std::process::Command::new(&ffmpeg_path)
            .arg("-version")
            .output()
            .map_err(|_| EngramError::FfmpegError(format!("Ffmpeg not found at path: {}. Ensure it is installed before re-running the program.", ffmpeg_path.display())))?;

        Ok(Self { path: ffmpeg_path })
    }

    pub fn extract_intof32le(&self, path: &Path) -> EngramResult<Vec<f32>> {
        let output = std::process::Command::new(&self.path)
            .args(&[
                "-i",
                path.to_string_lossy().as_ref(),
                "-ar",
                "16000",
                "-ac",
                "1",
                "-f",
                "f32le",
                "-",
            ])
            .output()
            .map_err(|e| {
                EngramError::FfmpegError(format!(
                    "FFmpeg audio extraction error: {e}"
                ))
            })?;

        if !output.status.success() {
            return Err(EngramError::FfmpegError(format!(
                "FFmpeg audio extraction failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let audio = output
            .stdout
            .chunks_exact(4)
            .map(|b| {
                b.try_into().map(f32::from_le_bytes).map_err(|_| {
                    EngramError::FfmpegError("Invalid audio data".into())
                })
            })
            .collect::<EngramResult<Vec<f32>>>()?;

        Ok(audio)
    }

    pub fn generate_thumbnail(
        &self,
        video_path: &Path,
        ts: u64,
    ) -> EngramResult<PathBuf> {
        let output_path = std::env::temp_dir()
            .join(format!("engram_thumbnail_{}.jpg", uuid::Uuid::new_v4()));

        let output = std::process::Command::new(&self.path)
            .args(&[
                "-hide_banner",
                "-loglevel",
                "error",
                "-ss",
                &format!("{}", ts),
                "-noaccurate_seek",
                "-probesize",
                "32k",
                "-analyzeduration",
                "0",
                "-i",
                video_path.to_string_lossy().as_ref(),
                "-an",
                "-sn",
                "-dn",
                "-vf",
                "scale=iw/6:ih/6:flags=fast_bilinear",
                "-y",
                output_path.to_string_lossy().as_ref(),
            ])
            .output()
            .map_err(|e| {
                EngramError::FfmpegError(format!(
                    "FFmpeg thumbnail generation error: {e}"
                ))
            })?;

        if !output.status.success() {
            return Err(EngramError::FfmpegError(format!(
                "FFmpeg thumbnail generation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(output_path)
    }

    pub fn generate_thumbnail_preview(
        &self,
        file: &Path,
        start: u64,
        end: u64,
        fps: u32,
    ) -> EngramResult<PathBuf> {
        let output_path = std::env::temp_dir().join(format!(
            "engram_thumbnail_preview_{}.webp",
            uuid::Uuid::new_v4()
        ));

        let output = std::process::Command::new(&self.path)
            .args(&[
                "-hide_banner",
                "-loglevel",
                "error",
                "-ss",
                &format!("{}", start),
                "-to",
                &format!("{}", end),
                "-noaccurate_seek",
                "-probesize",
                "32k",
                "-analyzeduration",
                "0",
                "-i",
                file.to_string_lossy().as_ref(),
                "-an",
                "-sn",
                "-dn",
                "-vf",
                format!("fps={},scale=iw/6:ih/6:flags=fast_bilinear", fps)
                    .as_ref(),
                "-loop",
                "0",
                "-y",
                output_path.to_string_lossy().as_ref(),
            ])
            .output()
            .map_err(|e| {
                EngramError::FfmpegError(format!(
                    "FFmpeg thumbnail preview generation error: {e}"
                ))
            })?;

        if !output.status.success() {
            return Err(EngramError::FfmpegError(format!(
                "FFmpeg thumbnail preview generation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(output_path)
    }

    pub fn generate_thumbnail_sr(
        &self,
        search_results: &[SearchResult],
    ) -> EngramResult<Vec<PathBuf>> {
        search_results
            .par_iter()
            .map(|result| self.generate_thumbnail(&result.file, result.start))
            .collect()
    }
}
