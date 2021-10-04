use dtottie_chess::Game;
use std::io;

fn main() {
    let mut game = Game::new_game();

    game.print_board();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let _ = game.player_move(input);
        game.print_board();
    }
}
