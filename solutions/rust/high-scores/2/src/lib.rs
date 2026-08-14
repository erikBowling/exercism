#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.into(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut highest: Vec<_> = Vec::new();

        let mut temp_scores = self.scores.clone();
        temp_scores.sort();

        while let Some(score) = temp_scores.pop() {
            highest.push(score);
            if highest.len() == 3 {
                break;
            }
        }

        highest
    }
}
