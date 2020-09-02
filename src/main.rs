mod game;

use game::Game;
use clap::{App, Arg,};

fn main() {
    let matches = App::new("snake")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A simple snake game. You can use both arrows and vim-like keys")
        .args(&[
            Arg::with_name("field width")
                .help("Field's width (cells)")
                .long("width")
                .short("w")
                .value_name("N")
                .default_value("20"),
            Arg::with_name("field height")
                .help("Field's height (cells)")
                .long("height")
                .short("h")
                .value_name("N")
                .default_value("10"),
            Arg::with_name("step delay")
                .help("Step delay (miliseconds)")
                .long("delay")
                .short("d")
                .takes_value(true)
                .value_name("N")
                .default_value("285")
        ])
        .get_matches();

    let width = matches.value_of("field width").unwrap().parse().unwrap();
    let height = matches.value_of("field height").unwrap().parse().unwrap();
    let delay = matches.value_of("step delay").unwrap().parse().unwrap();

    Game::new((width, height)).start(delay);
}
