use clap::{ArgEnum, Parser};
use hurdle::Guesser;

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, arg_enum)]
    implementation: Implementation,

    #[clap(short, long)]
    max: Option<usize>,
}

#[derive(ArgEnum, Debug, Clone, Copy)]
enum Implementation {
    Allocs,
    Vecrem,
    Once,
}

fn main() {
    let args = Args::parse();

    match args.implementation {
        Implementation::Allocs => play(hurdle::algorithms::Allocs::new, args.max),
        Implementation::Vecrem => play(hurdle::algorithms::Vecrem::new, args.max),
        Implementation::Once => play(hurdle::algorithms::OnceInit::new, args.max),
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let w = hurdle::Wordle::new();

    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let answer_b: hurdle::Word = answer
            .as_bytes()
            .try_into()
            .expect("all answers are five chars");
        let guesser = (mk)();

        if let Some(score) = w.play(answer_b, guesser) {
            println!("guessed '{}' in {}", answer, score);
        } else {
            eprintln!("failed to guess");
        }
    }
}
