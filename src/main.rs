mod game;

use game::Game;

fn main() {
    Game::new((30, 20)).start(300);
}
