use std::{thread::{current, sleep}, time};

use minifb::{Key, Window, WindowOptions};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 40;

const SQUARE_SIZE: usize = 20;

const WINDOW_WIDTH: usize = BOARD_WIDTH * SQUARE_SIZE;
const WINDOW_HEIGHT: usize = BOARD_HEIGHT * SQUARE_SIZE;

type Color = u32;

#[derive(Copy, Clone)]
struct Block {
    pub x: usize,
    pub y: usize,
}

impl Block {
    const SIZE: usize = SQUARE_SIZE;
}

struct Shape {
    blocks: Vec<Block>,
    color: Color,
}

impl Shape {
    fn line(left: Block) -> Shape {
        return Shape {
            blocks: vec![
                left.clone(),
                Block{ x: left.x + 1, y: left.y },
                Block{ x: left.x + 2, y: left.y },
                Block{ x: left.x + 3, y: left.y },
            ],
            color: 0x770077
        };
    }

    fn square(upperleft: Block) -> Shape {
        return Shape {
            blocks: vec![
                upperleft.clone(),
                Block{ x: upperleft.x + 1, y: upperleft.y },
                Block{ x: upperleft.x, y: upperleft.y + 1 },
                Block{ x: upperleft.x + 1, y: upperleft.y + 1 },
            ],
            color: 0x770077
        };
    }

    fn move_right(&mut self) -> bool {
        for block in &self.blocks {
            if block.x + 1 >= BOARD_WIDTH {
                return false;
            }
        }

        for block in &mut self.blocks {
            block.x += 1;
        }
        return true;
    }

    fn move_left(&mut self) -> bool {
        for block in &self.blocks {
            if block.x == 0 {
                return false;
            }
        }

        for block in &mut self.blocks {
            block.x -= 1;
        }
        return true;
    }
}

fn main() {

    let mut board: Vec<Color> = vec![0; BOARD_WIDTH * BOARD_HEIGHT];
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = init_window();

    set_block(&mut board, Block{ x: 3, y: 7}, 0xff0000);
    set_block(&mut board, Block{ x: 5, y: 5}, 0x00ff00);
    set_block(&mut board, Block{ x: 5, y: 6}, 0x0000ff);

    let mut running = true;

    let mut curr_shape: Shape = Shape::square(Block{x: 0, y: 0});

    while running {
        render_board(&board, &mut buffer, &curr_shape);
        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
        sleep(time::Duration::from_millis(100));

        if window.is_key_down(Key::Escape) {
            running = false;
        } else if window.is_key_down(Key::Right) {
            curr_shape.move_right();
        } else if window.is_key_down(Key::Left) {
            curr_shape.move_left();
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

fn render_board(board: &Vec<u32>, buffer: &mut Vec<u32>, curr_shape: &Shape) {
    let mut draw_block = |x: usize, y: usize, color: Color| {
        for ix in 0..Block::SIZE {
            for iy in 0..Block::SIZE {
                buffer[(y*Block::SIZE)*BOARD_WIDTH*Block::SIZE + iy*BOARD_WIDTH*Block::SIZE + x*Block::SIZE + ix] = color;
            }
        }
    };

    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let color = board[y*BOARD_WIDTH + x];
            draw_block(x, y, color);
        }
    }

    for block in &curr_shape.blocks {
        draw_block(block.x, block.y, curr_shape.color);
    }
}

fn set_block(board: &mut Vec<u32>, b: Block, val: Color) {
    board[b.y * BOARD_WIDTH + b.x] = val;
}
