use std::{thread::sleep, time};

use minifb::{Key, Window, WindowOptions};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 40;

const SQUARE_SIZE: usize = 20;

const WINDOW_WIDTH: usize = BOARD_WIDTH * SQUARE_SIZE;
const WINDOW_HEIGHT: usize = BOARD_HEIGHT * SQUARE_SIZE;

type Color = u32;

struct Block {
    pub x: usize,
    pub y: usize,
}

impl Block {
    const SIZE: usize = SQUARE_SIZE;
}

fn main() {

    let mut board: Vec<Color> = vec![0; BOARD_WIDTH * BOARD_HEIGHT];
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = init_window();

    set_block(&mut board, Block{ x: 3, y: 7}, 0xff0000);
    set_block(&mut board, Block{ x: 5, y: 5}, 0x00ff00);
    set_block(&mut board, Block{ x: 5, y: 6}, 0x0000ff);

    let mut running = true;

    while running {
        render_board(&board, &mut buffer);
        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
        sleep(time::Duration::from_millis(100));

        if window.is_key_down(Key::Escape) {
            running = false;
        }
    }
}

fn init_window() -> Window {
    let window = Window::new(
        "TetRust - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    window
}

fn render_board(board: &Vec<u32>, buffer: &mut Vec<u32>) {
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let color = board[y*BOARD_WIDTH + x];
            for ix in 0..Block::SIZE {
                for iy in 0..Block::SIZE {
                    buffer[(y*Block::SIZE)*BOARD_WIDTH*Block::SIZE + iy*BOARD_WIDTH*Block::SIZE + x*Block::SIZE + ix] = color;
                }
            }
        }
    }
}

fn set_block(board: &mut Vec<u32>, b: Block, val: Color) {
    board[b.y * BOARD_WIDTH + b.x] = val;
}
