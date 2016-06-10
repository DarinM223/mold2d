use std::collections::HashMap;

/// Maintains multiple updatable scores
pub struct Score {
    scores: HashMap<String, i64>,
}

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

    pub fn increment_score(&mut self, name: &str, increment_amount: i32) {
        if let Some(score) = self.scores.get_mut(name) {
            *score = *score + increment_amount as i64;
        }
    }

    pub fn score(&self, counter_name: &str) -> Option<i64> {
        if let Some(score) = self.scores.get(counter_name) {
            return Some(score.clone());
        }

        None
    }

    // TODO(DarinM223): add functions to save and load a score from a file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_score() {
        let mut score = Score::new();
        score.add_score("GAME");
        assert_eq!(score.score("GAME"), Some(0));
    }

    #[test]
    fn test_remove_score() {
        let mut score = Score::new();
        score.add_score("GAME");
        assert_eq!(score.score("GAME"), Some(0));
        score.remove_score("GAME");
        assert_eq!(score.score("GAME"), None);
    }

    #[test]
    fn test_update_score() {
        let mut score = Score::new();
        score.add_score("GAME");
        score.update_score("GAME", 20);
        assert_eq!(score.score("GAME"), Some(20));
    }
}
