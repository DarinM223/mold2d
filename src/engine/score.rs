use std::collections::HashMap;

pub struct Score {
    scores: HashMap<String, i64>,
}

/// Maintains multiple updatable scores
impl Score {
    pub fn new() -> Score {
        Score { scores: HashMap::new() }
    }

    pub fn add_score(&mut self, name: &str) {
        self.scores.insert(name.to_owned(), 0);
    }

    pub fn remove_score(&mut self, name: &str) {
        self.scores.remove(name);
    }

    pub fn update_score(&mut self, name: &str, new_score: i64) {
        if let Some(score) = self.scores.get_mut(name) {
            *score = new_score;
        }
    }

    pub fn score(&self, counter_name: &str) -> i64 {
        self.scores.get(counter_name).unwrap().clone()
    }
}
