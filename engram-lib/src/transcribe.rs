use std::path::Path;

use crate::EngramResult;
use crate::errors::EngramError;
use crate::subtitles::Segment;
use whisper_rs::{
    FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters,
};

pub struct Transcriber {
    ctx: WhisperContext,
}

impl Transcriber {
    pub fn new(model_path: &Path) -> EngramResult<Self> {
        let ctx = WhisperContext::new_with_params(
            model_path.to_string_lossy().as_ref(),
            WhisperContextParameters::default(),
        )
        .map_err(|e| EngramError::WhisperError(e))?;

        Ok(Self { ctx })
    }

    pub fn transcribe(&self, audio: &[f32]) -> EngramResult<Vec<Segment>> {
        let mut state = self
            .ctx
            .create_state()
            .map_err(|e| EngramError::WhisperError(e))?;
        let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        state
            .full(params, audio)
            .map_err(|e| EngramError::WhisperError(e))?;

        let result = state
            .as_iter()
            .map(|segment| Segment {
                start: segment.start_timestamp(),
                end: segment.end_timestamp(),
                text: segment.to_str().unwrap().to_string(),
            })
            .collect::<Vec<Segment>>();

        Ok(result)
    }
}

pub fn extract(path: &Path) -> EngramResult<Vec<f32>> {
    let _ = path;
    let output = std::process::Command::new("ffmpeg")
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
