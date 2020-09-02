mod game;

use game::Game;
use clap::{App, Arg};

fn main() {
    let matches = App::new("snake")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Just a simple snake game")
        .args(&[
            Arg::with_name("field width")
                .help("Witdth of the game field")
                .long("width")
                .short("w")
                .value_name("N")
                .default_value("20"),
            Arg::with_name("field height")
                .help("Height of the game field")
                .long("height")
                .short("h")
                .value_name("N")
                .default_value("10"),
            Arg::with_name("step delay")
                .help("Delay before a next step")
                .long("delay")
                .short("d")
                .takes_value(true)
                .value_name("MILISECONDS")
                .default_value("200")
        ])
        .get_matches();

    let width = matches.value_of("field width").unwrap().parse().unwrap();
    let height = matches.value_of("field height").unwrap().parse().unwrap();
    let delay = matches.value_of("step delay").unwrap().parse().unwrap();

    Game::new((width, height)).start(delay);
}
