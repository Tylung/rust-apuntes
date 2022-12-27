mod arg;
mod game;
mod utils;
use arg::get_opts;
use game::init_game;

fn main() {
    
    let opts = get_opts();

    init_game(opts);
}

