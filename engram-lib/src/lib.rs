pub mod errors;
pub mod index;
pub mod media;
pub mod search;
pub mod subtitles;
pub mod transcribe;

pub type EngramResult<T> = Result<T, errors::EngramError>;
