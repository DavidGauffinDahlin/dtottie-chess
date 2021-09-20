use dtottie_chess::Game;
use std::io;

fn main() {
    let mut game1 = Game::new_game();

    game1.print_board();

    let chesspiece = '\u{2659}';
    println!("{}", chesspiece);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let _ = game1.player_move(input);
        //print!("\x1B[2J\x1B[1;1H");
        game1.print_board();
        println!("black checked: {}",game1.b_checked);
        //let id = game1.board[7][0].unique_name;
        //println!("{}", id);
    }
    // let moves = ["E7E6\n", "A2A3\n", "E8E7\n"];
    // for pmove in moves {
    //     game1.player_move(pmove.to_owned());
    //     game1.print_board();
    // }
}
