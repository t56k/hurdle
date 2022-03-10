use std::borrow::Cow;
use std::collections::HashSet;

pub mod algorithms;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub type Word = [u8; 5];

pub struct Wordle {
    dictionary: HashSet<&'static Word>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ')
                    .expect("every line is word + space + freq")
                    .0
                    .as_bytes()
                    .try_into()
                    .expect("every word should be five characters")
            })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: Word, mut guesser: G) -> Option<usize> {
        let mut history = vec![];

        for i in 1..=32 {
            let guess = guesser.guess(&history);
            if guess == answer {
                return Some(i);
            }

            assert!(self.dictionary.contains(&guess));

            let correctness = Correctness::compute(&answer, &guess);
            history.push(Guess {
                word: Cow::Owned(guess),
                mask: correctness,
            });
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
    fn compute(answer: &Word, guess: &Word) -> [Self; 5] {
        let mut c = [Correctness::Wrong; 5];

        // Mark green
        for (i, (a, g)) in answer.iter().zip(guess.iter()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
            }
        }

        // Mark yellow
        let mut used = [false; 5];
        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::Correct {
                used[i] = true;
            }
        }

        for (i, g) in guess.iter().enumerate() {
            if c[i] == Correctness::Correct {
                continue;
            }

            if answer.iter().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }

        c
    }

    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

pub struct Guess<'a> {
    pub word: Cow<'a, Word>,
    pub mask: [Correctness; 5],
}

impl Guess<'_> {
    pub fn matches(&self, word: &Word) -> bool {
        return Correctness::compute(word, &self.word) == self.mask;
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> Word;
}

impl Guesser for fn(history: &[Guess]) -> Word {
    fn guess(&mut self, history: &[Guess]) -> Word {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> $crate::Word {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
macro_rules! mask {
    (C) => {$crate::Correctness::Correct};
    (M) => {$crate::Correctness::Misplaced};
    (W) => {$crate::Correctness::Wrong};
    ($($c:tt)+) => {[
        $(mask!($c)),+
    ]}
}

#[cfg(test)]
mod tests {
    mod guess_matcher {
        use crate::Guess;
        use std::borrow::Cow;

        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {
                    word: Cow::Borrowed($prev),
                    mask: mask![$($mask )+]
                }
                .matches($next));
                assert_eq!($crate::Correctness::compute($next, $prev), mask![$($mask )+]);
            };

            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {
                    word: Cow::Borrowed($prev),
                    mask: mask![$($mask )+]
                }
                .matches($next));
                assert_ne!($crate::Correctness::compute($next, $prev), mask![$($mask )+]);
            };
        }

        #[test]
        fn matches() {
            check!(b"abcde" + [C C C C C] allows b"abcde");
            check!(b"abcdf" + [C C C C C] disallows b"abcde");
            check!(b"abcde" + [W W W W W] allows b"fghik");
            check!(b"abcde" + [M M M M M] allows b"eabcd");
        }
    }

    mod game {
        use crate::{Guess, Wordle};

        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { *b"right" });

            assert_eq!(w.play(*b"right", guesser), Some(1));
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return *b"right";
                }
                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return *b"right";
                }
                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(3));
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return *b"right";
                }
                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(4));
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 4 {
                    return *b"right";
                }
                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(5));
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 5 {
                    return *b"right";
                }
                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(6));
        }

        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { *b"wrong" });

            assert_eq!(w.play(*b"right", guesser), None);
        }
    }

    mod compute {
        use crate::Correctness;

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute(b"abcde", b"abcde"), mask![C C C C C]);
        }

        #[test]
        fn all_grey() {
            assert_eq!(Correctness::compute(b"zxywq", b"abcde"), mask![W W W W W]);
        }

        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute(b"cedba", b"abcde"), mask![M M M M M]);
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute(b"aaabb", b"aaacc"), mask![C C C W W]);
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute(b"cccaa", b"aaacc"), mask![M M W M M]);
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute(b"aabbb", b"caacc"), mask![W C M W W]);
        }
    }
}
