use citestage_core::{tokenize, Chunk, ParsedDocument, ScoredChunk};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Bm25Index {
    chunks: Vec<Chunk>,
    doc_freqs: HashMap<String, usize>,
    term_freqs: Vec<HashMap<String, usize>>,
    avg_len: f32,
}

pub fn chunks_from_parsed(parsed: &[ParsedDocument]) -> Vec<Chunk> {
    parsed
        .iter()
        .flat_map(chunks_for_document)
        .collect::<Vec<_>>()
}

fn chunks_for_document(parsed: &ParsedDocument) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    for (index, section) in parsed.sections.iter().enumerate() {
        let tokens = tokenize(&section.text);
        if tokens.is_empty() {
            continue;
        }

        for (window_index, window) in tokens.chunks(90).enumerate() {
            chunks.push(Chunk {
                id: format!("{}-{}-{}", parsed.document.id, index, window_index),
                document_id: parsed.document.id.clone(),
                source_kind: parsed.document.source_kind.clone(),
                title: parsed.document.title.clone(),
                heading: section.heading.clone(),
                text: window.join(" "),
                token_count: window.len(),
            });
        }
    }

    if chunks.is_empty() {
        let tokens = tokenize(&parsed.document.content);
        chunks.push(Chunk {
            id: format!("{}-0-0", parsed.document.id),
            document_id: parsed.document.id.clone(),
            source_kind: parsed.document.source_kind.clone(),
            title: parsed.document.title.clone(),
            heading: "Overview".into(),
            text: tokens
                .iter()
                .take(90)
                .cloned()
                .collect::<Vec<_>>()
                .join(" "),
            token_count: tokens.len().min(90),
        });
    }

    chunks
}

impl Bm25Index {
    pub fn new(chunks: Vec<Chunk>) -> Self {
        let mut doc_freqs: HashMap<String, usize> = HashMap::new();
        let mut term_freqs = Vec::new();
        let mut total_len = 0usize;

        for chunk in &chunks {
            let tokens = tokenize(&chunk.text);
            total_len += tokens.len();
            let mut freqs: HashMap<String, usize> = HashMap::new();
            let mut seen: HashSet<String> = HashSet::new();

            for token in tokens {
                *freqs.entry(token.clone()).or_insert(0) += 1;
                seen.insert(token);
            }

            for token in seen {
                *doc_freqs.entry(token).or_insert(0) += 1;
            }

            term_freqs.push(freqs);
        }

        let avg_len = if chunks.is_empty() {
            0.0
        } else {
            total_len as f32 / chunks.len() as f32
        };

        Self {
            chunks,
            doc_freqs,
            term_freqs,
            avg_len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn search(&self, query: &str, top_k: usize) -> Vec<ScoredChunk> {
        if self.chunks.is_empty() || top_k == 0 {
            return Vec::new();
        }

        let query_terms = tokenize(query);
        let mut scored = self
            .chunks
            .iter()
            .enumerate()
            .filter_map(|(index, chunk)| {
                let score = self.score_chunk(index, &query_terms);
                if score <= 0.0 {
                    return None;
                }
                Some(ScoredChunk {
                    chunk: chunk.clone(),
                    score,
                    rank: 0,
                    evidence: format!("BM25 score {:.3} for heading '{}'", score, chunk.heading),
                })
            })
            .collect::<Vec<_>>();

        scored.sort_by(|left, right| right.score.total_cmp(&left.score));
        scored.truncate(top_k);
        for (rank, item) in scored.iter_mut().enumerate() {
            item.rank = rank + 1;
        }
        scored
    }

    fn score_chunk(&self, index: usize, query_terms: &[String]) -> f32 {
        let k1 = 1.5;
        let b = 0.75;
        let chunk_len = self.chunks[index].token_count.max(1) as f32;
        let corpus_size = self.chunks.len() as f32;
        let mut score = 0.0;

        for term in query_terms {
            let Some(tf) = self.term_freqs[index].get(term).copied() else {
                continue;
            };
            let df = self.doc_freqs.get(term).copied().unwrap_or(0) as f32;
            let idf = ((corpus_size - df + 0.5) / (df + 0.5) + 1.0).ln();
            let tf = tf as f32;
            let norm = tf + k1 * (1.0 - b + b * chunk_len / self.avg_len.max(1.0));
            score += idf * (tf * (k1 + 1.0)) / norm;
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use citestage_core::{Document, ParsedDocument, Section, SourceKind, StructuralFeatures};

    #[test]
    fn ranks_matching_chunk() {
        let parsed = vec![ParsedDocument {
            document: Document {
                id: "target".into(),
                title: "Target".into(),
                path: "README.md".into(),
                source_kind: SourceKind::Target,
                content: "citation debugger".into(),
            },
            sections: vec![Section {
                heading: "Overview".into(),
                level: 1,
                text: "citation debugger pipeline".into(),
            }],
            summary: "citation debugger".into(),
            features: StructuralFeatures {
                has_top_definition: true,
                max_heading_depth: 1,
                install_section_position: None,
                use_case_term_count: 1,
                clarity_score: 0.8,
            },
        }];
        let index = Bm25Index::new(chunks_from_parsed(&parsed));
        let results = index.search("citation debugger", 3);
        assert_eq!(results[0].chunk.document_id, "target");
    }
}
