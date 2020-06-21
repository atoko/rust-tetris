use rltk::{Rltk, GameState, VirtualKeyCode};
use rand::prelude::*;

// Map
const WIDTH: i32 = 12;
const HEIGHT: i32 = 21;
const MAP_OFFSET_X: i32 = 4;
const MAP_OFFSET_Y: i32 = 4;

#[derive(PartialEq, Copy, Clone)]
enum MapTile {
    Empty,
    Tile,
    Wall,
    Cleared
}

struct Position {
    x: i32,
    y: i32
}

const SQUARE: [&'static str; 16] = [
    ".", ".", ".", ".",
    ".", "x", "x", ".",
    ".", "x", "x", ".",
    ".", ".", ".", "."
];
const TSHAPE: [&'static str; 16] = [
    ".", ".", "x", ".",
    ".", "x", "x", ".",
    ".", ".", "x", ".",
    ".", ".", ".", "."
];
const LSHAPE: [&'static str; 16] = [
    ".", ".", "x", ".",
    ".", "x", "x", ".",
    ".", "x", ".", ".",
    ".", ".", ".", "."
];
const RSHAPE: [&'static str; 16] = [
    ".", "x", ".", ".",
    ".", "x", "x", ".",
    ".", ".", "x", ".",
    ".", ".", ".", "."
];
const LHOOK: [&'static str; 16] = [
    ".", ".", ".", ".",
    ".", "x", "x", ".",
    ".", ".", "x", ".",
    ".", ".", "x", "."
];
const RHOOK: [&'static str; 16] = [
    ".", ".", ".", ".",
    ".", "x", "x", ".",
    ".", "x", ".", ".",
    ".", "x", ".", "."
];
const LINE: [&'static str; 16] = [
    ".", ".", "x", ".",
    ".", ".", "x", ".",
    ".", ".", "x", ".",
    ".", ".", "x", "."
];

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH as usize) + x as usize
}


fn render_map(map: &Vec<MapTile>, ctx: &mut Rltk) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let current = map[xy_idx(x, y)];
            let mut toRender = "";
            if current == MapTile::Empty {
                toRender = " "
            } else if current == MapTile::Tile {
                toRender = "#"
            } else if current == MapTile::Wall {
                toRender = "X"
            } else if current == MapTile::Cleared {
                toRender = "~"
            }

            let scaledX = x * 2;
            let scaledY = y * 2;


            ctx.print(MAP_OFFSET_X + scaledX, MAP_OFFSET_Y + scaledY, toRender);
            ctx.print(MAP_OFFSET_X + scaledX, MAP_OFFSET_Y + scaledY + 1, toRender);
            ctx.print(MAP_OFFSET_X + scaledX + 1, MAP_OFFSET_Y + scaledY, toRender);
            ctx.print(MAP_OFFSET_X + scaledX + 1, MAP_OFFSET_Y + scaledY + 1, toRender);
        }
    }
}

fn render_current(game: &PlayingState, ctx: &mut Rltk) {
    let current_tile = &game.tetrominos[game.currentPiece];
    for x in 0..4 {
        for y in 0..4 {
            let current = current_tile[rotate(x, y, game.currentRotation as i32)];
            if current == "x" {

                let scaledX = (game.currentPosition.x + x as i32) * 2;
                let scaledY = (game.currentPosition.y + y as i32) * 2;

                ctx.print(MAP_OFFSET_X + scaledX, MAP_OFFSET_Y + scaledY, "*");
                ctx.print(MAP_OFFSET_X + scaledX, MAP_OFFSET_Y + scaledY + 1, "*");
                ctx.print(MAP_OFFSET_X + scaledX + 1, MAP_OFFSET_Y + scaledY, "*");
                ctx.print(MAP_OFFSET_X + scaledX + 1, MAP_OFFSET_Y + scaledY + 1, "*");
            }
        }
    }
}


fn render_gui(game: &PlayingState, ctx: &mut Rltk) {
    ctx.print((MAP_OFFSET_X *2) + (WIDTH * 2), (HEIGHT * 2) - 2, format!("Score: {score}", score = game.currentScore));
    ctx.print((MAP_OFFSET_X *2) + (WIDTH * 2), (HEIGHT * 2) - 1, format!("Lines Cleared: {lines}", lines = game.currentRowsCleared));

    ctx.print((MAP_OFFSET_X *2) + (WIDTH * 2), MAP_OFFSET_Y, "Next tile:");
    let next_tile = &game.tetrominos[game.nextPiece];
    for x in 0..4 {
        for y in 0..4 {
            let current = next_tile[rotate(x, y, 0)];
            if current == "x" {

                let scaledX = (x as i32) * 2;
                let scaledY = ( y as i32) * 2;

                ctx.print((MAP_OFFSET_X *2) + (WIDTH * 2) + scaledX, MAP_OFFSET_Y + scaledY + 2, "*");
                ctx.print((MAP_OFFSET_X *2) + (WIDTH * 2) + scaledX, MAP_OFFSET_Y + scaledY + 2 + 1, "*");
                ctx.print((MAP_OFFSET_X *2) + (WIDTH * 2) + scaledX + 1, MAP_OFFSET_Y + scaledY + 2, "*");
                ctx.print((MAP_OFFSET_X *2) + (WIDTH * 2) + scaledX + 1, MAP_OFFSET_Y + scaledY + 2 + 1, "*");
            }
        }
    }

    if game.isGameover {
        ctx.print((MAP_OFFSET_X * 2) + 5, 30, "Game over!!");
    }
}


fn rotate(px: i32, py: i32, rotation: i32) -> usize {
    let rotationNormalized = rotation % 4;
    return match rotationNormalized {
        0 => py * 4 + px,
        1 => 12 + py - (px * 4),
        2 => 15 - (py * 4) - px,
        3 => 3 - py + (px * 4),
        _ => 0
    } as usize;
}

fn piece_fits(checkX: i32, checkY: i32, rotation: i32, game: &PlayingState) -> bool {
    let current_tile = &game.tetrominos[game.currentPiece];
    for x in 0..4 {
        for y in 0..4 {
            let current = current_tile[rotate(x, y, rotation as i32)];

            if checkX + x >= 0 && checkX + x < WIDTH {
                if checkY + y >= 0 && checkY + y < HEIGHT {
                    if current == "x" && game.map[xy_idx(checkX + x, checkY + y)] != MapTile::Empty {
                        return false;
                    }
                }
            }
        }
    }

    return true;
}

fn check_current(gs: &mut PlayingState) {
    let elapsed = gs.lastTimestamp.elapsed();
    if elapsed.as_secs() > 0 {
        if piece_fits(
            gs.currentPosition.x,
            gs.currentPosition.y + 1,
            gs.currentRotation as i32,
            gs
        ) {
            gs.currentPosition.y += 1;
        } else {
            let current_tile = &gs.tetrominos[gs.currentPiece];
            for x in 0..4 {
                for y in 0..4 {
                    let current = current_tile[rotate(x, y, gs.currentRotation as i32)];
                    if current == "x" {
                        gs.map[xy_idx(gs.currentPosition.x + x, gs.currentPosition.y + y)] = MapTile::Tile;
                    }
                }
            }

            for y in 0..4 {
                let mut lineComplete = true;
                if gs.currentPosition.y + y < HEIGHT - 1 {
                    for x in 1..WIDTH-1 {
                        if gs.map[xy_idx(x, gs.currentPosition.y + y) as usize] == MapTile::Empty {
                            lineComplete = false;
                        }
                    }
                    if lineComplete == true {
                        for x in 1..WIDTH-1 {
                            gs.map[xy_idx(x, gs.currentPosition.y + y) as usize] = MapTile::Cleared
                        }

                        gs.clearedTimestamp = instant::Instant::now();
                        gs.clearedRows.push(gs.currentPosition.y + y);
                    }
                }
            }


            gs.currentScore += 25;
            gs.currentPiece = gs.nextPiece;
            gs.nextPiece = gs.rng.gen_range(0, gs.tetrominos.len());
            gs.currentPosition = Position {
                x: WIDTH / 2,
                y: 0
            };
            gs.currentRotation = 0;

            if !piece_fits(
                gs.currentPosition.x,
                gs.currentPosition.y,
                gs.currentRotation as i32,
                gs
            ) {
                gs.isGameover = true
            }
        }

        gs.lastTimestamp = instant::Instant::now();
    }
}

fn drop_rows(gs: &mut PlayingState) {
    let elapsed = gs.clearedTimestamp.elapsed();
    if elapsed.as_millis() > 300 {
        if !gs.clearedRows.is_empty() {
            for row in &gs.clearedRows {
                for x in 1..WIDTH-1 {
                    for y in (1..*row + 1).rev() {
                        gs.map[xy_idx(x, y) as usize] = gs.map[xy_idx(x, y - 1) as usize]
                    }
                }
            }

            gs.currentRowsCleared += gs.clearedRows.len() as i32;
            gs.currentScore += (1 << gs.clearedRows.len() as i32) * 100;
            gs.clearedRows.clear();
        }
    }
}

fn player_input(gs: &mut PlayingState, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => {
                if piece_fits(
                    gs.currentPosition.x - 1,
                    gs.currentPosition.y,
                    gs.currentRotation as i32,
                    gs
                ) {
                    gs.currentPosition.x -= 1
                }
            },
            VirtualKeyCode::Right => {
                if piece_fits(
                    gs.currentPosition.x + 1,
                    gs.currentPosition.y,
                    gs.currentRotation as i32,
                    gs
                ){
                    gs.currentPosition.x += 1
                }
            },
            VirtualKeyCode::Down => {
                if piece_fits(
                    gs.currentPosition.x,
                    gs.currentPosition.y + 1,
                    gs.currentRotation as i32,
                    gs
                ) {
                    gs.currentPosition.y += 1
                }
            },
            VirtualKeyCode::Z => {
                if piece_fits(
                    gs.currentPosition.x,
                    gs.currentPosition.y,
                    gs.currentRotation as i32 + 1,
                    gs
                ) {
                    gs.currentRotation = (gs.currentRotation + 1) % 4
                }
            }
            _ => {}
        },
    }
}

pub struct PlayingState {
    map: Vec<MapTile>,
    lastTimestamp: instant::Instant,
    clearedTimestamp: instant::Instant,
    rng: ThreadRng,
    tetrominos: Vec<[&'static str; 16]>,
    clearedRows: Vec<i32>,
    isGameover: bool,
    currentPiece: usize,
    nextPiece: usize,
    currentRotation: usize,
    currentPosition: Position,
    currentScore: i32,
    currentRowsCleared: i32
}


impl GameState for PlayingState {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        if self.isGameover == false {
            check_current(self);
            drop_rows(self);
        }
        render_map(&self.map, ctx);
        render_current(&self, ctx);
        render_gui(&self, ctx);
    }
}

pub fn new_game() -> PlayingState {
    let mut rng = rand::thread_rng();

    let mut map = vec![MapTile::Empty; (WIDTH * HEIGHT) as usize];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == 0 || x == WIDTH-1 || y == HEIGHT - 1 {
                map[((y * WIDTH) + x) as usize] = MapTile::Wall
            }
        }
    }

    let tetrominos = vec![
        SQUARE,
        LHOOK,
        RHOOK,
        LSHAPE,
        RSHAPE,
        LINE,
        TSHAPE
    ];

    return PlayingState {
        lastTimestamp: instant::Instant::now(),
        clearedTimestamp: instant::Instant::now(),
        map,
        rng,
        isGameover: false,
        clearedRows: vec![],
        currentPiece: rng.gen_range(0, tetrominos.len()),
        nextPiece: rng.gen_range(0, tetrominos.len()),
        tetrominos,
        currentRotation: 0,
        currentPosition: Position {x: WIDTH / 2, y: 0},
        currentRowsCleared: 0,
        currentScore: 0
    };
}