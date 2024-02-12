use clap::Parser;
use life::GameOfLife;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'r', long, default_value_t = 3)]
    rows: usize,
    #[clap(short = 'c', long, default_value_t = 3)]
    cols: usize,
}

fn main() {
    let args = Args::parse();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        r#"{}{}[Right] to show the next step, [CTRL + q] to exit"#,
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut game_of_life = GameOfLife::new(args.rows, args.cols);

    for c in stdin.keys() {
        // Clears the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Right => {
                game_of_life.get_next();
                println!("{}", game_of_life);
            }
            _ => (),
        }

        stdout.flush().unwrap();
    }
}
