use std::path::PathBuf;

pub mod errors;
pub mod ffmpeg;
pub mod index;
pub mod search;
pub mod subtitles;
pub mod transcribe;

pub type EngramResult<T> = Result<T, errors::EngramError>;

pub struct UserConfig {
    pub ffmpeg_bin: Option<PathBuf>,
    pub whisper_model: Option<String>,
}

impl UserConfig {
    pub fn new(
        ffmpeg_bin: Option<PathBuf>,
        whisper_model: Option<String>,
    ) -> Self {
        let whisper_model = if whisper_model.is_none() {
            Some("base".to_string())
        } else {
            whisper_model
        };

        Self {
            ffmpeg_bin,
            whisper_model,
        }
    }
}
