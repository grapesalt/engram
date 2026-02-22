use crate::EngramResult;
use crate::errors::EngramError;
use crate::subtitles::Segment;
use whisper_rs::{
    FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters,
};

// NOTE: Should there be options to select the model?
//       The embeding may cause the binary to be way too large.
//       Maybe just the base model, should be good enough for most usecases.
pub enum TranscriberModel {
    Tiny,
    Base,
    Small,
    Medium,
    Large,
}

pub struct Transcriber {
    ctx: WhisperContext,
    _model: TranscriberModel,
}

impl Transcriber {
    pub fn new(model: TranscriberModel) -> EngramResult<Self> {
        // TODO: actually add the models
        let model_path = match model {
            TranscriberModel::Tiny => "models/ggml-tiny.bin",
            TranscriberModel::Base => "models/ggml-base.bin",
            TranscriberModel::Small => "models/ggml-small.bin",
            TranscriberModel::Medium => "models/ggml-medium.bin",
            TranscriberModel::Large => "models/ggml-large.bin",
        };

        let ctx = WhisperContext::new_with_params(
            model_path,
            WhisperContextParameters::default(),
        )
        .map_err(|e| EngramError::WhisperError(e))?;

        Ok(Self { ctx, _model: model })
    }

    pub fn transcribe(&self, audio: &[f32]) -> EngramResult<Box<[Segment]>> {
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

        Ok(result.into())
    }
}
