/// Scoring weights for search results
#[derive(Debug, Clone)]
pub struct ScoreWeights {
    pub exact_match: f64,
    pub prefix_match: f64,
    pub fuzzy_match: f64,
    pub recent_use: f64,
}

impl Default for ScoreWeights {
    fn default() -> Self {
        Self {
            exact_match: 100.0,
            prefix_match: 80.0,
            fuzzy_match: 50.0,
            recent_use: 20.0,
        }
    }
}
