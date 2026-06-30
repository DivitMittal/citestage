use citestage_core::{tokenize, ParsedDocument, ScoredChunk, StageResult, StageStatus};
use std::collections::{HashMap, HashSet};

pub fn rerank(
    query: &str,
    retrieved: &[ScoredChunk],
    parsed: &[ParsedDocument],
) -> Vec<ScoredChunk> {
    let clarity_by_doc = parsed
        .iter()
        .map(|doc| (doc.document.id.clone(), doc.features.clarity_score))
        .collect::<HashMap<_, _>>();
    let query_terms = tokenize(query).into_iter().collect::<HashSet<_>>();
    let mut reranked = retrieved
        .iter()
        .map(|item| {
            let overlap = overlap_score(&query_terms, &item.chunk.text);
            let clarity = clarity_by_doc
                .get(&item.chunk.document_id)
                .copied()
                .unwrap_or(0.2);
            let mut next = item.clone();
            next.score = item.score + overlap * 0.8 + clarity * 0.7;
            next.evidence = format!(
                "rerank score {:.3}: BM25 {:.3}, overlap {:.3}, clarity {:.3}",
                next.score, item.score, overlap, clarity
            );
            next
        })
        .collect::<Vec<_>>();

    reranked.sort_by(|left, right| right.score.total_cmp(&left.score));
    for (rank, item) in reranked.iter_mut().enumerate() {
        item.rank = rank + 1;
    }
    reranked
}

pub fn rerank_stage(before: &[ScoredChunk], after: &[ScoredChunk], target_id: &str) -> StageResult {
    let before_rank = rank_of(before, target_id);
    let after_rank = rank_of(after, target_id);
    let status = match (before_rank, after_rank) {
        (_, Some(rank)) if rank <= 3 => StageStatus::Pass,
        (Some(before), Some(after)) if after <= before => StageStatus::Partial,
        (None, Some(_)) => StageStatus::Partial,
        (Some(_), Some(_)) => StageStatus::Fail,
        (_, None) => StageStatus::Fail,
    };

    let evidence = match (before_rank, after_rank) {
        (Some(before), Some(after)) => vec![format!(
            "target moved from retrieval rank {} to rerank rank {}",
            before, after
        )],
        (None, Some(after)) => vec![format!("target appeared after rerank at rank {}", after)],
        _ => vec!["target absent after rerank".into()],
    };

    StageResult::with_rank("rerank", status, after_rank, evidence)
}

fn rank_of(items: &[ScoredChunk], target_id: &str) -> Option<usize> {
    items
        .iter()
        .find(|item| item.chunk.document_id == target_id)
        .map(|item| item.rank)
}

fn overlap_score(query_terms: &HashSet<String>, text: &str) -> f32 {
    if query_terms.is_empty() {
        return 0.0;
    }

    let text_terms = tokenize(text).into_iter().collect::<HashSet<_>>();
    let matches = query_terms
        .iter()
        .filter(|term| text_terms.contains(*term))
        .count();
    matches as f32 / query_terms.len() as f32
}
