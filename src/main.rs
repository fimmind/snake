mod game;

use game::Game;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "snake")]
struct Opt {
    /// Field's width (in cells)
    #[structopt(short, long, default_value = "20")]
    width: usize,

    /// Field's height (in cells)
    #[structopt(short, long, default_value = "10")]
    height: usize,

    /// Step delay (in miliseconds)
    #[structopt(short, long, default_value = "285")]
    delay: u64,
}

fn main() {
    let opt = Opt::from_args();
    Game::new((opt.width, opt.height)).start(opt.delay);
}
