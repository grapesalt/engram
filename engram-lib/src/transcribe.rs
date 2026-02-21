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
