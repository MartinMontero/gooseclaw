//! gooseclaw-knowledge — knowledge store + Alfred MCP-vault federation.
//!
//! Phase 1: pure RRF fusion + the store trait seam. The libSQL binding
//! (embedded, no server; FTS5 + native vector — libsql crate 0.10.0 as of
//! 2026-10-27, docs.turso.tech) lands in Phase 2 with Alfred federation.
//!
//! Resolved decision (canon): persistence = libSQL/FTS5 + native vector, RRF
//! on top. Alfred remains the system-of-record and SOLE WRITER of the vault;
//! GooseClaw reads free within the path root and writes only through
//! Alfred's MCP tools (single-writer + optimistic concurrency). Phase 2+.

#![forbid(unsafe_code)]

/// Reciprocal Rank Fusion score for one document given its 1-based ranks.
/// rrf = sum of 1 / (k + rank_i). Standard constant k = 60 (IronClaw memory
/// uses RRF; see NOTICE).
pub fn rrf_score(ranks: impl IntoIterator<Item = usize>, k: f64) -> f64 {
    ranks
        .into_iter()
        .filter(|r| *r > 0)
        .map(|r| 1.0 / (k + r as f64))
        .sum()
}

/// Default RRF constant.
pub const RRF_K: f64 = 60.0;

/// One fused search hit.
#[derive(Debug, Clone, PartialEq)]
pub struct FusedHit {
    pub doc_id: String,
    pub score: f64,
}

/// Fuse FTS5 and vector result lists (doc_id, 1-based rank) via RRF.
/// Pure function; the store implementation supplies the rank lists.
pub fn fuse(fts_ranks: &[(&str, usize)], vec_ranks: &[(&str, usize)]) -> Vec<FusedHit> {
    let mut acc: std::collections::BTreeMap<String, f64> = std::collections::BTreeMap::new();
    for (id, rank) in fts_ranks.iter().chain(vec_ranks.iter()) {
        *acc.entry((*id).to_string()).or_default() += rrf_score([*rank], RRF_K);
    }
    let mut hits: Vec<FusedHit> = acc
        .into_iter()
        .map(|(doc_id, score)| FusedHit { doc_id, score })
        .collect();
    hits.sort_by(|a, b| b.score.total_cmp(&a.score));
    hits
}

/// The persistence seam. Implementation (Phase 2): embedded libSQL — a local
/// file DB, no server, no network bind (sovereignty invariant).
pub trait KnowledgeStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Full-text (FTS5) ranked candidates: (doc_id, 1-based rank).
    fn fts_candidates(&mut self, query: &str) -> Result<Vec<(String, usize)>, Self::Error>;
    /// Native-vector ranked candidates: (doc_id, 1-based rank).
    fn vector_candidates(&mut self, embedding: &[f32])
    -> Result<Vec<(String, usize)>, Self::Error>;

    /// Default hybrid query: RRF over both candidate lists.
    fn hybrid(&mut self, query: &str, embedding: &[f32]) -> Result<Vec<FusedHit>, Self::Error> {
        let fts = self.fts_candidates(query)?;
        let vec = self.vector_candidates(embedding)?;
        let fts_refs: Vec<(&str, usize)> = fts.iter().map(|(i, r)| (i.as_str(), *r)).collect();
        let vec_refs: Vec<(&str, usize)> = vec.iter().map(|(i, r)| (i.as_str(), *r)).collect();
        Ok(fuse(&fts_refs, &vec_refs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rrf_math_is_exact() {
        assert!((rrf_score([1], RRF_K) - 1.0 / 61.0).abs() < 1e-12);
        assert!((rrf_score([2, 2], RRF_K) - 2.0 / 62.0).abs() < 1e-12);
        assert_eq!(rrf_score([0], RRF_K), 0.0, "rank 0 is not a rank");
    }

    #[test]
    fn fuse_prefers_docs_ranked_in_both_lists() {
        let hits = fuse(&[("a", 1), ("b", 2)], &[("b", 1), ("c", 3)]);
        assert_eq!(hits[0].doc_id, "b", "present in both lists must win");
        assert!(hits[0].score > hits[1].score);
    }
}
