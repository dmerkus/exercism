#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.len() {
            0 => None,
            _ => Some(self.scores[self.scores.len() - 1]),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.len() {
            0 => None,
            _ => Some(
                self.scores
                    .iter()
                    .fold(0, |best, &score| if score > best { score } else { best }),
            ),
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();

        scores.sort_unstable_by(|a, b| b.cmp(a));
        scores.truncate(3);

        scores
    }
}
