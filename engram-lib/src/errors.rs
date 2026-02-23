use std::io;
use thiserror::Error;
use whisper_rs;

#[derive(Debug, Error)]
pub enum EngramError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Whisper error: {0}")]
    WhisperError(#[from] whisper_rs::WhisperError),
    #[error("Subtitle parse error: {0}")]
    SubtitleParseError(String),
    #[error("Walk dir error: {0}")]
    WalkDirError(#[from] walkdir::Error),
    #[error("Media error: {0}")]
    MediaError(String),
    #[error("FFmpeg error: {0}")]
    FFmpegError(#[from] ffmpeg_next::Error),
    #[error("Tantivy error: {0}")]
    TantivyError(#[from] tantivy::TantivyError),
    #[error("Search error: {0}")]
    SearchError(String),
    #[error("HTTP error: {0}")]
    HttpError(#[from] ureq::Error),
}
