#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u32>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u32) -> Result<(), Error> {
        if self.score().is_some() {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let mut frame = 0;
        let mut i = 0;

        while frame < 10 && i < self.rolls.len() {
            if self.rolls[i] == 10 {
                frame += 1;
                i += 1;
            } else if i + 1 < self.rolls.len() {
                frame += 1;
                i += 2;
            } else {
                break;
            }
        }

        if frame < 10 && i < self.rolls.len() && self.rolls[i] != 10 {
            if self.rolls[i] + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u32> {
        let mut score = 0;
        let mut i = 0;

        for _ in 0..10 {
            if i >= self.rolls.len() {
                return None;
            }

            if self.rolls[i] == 10 {
                if i + 2 >= self.rolls.len() {
                    return None;
                }

                score += 10 + self.rolls[i + 1] + self.rolls[i + 2];
                i += 1;
            } else {
                if i + 1 >= self.rolls.len() {
                    return None;
                }

                let frame_score = self.rolls[i] + self.rolls[i + 1];

                if frame_score > 10 {
                    return None;
                }

                if frame_score == 10 {
                    if i + 2 >= self.rolls.len() {
                        return None;
                    }

                    score += 10 + self.rolls[i + 2];
                } else {
                    score += frame_score;
                }

                i += 2;
            }
        }

        Some(score)
    }
}