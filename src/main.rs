const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = hurdle::Wordle::new();

    for answer in GAMES.split_whitespace() {
        let guesser = hurdle::algorithms::Naive::new();
        w.play(answer, guesser);
    }
}
