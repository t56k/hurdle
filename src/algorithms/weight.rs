use std::borrow::Cow;

use once_cell::sync::OnceCell;

use crate::{Correctness, Guess, Guesser, Word, DICTIONARY};

static INITIAL: OnceCell<Vec<(&'static Word, usize)>> = OnceCell::new();

pub struct Weight {
    remaining: Cow<'static, Vec<(&'static Word, usize)>>,
}

impl Weight {
    pub fn new() -> Self {
        Self {
            remaining: Cow::Borrowed(INITIAL.get_or_init(|| {
                Vec::from_iter(DICTIONARY.lines().map(|line| {
                    let (word, count) = line
                        .split_once(' ')
                        .expect("every line is word + space + freq");
                    let count: usize = count.parse().expect("every count a number");
                    let word = word
                        .as_bytes()
                        .try_into()
                        .expect("every dict word is 5 chars");

                    (word, count)
                }))
            })),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static Word,
    goodness: f64,
}

impl Guesser for Weight {
    fn guess(&mut self, history: &[Guess]) -> Word {
        let mut best: Option<Candidate> = None;

        if let Some(last) = history.last() {
            if matches!(self.remaining, Cow::Owned(_)) {
                self.remaining
                    .to_mut()
                    .retain(|(word, _)| last.matches(word));
            } else {
                self.remaining = Cow::Owned(
                    self.remaining
                        .iter()
                        .filter(|(word, _)| last.matches(word))
                        .copied()
                        .collect(),
                );
            }
        }

        if history.is_empty() {
            return *b"tares";
        }

        let remaining_count: usize = self.remaining.iter().map(|&(_, c)| c).sum();

        for &(word, count) in &*self.remaining {
            let mut sum = 0.0;

            for pattern in Correctness::patterns() {
                let mut in_pattern_total = 0;
                for (candidate, count) in &*self.remaining {
                    let g = Guess {
                        word: Cow::Borrowed(word),
                        mask: pattern,
                    };

                    if g.matches(candidate) {
                        in_pattern_total += count;
                    }
                }

                if in_pattern_total == 0 {
                    continue;
                }

                let p_of_this_pattern = in_pattern_total as f64 / remaining_count as f64;
                sum += p_of_this_pattern * p_of_this_pattern.log2();
            }

            let p_word = count as f64 / remaining_count as f64;
            let goodness = p_word * -sum;

            if let Some(c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate { word, goodness });
                }
            } else {
                best = Some(Candidate { word, goodness });
            }
        }

        *best.unwrap().word
    }
}