pub mod db;
pub mod errors;
pub mod index;
pub mod media;
pub mod search;
pub mod subtitles;
pub mod transcribe;

pub type EngramResult<T> = Result<T, errors::EngramError>;

pub fn get_engram_dir() -> EngramResult<std::path::PathBuf> {
    let dir = dirs::data_dir()
        .ok_or_else(|| {
            errors::EngramError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Data directory not found",
            ))
        })?
        .join("engram");

    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}
