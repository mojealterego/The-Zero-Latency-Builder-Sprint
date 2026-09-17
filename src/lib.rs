//! Minimal deterministic local context selector for the sprint prototype.
//! This is a lexical baseline, not Moss integration or semantic search.

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

/// Rank records by the number of distinct query terms found in each record.
/// Ties are resolved by record ID for deterministic output. `limit` bounds output.
pub fn select_context(query: &str, records: &[ContextRecord], limit: usize) -> Vec<RankedRecord> {
    if limit == 0 {
        return Vec::new();
    }

    let terms: Vec<String> = query
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(str::to_lowercase)
        .collect();

    let mut ranked: Vec<RankedRecord> = records
        .iter()
        .map(|record| {
            let text = record.text.to_lowercase();
            let score = terms.iter().collect::<std::collections::BTreeSet<_>>()
                .iter()
                .filter(|term| text.contains(term.as_str()))
                .count();
            RankedRecord { id: record.id.clone(), score, text: record.text.clone() }
        })
        .filter(|item| item.score > 0)
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
            ContextRecord { id: "b".into(), text: "Rust local retrieval runtime".into() },
            ContextRecord { id: "a".into(), text: "Local context selection".into() },
            ContextRecord { id: "c".into(), text: "Unrelated material".into() },
        ]
    }

    #[test]
    fn ranks_by_distinct_matching_terms() {
        let result = select_context("local rust", &corpus(), 10);
        assert_eq!(result.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), vec!["b", "a"]);
        assert_eq!(result[0].score, 2);
    }

    #[test]
    fn ties_are_deterministic_by_id() {
        let result = select_context("local", &corpus(), 10);
        assert_eq!(result.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), vec!["a", "b"]);
    }

    #[test]
    fn limit_zero_returns_empty() {
        assert!(select_context("local", &corpus(), 0).is_empty());
    }

    #[test]
    fn no_match_returns_empty() {
        assert!(select_context("missing", &corpus(), 5).is_empty());
    }
}
