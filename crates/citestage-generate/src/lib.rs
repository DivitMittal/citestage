use citestage_core::{ScoredChunk, StageResult, StageStatus};

#[derive(Debug, Clone)]
pub struct GeneratedAnswer {
    pub answer: String,
    pub cited_sources: Vec<String>,
}

pub fn synthesize(query: &str, chunks: &[ScoredChunk], max_citations: usize) -> GeneratedAnswer {
    let mut cited_sources = Vec::new();
    let mut parts = Vec::new();

    for chunk in chunks.iter().take(max_citations) {
        if !cited_sources.contains(&chunk.chunk.document_id) {
            cited_sources.push(chunk.chunk.document_id.clone());
        }
        parts.push(format!(
            "{}: {} [{}]",
            chunk.chunk.title,
            first_words(&chunk.chunk.text, 24),
            chunk.chunk.document_id
        ));
    }

    let answer = if parts.is_empty() {
        format!("No local evidence found for query: {}", query)
    } else {
        format!(
            "For '{}', local evidence suggests: {}",
            query,
            parts.join(" ")
        )
    };

    GeneratedAnswer {
        answer,
        cited_sources,
    }
}

pub fn synthesize_stage(answer: &GeneratedAnswer, target_id: &str) -> StageResult {
    if answer.answer.starts_with("No local evidence") {
        return StageResult::with_rank(
            "synthesize",
            StageStatus::Fail,
            None,
            vec!["generator had no evidence to synthesize".into()],
        );
    }

    StageResult::pass(
        "synthesize",
        vec!["deterministic extractive answer produced".into()],
    )
    .with_target_presence(answer.cited_sources.contains(&target_id.to_string()))
}

pub fn citation_stage(answer: &GeneratedAnswer, target_id: &str) -> StageResult {
    let target_rank = answer
        .cited_sources
        .iter()
        .position(|source| source == target_id)
        .map(|index| index + 1);
    let status = if target_rank.is_some() {
        StageStatus::Pass
    } else {
        StageStatus::Fail
    };
    let evidence = if let Some(rank) = target_rank {
        vec![format!("target cited at citation position {}", rank)]
    } else {
        vec![format!(
            "target not cited; cited sources were: {}",
            answer.cited_sources.join(", ")
        )]
    };

    StageResult::with_rank("cite", status, target_rank, evidence)
}

trait TargetPresence {
    fn with_target_presence(self, present: bool) -> Self;
}

impl TargetPresence for StageResult {
    fn with_target_presence(mut self, present: bool) -> Self {
        self.target_present = present;
        self
    }
}

fn first_words(text: &str, count: usize) -> String {
    text.split_whitespace()
        .take(count)
        .collect::<Vec<_>>()
        .join(" ")
}
