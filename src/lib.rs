//! Minimal deterministic local context selector for the sprint prototype.
//! This is a lexical baseline, not Moss integration or semantic search.

pub mod policy;
pub mod replay;
pub mod runtime;

use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextRecord {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedRecord {
    pub id: String,
    pub score: usize,
    pub text: String,
}

fn tokenize(input: &str) -> BTreeSet<String> {
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|term| !term.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// Rank records by distinct query terms found in each record.
/// Matching uses whole tokens. Ties are resolved by record ID.
pub fn select_context(query: &str, records: &[ContextRecord], limit: usize) -> Vec<RankedRecord> {
    if limit == 0 {
        return Vec::new();
    }
    let terms = tokenize(query);
    if terms.is_empty() {
        return Vec::new();
    }

    let mut ranked: Vec<RankedRecord> = records
        .iter()
        .filter_map(|record| {
            let record_terms = tokenize(&record.text);
            let score = terms.intersection(&record_terms).count();
            (score > 0).then(|| RankedRecord {
                id: record.id.clone(),
                score,
                text: record.text.clone(),
            })
        })
        .collect();

    ranked.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
    ranked.truncate(limit);
    ranked
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus() -> Vec<ContextRecord> {
        vec![
            ContextRecord {
                id: "b".into(),
                text: "Rust local retrieval runtime".into(),
            },
            ContextRecord {
                id: "a".into(),
                text: "Local context selection".into(),
            },
            ContextRecord {
                id: "c".into(),
                text: "Unrelated material".into(),
            },
        ]
    }

    #[test]
    fn ranks_by_distinct_matching_terms() {
        let result = select_context("local rust", &corpus(), 10);
        assert_eq!(
            result.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
            vec!["b", "a"]
        );
        assert_eq!(result[0].score, 2);
    }

    #[test]
    fn ties_are_deterministic_by_id() {
        let result = select_context("local", &corpus(), 10);
        assert_eq!(
            result.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
            vec!["a", "b"]
        );
    }

    #[test]
    fn limit_zero_returns_empty() {
        assert!(select_context("local", &corpus(), 0).is_empty());
    }

    #[test]
    fn no_match_returns_empty() {
        assert!(select_context("missing", &corpus(), 5).is_empty());
    }

    #[test]
    fn matching_is_case_insensitive_and_uses_whole_tokens() {
        let records = vec![
            ContextRecord {
                id: "partial".into(),
                text: "locality".into(),
            },
            ContextRecord {
                id: "exact".into(),
                text: "LOCAL context".into(),
            },
        ];
        let result = select_context("local", &records, 10);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "exact");
    }

    #[test]
    fn duplicate_query_terms_count_once() {
        let result = select_context("local local local", &corpus(), 10);
        assert_eq!(result[0].score, 1);
    }

    #[test]
    fn empty_query_returns_empty() {
        assert!(select_context(" !!! ", &corpus(), 5).is_empty());
    }
}
