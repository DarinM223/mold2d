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

    pub fn score(&self, counter_name: &str) -> i64 {
        self.scores.get(counter_name).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_score() {
        let mut score = Score::new();
        score.add_score("GAME");
        assert_eq!(score.score("GAME"), 0);
    }

    #[test]
    #[should_panic]
    fn test_remove_score() {
        let mut score = Score::new();
        score.add_score("GAME");
        assert_eq!(score.score("GAME"), 0);
        score.remove_score("GAME");
        score.score("GAME");
    }

    #[test]
    fn test_update_score() {
        let mut score = Score::new();
        score.add_score("GAME");
        score.update_score("GAME", 20);
        assert_eq!(score.score("GAME"), 20);
    }
}
