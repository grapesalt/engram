use std::fs;
use std::path::{Path, PathBuf};
use tantivy::{
    Index, IndexReader, IndexWriter, ReloadPolicy, collector::TopDocs, doc,
    query::QueryParser, schema::*,
};

use crate::index::MediaFile;
use crate::subtitles::parse_srt_file;
use crate::{EngramResult, errors::EngramError};

pub struct SearchIndex {
    index: Index,
    writer: IndexWriter,
    reader: IndexReader,
    file_field: Field,
    text_field: Field,
    start_field: Field,
    end_field: Field,
    segment_id_field: Field,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub file: PathBuf,
    pub start: u64,
    pub end: u64,
    pub text: String,
    pub score: f32,
}

impl SearchIndex {
    pub fn create(path: &Path) -> EngramResult<Self> {
        fs::create_dir_all(path)?;

        let mut schema_builder = Schema::builder();

        let path_field = schema_builder.add_text_field("path", STRING | STORED);
        let text_field = schema_builder.add_text_field("text", TEXT | STORED);
        let start_field =
            schema_builder.add_u64_field("start_ms", INDEXED | STORED);
        let end_field =
            schema_builder.add_u64_field("end_ms", INDEXED | STORED);
        let segment_id_field =
            schema_builder.add_u64_field("segment_id", INDEXED | STORED);

        let schema = schema_builder.build();

        let index = Index::create_in_dir(path, schema.clone())?;

        let writer = index.writer(100_000_000)?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        Ok(Self {
            index,
            writer,
            reader,
            file_field: path_field,
            text_field,
            start_field,
            end_field,
            segment_id_field,
        })
    }

    pub fn open(index_path: &Path) -> EngramResult<Self> {
        let index = Index::open_in_dir(index_path)?;

        let schema = index.schema();

        let path_field = schema.get_field("path")?;
        let text_field = schema.get_field("text")?;
        let start_field = schema.get_field("start_ms")?;
        let end_field = schema.get_field("end_ms")?;
        let segment_id_field = schema.get_field("segment_id")?;

        let writer = index.writer(100_000_000)?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        Ok(Self {
            index,
            writer,
            reader,
            file_field: path_field,
            text_field,
            start_field,
            end_field,
            segment_id_field,
        })
    }

    pub fn add_media_file(&mut self, file: &MediaFile) -> EngramResult<()> {
        let subtitle_path = file.subtitles.as_ref().ok_or_else(|| {
            EngramError::SearchError(
                "MediaFile has no subtitles associated.".into(),
            )
        })?;

        let segments = parse_srt_file(subtitle_path)?;

        if segments.is_empty() {
            return Err(EngramError::SubtitleParseError(format!(
                "No segments parsed from {:?}",
                subtitle_path
            )));
        }

        let media_path = file.media.to_string_lossy().to_string();

        for (idx, segment) in segments.iter().enumerate() {
            let doc = doc!(
                self.file_field => media_path.clone(),
                self.text_field => segment.text.clone(),
                self.start_field => segment.start,
                self.end_field => segment.end,
                self.segment_id_field => idx as u64,
            );

            self.writer.add_document(doc)?;
        }

        Ok(())
    }

    pub fn remove_media_file(&mut self, path: &Path) {
        let media_path_str = path.to_string_lossy().to_string();

        self.writer.delete_term(tantivy::Term::from_field_text(
            self.file_field,
            &media_path_str,
        ));
    }

    pub fn update_media_file(&mut self, file: &MediaFile) -> EngramResult<()> {
        self.remove_media_file(&file.media);
        self.add_media_file(file)?;

        Ok(())
    }

    pub fn commit(&mut self) -> EngramResult<()> {
        self.writer.commit()?;
        self.reader.reload()?;

        Ok(())
    }

    pub fn has_media_file(&self, path: &Path) -> EngramResult<bool> {
        let searcher = self.reader.searcher();
        let media_path = path.to_string_lossy().to_string();

        let term = tantivy::Term::from_field_text(self.file_field, &media_path);
        let term_query = tantivy::query::TermQuery::new(
            term,
            tantivy::schema::IndexRecordOption::Basic,
        );

        let top_docs = searcher.search(&term_query, &TopDocs::with_limit(1))?;

        Ok(!top_docs.is_empty())
    }

    pub fn search(
        &self,
        query: &str,
        limit: usize,
    ) -> EngramResult<Vec<SearchResult>> {
        let searcher = self.reader.searcher();

        let query_parser =
            QueryParser::for_index(&self.index, vec![self.text_field]);

        let formatted_query = if query.starts_with('"') && query.ends_with('"')
        {
            query.to_string()
        } else {
            format!("\"{}\"", query.replace('"', "\\\""))
        };

        let query =
            query_parser.parse_query(&formatted_query).map_err(|e| {
                EngramError::SearchError(format!("Failed to parse query: {e}"))
            })?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();

        for (score, doc_address) in top_docs {
            let retrieved_doc =
                searcher.doc::<tantivy::TantivyDocument>(doc_address)?;

            let file = retrieved_doc
                .get_first(self.file_field)
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    EngramError::SearchError(
                        "Missing file field in result".into(),
                    )
                })?;

            let text = retrieved_doc
                .get_first(self.text_field)
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    EngramError::SearchError(
                        "Missing text field in result".into(),
                    )
                })?;

            let start = retrieved_doc
                .get_first(self.start_field)
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    EngramError::SearchError(
                        "Missing start field in result".into(),
                    )
                })?;

            let end = retrieved_doc
                .get_first(self.end_field)
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    EngramError::SearchError(
                        "Missing end field in result".into(),
                    )
                })?;

            results.push(SearchResult {
                file: PathBuf::from(file),
                start,
                end,
                text: text.to_string(),
                score,
            });
        }

        Ok(results)
    }
}
