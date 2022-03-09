use std::collections::HashMap;

use crate::{Guess, Guesser, DICTIONARY};

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(' ')
                    .expect("every line is word + space + freq");
                let count: usize = count.parse().expect("every count a number");
                (word, count)
            })),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    count: usize,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        let mut best: Option<Candidate> = None;

        if let Some(last) = history.last() {
            self.remaining.retain(|word, _| last.matches(word));
        }

        for (&word, &count) in &self.remaining {
            let goodness = 0.0; // TODO: how do we compute goodness

            if let Some(c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate {
                        word,
                        count,
                        goodness,
                    });
                }
            } else {
                best = Some(Candidate {
                    word,
                    count,
                    goodness,
                });
            }
        }

        best.unwrap().word.to_string()
    }
}
