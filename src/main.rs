use std::{thread::sleep, time};

use minifb::{Key, Window, WindowOptions};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 40;

const SQUARE_SIZE: usize = 20;

fn main() {
    const WIDTH: usize = BOARD_WIDTH * SQUARE_SIZE;
    const HEIGHT: usize = BOARD_HEIGHT * SQUARE_SIZE;

    let mut board: Vec<u32> = vec![0; BOARD_WIDTH * BOARD_HEIGHT];

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    

    let mut window = Window::new(
        "TetRust - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    set_block(&mut board, 3, 7, 0xff0000);
    set_block(&mut board, 5, 5, 0x00ff00);

    let mut running = true;

    while running {
        render_board(&board, &mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        sleep(time::Duration::from_millis(100));

        if window.is_key_down(Key::Escape) {
            running = false;
        }
    }
}

fn render_board(board: &Vec<u32>, buffer: &mut Vec<u32>) {
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let color = board[y*BOARD_WIDTH + x];
            for ix in 0..SQUARE_SIZE {
                for iy in 0..SQUARE_SIZE {
                    buffer[(y*SQUARE_SIZE)*BOARD_WIDTH*SQUARE_SIZE + iy*BOARD_WIDTH*SQUARE_SIZE + x*SQUARE_SIZE + ix] = color;
                }
            }
        }
    }
}

fn set_block(board: &mut Vec<u32>, x: usize, y: usize, val: u32) {
    board[y * BOARD_WIDTH + x] = val;
}
