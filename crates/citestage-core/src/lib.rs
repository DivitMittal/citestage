use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CiteStageError {
    #[error("missing target document: {0}")]
    MissingTarget(String),
    #[error("empty corpus")]
    EmptyCorpus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceKind {
    Target,
    Competitor,
    Distractor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub path: String,
    pub source_kind: SourceKind,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Corpus {
    pub target_id: String,
    pub documents: Vec<Document>,
}

impl Corpus {
    pub fn target(&self) -> Option<&Document> {
        self.documents.iter().find(|doc| doc.id == self.target_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Query {
    pub text: String,
    pub target_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Section {
    pub heading: String,
    pub level: usize,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructuralFeatures {
    pub has_top_definition: bool,
    pub max_heading_depth: usize,
    pub install_section_position: Option<usize>,
    pub use_case_term_count: usize,
    pub clarity_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedDocument {
    pub document: Document,
    pub sections: Vec<Section>,
    pub summary: String,
    pub features: StructuralFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chunk {
    pub id: String,
    pub document_id: String,
    pub source_kind: SourceKind,
    pub title: String,
    pub heading: String,
    pub text: String,
    pub token_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoredChunk {
    pub chunk: Chunk,
    pub score: f32,
    pub rank: usize,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StageStatus {
    Pass,
    Partial,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageResult {
    pub stage: String,
    pub status: StageStatus,
    pub target_present: bool,
    pub target_rank: Option<usize>,
    pub evidence: Vec<String>,
}

impl StageResult {
    pub fn pass(stage: impl Into<String>, evidence: Vec<String>) -> Self {
        Self {
            stage: stage.into(),
            status: StageStatus::Pass,
            target_present: true,
            target_rank: None,
            evidence,
        }
    }

    pub fn with_rank(
        stage: impl Into<String>,
        status: StageStatus,
        target_rank: Option<usize>,
        evidence: Vec<String>,
    ) -> Self {
        Self {
            stage: stage.into(),
            status,
            target_present: target_rank.is_some(),
            target_rank,
            evidence,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FailureClass {
    CrawlFailure,
    ParseFailure,
    IndexFailure,
    RetrievalFailure,
    RerankFailure,
    SynthesisFailure,
    CitationFailure,
    FactualityFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepairPlan {
    pub title: String,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Diagnosis {
    pub primary_failure: FailureClass,
    pub stage: String,
    pub evidence: Vec<String>,
    pub repair_plan: RepairPlan,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageTrace {
    pub query: Query,
    pub target: String,
    pub stages: Vec<StageResult>,
    pub cited_sources: Vec<String>,
    pub diagnosis: Option<Diagnosis>,
}

pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|ch: char| !ch.is_alphanumeric())
        .filter(|token| token.len() > 1)
        .map(|token| token.to_lowercase())
        .collect()
}
