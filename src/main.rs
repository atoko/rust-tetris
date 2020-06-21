use rltk::{Rltk, GameState, VirtualKeyCode};
mod tetris;


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Tetris-Rust")
        .build()?;


    rltk::main_loop(context, tetris::new_game())
}
