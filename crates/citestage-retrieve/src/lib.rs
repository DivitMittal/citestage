use citestage_core::{ScoredChunk, StageResult, StageStatus};
use citestage_index::Bm25Index;

pub fn retrieve(index: &Bm25Index, query: &str, top_k: usize) -> Vec<ScoredChunk> {
    index.search(query, top_k)
}

pub fn retrieval_stage(results: &[ScoredChunk], target_id: &str) -> StageResult {
    let target_rank = results
        .iter()
        .find(|result| result.chunk.document_id == target_id)
        .map(|result| result.rank);
    let status = match target_rank {
        Some(rank) if rank <= 3 => StageStatus::Pass,
        Some(_) => StageStatus::Partial,
        None => StageStatus::Fail,
    };
    let evidence = match target_rank {
        Some(rank) => vec![format!("target chunk retrieved at rank {}", rank)],
        None => vec!["target project had no chunks in retrieval top-k".into()],
    };

    StageResult::with_rank("retrieve", status, target_rank, evidence)
}
