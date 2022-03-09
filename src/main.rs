const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = hurdle::algorithms::Naive::new();
        hurdle::play(answer, guesser);
    }
}
