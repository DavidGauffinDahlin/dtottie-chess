use std::path;
use dtottie_chess::Game;
use dtottie_chess::pieces::PieceKind;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::{Context, GameResult};
use ggez::input::mouse::MouseButton;
// use ggez::mint::Point2;
use glam::Vec2;

const WINDOWSIZE: f32 = 1000.0;

struct SpriteSheet {
    pawn_white: graphics::Image,
    pawn_black: graphics::Image,
    bishop_white: graphics::Image,
    bishop_black: graphics::Image,
    knight_white: graphics::Image,
    knight_black: graphics::Image,
    rook_white: graphics::Image,
    rook_black: graphics::Image,
    queen_white: graphics::Image,
    queen_black: graphics::Image,
    king_white: graphics::Image,
    king_black: graphics::Image,
}

struct MouseInfo {
    mouse_down: bool,
    mouse_button: MouseButton,
    pressed_position: Vec2,
}

pub struct GAME {
    logic: Game,
    board_boundary: graphics::Rect,
    move_from: usize,
    move_to: usize,
    make_move: bool,
}

struct MainState {
    spritesheet: SpriteSheet,
    pub current_cursor_position: Vec2,
    mouse_info: MouseInfo,
    game: GAME,
}

impl MainState {
    fn new(_ctx: &mut Context) -> Self {
        let spritesheet = SpriteSheet {
            pawn_white: graphics::Image::new(_ctx, "/wp.png").unwrap(),
            pawn_black: graphics::Image::new(_ctx, "/bp.png").unwrap(),
            bishop_white: graphics::Image::new(_ctx, "/wb.png").unwrap(),
            bishop_black: graphics::Image::new(_ctx, "/bb.png").unwrap(),
            knight_white: graphics::Image::new(_ctx, "/wkn.png").unwrap(),
            knight_black: graphics::Image::new(_ctx, "/bkn.png").unwrap(),
            rook_white: graphics::Image::new(_ctx, "/wr.png").unwrap(),
            rook_black: graphics::Image::new(_ctx, "/br.png").unwrap(),
            queen_white: graphics::Image::new(_ctx, "/wq.png").unwrap(),
            queen_black: graphics::Image::new(_ctx, "/bq.png").unwrap(),
            king_white: graphics::Image::new(_ctx, "/wk.png").unwrap(),
            king_black: graphics::Image::new(_ctx, "/bk.png").unwrap()};
        MainState {
            spritesheet: spritesheet,
            current_cursor_position: Vec2::new(0.0, 0.0),
            mouse_info: MouseInfo {
                mouse_down: false,
                mouse_button: MouseButton::Left,
                pressed_position: Vec2::new(0.0, 0.0),
            },
            game: GAME {
                logic: Game::new_game(),
                board_boundary: graphics::Rect::new(0.0, 0.0, 0.0, 0.0),
                move_from: 100,
                move_to: 100,
                make_move: false,
            },
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(209, 204, 192));

        let _err = handle_tile_selection(ctx, self);

        let _err = make_a_move(ctx, self);

        let _err = render_graphical_board(ctx, self);

        let _err = render_pieces(ctx, self);
        
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.current_cursor_position = Vec2::new(x, y);
    }
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        self.mouse_info.mouse_down = true;
        self.mouse_info.pressed_position = Vec2::new(_x, _y);
        self.mouse_info.mouse_button = _button;
        // println!("Mouse button pressed: {:?}, x: {}, y: {}", _button, _x, _y);
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        self.mouse_info.mouse_down = false;
        self.mouse_info.pressed_position = Vec2::new(_x, _y);
        self.mouse_info.mouse_button = _button;
        // println!("Mouse button released: {:?}, x: {}, y: {}", _button, _x, _y);
    }
}

fn draw_piece(ctx: &mut Context, piece: &ggez::graphics::Image, file: usize, rank: usize) -> GameResult {
    let image = piece;
    let scale_factor = 860.0 / (image.dimensions().h * 8.0);
    let scale = ggez::mint::Point2 {x: scale_factor, y: scale_factor};
    let size = image.dimensions().h * scale.x;
    let dst = ggez::mint::Point2 {x: 70.0 +size * file as f32, y: 70.0 + size * rank as f32};

    graphics::draw(
        ctx,
        image,
        graphics::DrawParam::new().dest(dst).scale(scale),)?;
    Ok(())
}

fn render_pieces(ctx: &mut Context, state: &mut MainState) -> GameResult {
    let board = state.game.logic.board;

    for rank in 0..8 {
        for file in 0..8 {
            if board[rank][file].color_white {
                // match board[rank][file].kind {
                //     // Match a single value
                //     PieceKind::Rook => println!("One!"),
                //     // Match several values
                //     2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
                //     // TODO ^ Try adding 13 to the list of prime values
                //     // Match an inclusive range
                //     13..=19 => println!("A teen"),
                //     // Handle the rest of cases
                //     _ => println!("Ain't special"),
                //     // TODO ^ Try commenting out this catch-all arm
                // }
                if board[rank][file].kind == PieceKind::Rook {
                    let piece = &state.spritesheet.rook_white;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::Bishop {
                    let piece = &state.spritesheet.bishop_white;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::Knight {
                    let piece = &state.spritesheet.knight_white;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::Queen {
                    let piece = &state.spritesheet.queen_white;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::King {
                    let piece = &state.spritesheet.king_white;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::Pawn {
                    let piece = &state.spritesheet.pawn_white;
                    let _err = draw_piece(ctx, piece, file, rank);
                }
            } else {
                if board[rank][file].kind == PieceKind::Rook {
                    let piece = &state.spritesheet.rook_black;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::Bishop {
                    let piece = &state.spritesheet.bishop_black;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::Knight {
                    let piece = &state.spritesheet.knight_black;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::Queen {
                    let piece = &state.spritesheet.queen_black;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::King {
                    let piece = &state.spritesheet.king_black;
                    let _err = draw_piece(ctx, piece, file, rank);
                } else if board[rank][file].kind == PieceKind::Pawn {
                    let piece = &state.spritesheet.pawn_black;
                    let _err = draw_piece(ctx, piece, file, rank);
                }
            }
        }
    }

    Ok(())
}

fn make_a_move(_ctx: &mut Context, state: & mut MainState) -> GameResult {

    if state.game.make_move {
        let move_from = state.game.move_from;
        let move_to = state.game.move_to;

        let move_command_to_logic = convert_move_to_logic_understandable(state, move_from, move_to);

        let is_valid_move = state.game.logic.player_move(move_command_to_logic);
    }

    Ok(())
}

fn convert_move_to_logic_understandable(state: & mut MainState, move_from: usize, move_to: usize) -> String {
    let index_to_alphabet = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    let from_rank = (move_from as f32 / 8.0).floor();
    let from_file = ((move_from as f32) -  8.0 * (from_rank as f32)) as usize;
    let from_rank = from_rank + 1.0;
    let from_file = index_to_alphabet[from_file];

    let to_rank = (move_to as f32 / 8.0).floor();
    let to_file = ((move_to as f32) -  8.0 * (to_rank as f32)) as usize;
    let to_rank = to_rank + 1.0;
    let to_file = index_to_alphabet[to_file];
    
    let command = from_file.to_string().as_str().to_owned() + from_rank.to_string().as_str() + to_file.to_string().as_str() + to_rank.to_string().as_str() + "\n";

    state.game.make_move = false;
    
    command
}

fn handle_tile_selection(_ctx: &mut Context, state: & mut MainState) -> GameResult {
    let mouse_down = state.mouse_info.mouse_down;
    let mouse_button = state.mouse_info.mouse_button;
    let pressed_position = state.mouse_info.pressed_position;

    let pressed_tile_index = convert_coors_to_tile_index(pressed_position, state);

    if mouse_down {
        if mouse_button == MouseButton::Left {
            if pressed_tile_index != state.game.move_from && pressed_tile_index != state.game.move_to {
                if (state.game.move_from == 100 && state.game.move_to == 100) || (state.game.move_from < 64 && state.game.move_to < 64) {
                    if state.game.move_to < 64 {
                        state.game.move_to = 100;
                    }
                    state.game.move_from = pressed_tile_index;
                } else if state.game.move_to == 100 && pressed_tile_index != state.game.move_from {
                    state.game.move_to = pressed_tile_index;
                    state.game.make_move = true;
                }
            }
        } else if mouse_button == MouseButton::Right {
            state.game.move_from = 100;
            state.game.move_to = 100;
        }
    }

    Ok(())
}

fn convert_coors_to_tile_index(pressed_position: Vec2, state: &mut MainState) -> usize {
    let x_pressed = pressed_position[0];
    let y_pressed = pressed_position[1];

    let xy = state.game.board_boundary.x;
    let wh = state.game.board_boundary.w;

    let board_space_x = x_pressed - xy;
    let board_space_y = y_pressed - xy;

    let file = (board_space_x / (wh / 8.0)).floor();
    let rank = (board_space_y / (wh / 8.0)).floor();
    
    let index = (rank * 8.0 + file) as usize;
    index
}

fn render_graphical_board(ctx: &mut Context, state: &mut MainState) -> GameResult {
    let (screen_w, _) = graphics::drawable_size(ctx);

    let offset: f32 = 70.0;
    let tile_size: f32 = ((screen_w as f32) - (2 as f32) * (offset as f32)) / (8 as f32);

    state.game.board_boundary = graphics::Rect::new(offset, offset, tile_size * 8.0, tile_size * 8.0);

    for file in 0..8 {
        for rank in 0..8 {
            let tile_index = (rank * 8 + file) as usize;
            let is_light_square = (file + rank) % 2 != 0;

            let bound= graphics::Rect { x: (tile_size * file as f32 + offset), y: (tile_size * rank as f32 + offset), w: tile_size, h: tile_size };
            let square_color = manage_tile_color(state, bound, tile_index, is_light_square);

            let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bound, graphics::Color::from_rgb(square_color.0, square_color.1, square_color.2))?;
            let draw_param = graphics::DrawParam::default();
            graphics::draw(ctx, &rect_mesh, draw_param)?;
        }
    }
    Ok(())
}

fn manage_tile_color(state: &mut MainState, bound: graphics::Rect, tile_index: usize, is_light_square: bool) -> (u8, u8, u8) {
    let hovering_tile_color = (179, 57, 57);
    let move_from_tile_color = (255, 82, 82);
    let move_to_tile_color = (179, 57, 57);

    let light_color_tile = (255, 218, 121);
    let dark_color_tile = (204, 142, 53);

    let mut square_color = if is_light_square { light_color_tile }  else { dark_color_tile};
    let (x, y) = (state.current_cursor_position[0], state.current_cursor_position[1]);

    // Hovering tile color
    if x > bound.x && x < (bound.x + bound.w) && y > bound.y && y < (bound.y + bound.h) {
        square_color = hovering_tile_color;
    }

    // Move from tile color
    if state.game.move_from == tile_index {
        square_color = move_from_tile_color;
    }

    // Move to tile color
    if state.game.move_to ==tile_index {
        square_color = move_to_tile_color;
    }

    square_color
}

fn main() -> GameResult {
    // set up resource path from the base of the project
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("Chess", "David Gauffin Dahlin").add_resource_path(resource_dir);
    let (mut ctx, mut event_loop) = cb.build()?;
    graphics::set_window_title(&ctx, "Chessy Stressy");
    graphics::set_drawable_size(&mut ctx, WINDOWSIZE, WINDOWSIZE)?;
    graphics::set_screen_coordinates(&mut ctx, graphics::Rect { x: 0.0, y: 0.0, w: 1000.0, h: 1000.0 })?;
    let mut state = MainState::new(&mut ctx);
    event::run(&mut ctx, &mut event_loop, &mut state)
}

