#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Copy, Clone)]
pub struct Frame {
    rolls: [Option<u16>; 3],
}

impl Frame {
    pub fn new() -> Self {
        Self { rolls: [None; 3] }
    }
}

pub struct BowlingGame {
    cur_frame: usize,
    frames: [Frame; 10],
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            cur_frame: 0,
            frames: [Frame::new(); 10],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.cur_frame >= 10 {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let frame = &mut self.frames[self.cur_frame];

        // First roll
        if frame.rolls[0].is_none() {
            frame.rolls[0] = Some(pins);
            if pins == 10 && self.cur_frame != 9 {
                self.cur_frame += 1;
            }
        // Second roll
        } else if frame.rolls[1].is_none() {
            frame.rolls[1] = Some(pins);

            // If frame 10 don't move on if spare
            if self.cur_frame == 9 {
                if pins + frame.rolls[0].unwrap() < 10 {
                    self.cur_frame += 1;
                }
            } else {
                if pins + frame.rolls[0].unwrap() > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.cur_frame += 1;
            }
        // Third roll (10th frame)
        } else {
            // If first roll is strike
            if frame.rolls[0].unwrap() == 10
                && frame.rolls[1].unwrap() != 10
                && frame.rolls[1].unwrap() + pins > 10
            {
                return Err(Error::NotEnoughPinsLeft);
            }

            frame.rolls[2] = Some(pins);

            self.cur_frame += 1;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.cur_frame < 10 {
            return None;
        }

        let mut score: u16 = 0;
        let rolls: Vec<u16> = self
            .frames
            .iter()
            .flat_map(|f| f.rolls.iter().filter_map(|&r| r))
            .collect();

        let mut rolls = rolls.iter();

        for frame in self.frames {
            match frame.rolls {
                [Some(_), None, None] => {
                    // Strike
                    score += rolls.next().unwrap();

                    let mut temp = rolls.as_slice().iter();
                    score += [temp.next(), temp.next()]
                        .iter()
                        .filter_map(|&x| x)
                        .sum::<u16>();
                }
                [Some(x), Some(y), None] => {
                    score += rolls.next().unwrap_or(&0);
                    score += rolls.next().unwrap_or(&0);

                    // Spare
                    if x + y == 10 {
                        println!("Spare {x} {y}");
                        score += rolls.as_slice().iter().next().unwrap();
                    }
                }
                [Some(_), Some(_), Some(_)] => {
                    score += rolls.next().unwrap();
                    score += rolls.next().unwrap();
                    score += rolls.next().unwrap();
                }
                _ => {}
            }
        }

        Some(score)
    }
}
