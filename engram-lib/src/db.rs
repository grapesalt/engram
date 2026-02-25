use std::path::Path;

use crate::EngramResult;
use rusqlite::{Connection, OptionalExtension, params};

pub struct Database {
    conn: Connection,
}

#[derive(Debug)]
pub struct IndexedFile {
    pub path: String,
    pub modified_at: i64,
    pub file_size: i64,
    pub has_subtitles: bool,
    pub transcription_model: Option<String>,
}

impl Database {
    pub fn open(path: &Path) -> EngramResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;

        // Keep writes fast: we can afford to lose the last few ms of work on
        // a crash because the data is always re-derivable from the media files.
        conn.execute_batch(
            "PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;",
        )?;

        let db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> EngramResult<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS indexed_files (
                path TEXT PRIMARY KEY,
                modified_at INTEGER NOT NULL,
                file_size INTEGER NOT NULL,
                has_subtitles BOOLEAN NOT NULL,
                transcription_model TEXT,
                indexed_at INTEGER NOT NULL DEFAULT (unixepoch())
            );
            
            CREATE TABLE IF NOT EXISTS transcriptions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT NOT NULL REFERENCES indexed_files(path) ON DELETE CASCADE,
                start_ms INTEGER NOT NULL,
                end_ms INTEGER NOT NULL,
                text TEXT NOT NULL
            ); 

            CREATE INDEX IF NOT EXISTS idx_transcriptions_file_path ON transcriptions(file_path);
            ",
        )?;

        Ok(())
    }

    pub fn upsert_file(&self, entry: &IndexedFile) -> EngramResult<()> {
        self.conn
            .execute(
                "INSERT INTO indexed_files
                    (path, modified_at, file_size, has_subtitles, transcription_model, indexed_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, unixepoch())
                 ON CONFLICT(path) DO UPDATE SET
                    modified_at         = excluded.modified_at,
                    file_size           = excluded.file_size,
                    has_subtitles       = excluded.has_subtitles,
                    transcription_model = excluded.transcription_model,
                    indexed_at          = excluded.indexed_at",
                params![
                    entry.path,
                    entry.modified_at,
                    entry.file_size,
                    entry.has_subtitles,
                    entry.transcription_model,
                ],
            )?;

        Ok(())
    }

    pub fn get_file(&self, path: &str) -> EngramResult<Option<IndexedFile>> {
        Ok(self.conn
            .query_row(
                "SELECT path, modified_at, file_size, has_subtitles, transcription_model
                 FROM indexed_files WHERE path = ?1",
                params![path],
                |row| {
                    Ok(IndexedFile {
                        path: row.get(0)?,
                        modified_at: row.get(1)?,
                        file_size: row.get(2)?,
                        has_subtitles: row.get::<_, bool>(3)?,
                        transcription_model: row.get(4)?,
                    })
                },
            ).optional()?)
    }

    pub fn remove_file(&self, path: &str) -> EngramResult<()> {
        self.conn.execute(
            "DELETE FROM indexed_files WHERE path = ?1",
            params![path],
        )?;

        Ok(())
    }

    pub fn is_upto_date(
        &self,
        path: &str,
        modified_at: i64,
        file_size: i64,
    ) -> EngramResult<bool> {
        if let Some(entry) = self.get_file(path)? {
            Ok(
                entry.modified_at == modified_at
                    && entry.file_size == file_size,
            )
        } else {
            Ok(false)
        }
    }

    pub fn all_paths(&self) -> EngramResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT path FROM indexed_files")?;

        let paths = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;

        Ok(paths)
    }

    pub fn store_segments(
        &self,
        file_path: &str,
        segments: &[crate::subtitles::Segment],
    ) -> EngramResult<()> {
        self.conn.execute(
            "DELETE FROM transcriptions WHERE file_path = ?1",
            params![file_path],
        )?;

        let mut stmt = self.conn.prepare(
            "INSERT INTO transcriptions (file_path, start_ms, end_ms, text)
                 VALUES (?1, ?2, ?3, ?4)",
        )?;

        for seg in segments {
            stmt.execute(params![file_path, seg.start, seg.end, seg.text])?;
        }

        Ok(())
    }

    pub fn load_segments(
        &self,
        file_path: &str,
    ) -> EngramResult<Option<Vec<crate::subtitles::Segment>>> {
        let mut stmt = self.conn.prepare(
            "SELECT start_ms, end_ms, text
                 FROM transcriptions
                 WHERE file_path = ?1
                 ORDER BY start_ms",
        )?;

        let rows: Vec<crate::subtitles::Segment> = stmt
            .query_map(params![file_path], |row| {
                Ok(crate::subtitles::Segment {
                    start: row.get(0)?,
                    end: row.get(1)?,
                    text: row.get(2)?,
                })
            })?
            .collect::<Result<_, _>>()?;

        if rows.is_empty() {
            Ok(None)
        } else {
            Ok(Some(rows))
        }
    }

    pub fn prune_missing(&self) -> EngramResult<usize> {
        let paths = self.all_paths()?;
        let mut removed = 0usize;

        for path in paths {
            if !std::path::Path::new(&path).exists() {
                self.remove_file(&path)?;
                removed += 1;
            }
        }

        Ok(removed)
    }
}
