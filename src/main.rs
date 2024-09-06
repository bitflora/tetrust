use std::{thread::{current, sleep}, time::{Duration, SystemTime}};
use rand::{rngs::ThreadRng, Rng};
use minifb::{Key, KeyRepeat, Window, WindowOptions};

const TICK_DURATION: Duration = Duration::from_secs(1);
const SLEEP_DURATION: Duration = Duration::from_millis(10);

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 25;

const SQUARE_SIZE: usize = 30;

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

struct Board {
    data: Vec<Color>,
}

impl Default for Board {
    fn default() -> Board {
        Board { data: vec![0; BOARD_WIDTH * BOARD_HEIGHT] }
    }
}

impl Board {
    fn emplace(&mut self, shape: &Shape) {
        for b in &shape.blocks {
            self.set_block(&b, shape.color)
        }
    }

    fn set_block(&mut self, b: &Block, val: Color) {
        self.data[b.y * BOARD_WIDTH + b.x] = val;
    }

    // TODO: maybe overload []
    fn color_at(&self, x: usize, y: usize) -> Color {
        self.data[y*BOARD_WIDTH + x]
    }

    fn is_filled(&self, x: usize, y: usize) -> bool {
        self.color_at(x, y) != 0x000000
    }
}


// TODO: learn more about cool ways to do enum stuff
enum ShapeSpecies {
    Line,
    Square,
    LRight,
    LLeft,
    SquiggleRight,
    SquiggleLeft,
    Hat,
}

struct Shape {
    species: ShapeSpecies,
    blocks: Vec<Block>,
    color: Color,
}

impl Shape {
    fn random(start: &Block, rng: &mut ThreadRng) -> Shape {
        let c: Color = rng.gen();
        let r = rng.gen_range(0..7);
        println!("shape: {r}");
        return match r {
            0 => Shape::line(&start, c),
            1 => Shape::square(&start, c),
            2 => Shape::l_right(&start, c),
            3 => Shape::l_left(&start, c),
            4 => Shape::squiggle_right(&start, c),
            5 => Shape::squiggle_left(&start, c),
            6 => Shape::hat(&start, c),
            _ => Shape::line(&start, c),
        };
    }

    fn line(center: &Block, color: Color) -> Shape {
        return Shape {
            species: ShapeSpecies::Line,
            blocks: vec![
                Block{ x: center.x - 1, y: center.y },
                center.clone(),
                Block{ x: center.x + 1, y: center.y },
                Block{ x: center.x + 2, y: center.y },
            ],
            color: color
        };
    }

    fn square(upperleft: &Block, color: Color) -> Shape {
        return Shape {
            species: ShapeSpecies::Square,
            blocks: vec![
                upperleft.clone(),
                Block{ x: upperleft.x + 1, y: upperleft.y },
                Block{ x: upperleft.x, y: upperleft.y + 1 },
                Block{ x: upperleft.x + 1, y: upperleft.y + 1 },
            ],
            color: color
        };
    }

    fn l_right(center: &Block, color: Color) -> Shape {
        return Shape {
            species: ShapeSpecies::LRight,
            blocks: vec![
                Block{ x: center.x - 1, y: center.y + 1 },
                Block{ x: center.x, y: center.y + 1 },
                Block{ x: center.x + 1, y: center.y + 1 },
                Block{ x: center.x + 1, y: center.y },
            ],
            color: color
        };
    }

    fn l_left(center: &Block, color: Color) -> Shape {
        return Shape {
            species: ShapeSpecies::LLeft,
            blocks: vec![
                Block{ x: center.x - 1, y: center.y + 1 },
                Block{ x: center.x, y: center.y + 1 },
                Block{ x: center.x + 1, y: center.y + 1 },
                Block{ x: center.x - 1, y: center.y },
            ],
            color: color
        };
    }

    fn squiggle_right(center: &Block, color: Color) -> Shape {
        return Shape {
            species: ShapeSpecies::SquiggleRight,
            blocks: vec![
                Block{ x: center.x - 1, y: center.y },
                Block{ x: center.x - 1, y: center.y + 1 },
                Block{ x: center.x, y: center.y + 1 },
                Block{ x: center.x, y: center.y + 2 },
            ],
            color: color
        };
    }

    fn squiggle_left(center: &Block, color: Color) -> Shape {
        return Shape {
            species: ShapeSpecies::SquiggleLeft,
            blocks: vec![
                Block{ x: center.x + 1, y: center.y },
                Block{ x: center.x + 1, y: center.y + 1 },
                Block{ x: center.x, y: center.y + 1 },
                Block{ x: center.x, y: center.y + 2 },
            ],
            color: color
        };
    }

    fn hat(center: &Block, color: Color) -> Shape {
        return Shape {
            species: ShapeSpecies::SquiggleLeft,
            blocks: vec![
                center.clone(),
                Block{ x: center.x - 1, y: center.y + 1 },
                Block{ x: center.x, y: center.y + 1 },
                Block{ x: center.x + 1, y: center.y + 1 },
            ],
            color: color
        };
    }

    fn move_down(&mut self, board: &mut Board) -> bool {
        for block in &self.blocks {
            if block.y + 1 >= BOARD_HEIGHT || board.is_filled(block.x, block.y+1) {
                board.emplace(&self);
                return false;
            }
        }
        for block in &mut self.blocks {
            block.y += 1;
        }
        true
    }

    fn drop(&mut self, board: &mut Board) {
        while self.move_down(board) {
            
        }
    }

    fn move_right(&mut self, board: &Board) -> bool {
        for block in &self.blocks {
            if block.x + 1 >= BOARD_WIDTH || board.is_filled(block.x + 1, block.y) {
                return false;
            }
        }

        for block in &mut self.blocks {
            block.x += 1;
        }
        return true;
    }

    fn move_left(&mut self, board: &Board) -> bool {
        for block in &self.blocks {
            if block.x == 0 || board.is_filled(block.x - 1, block.y) {
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
    let mut board: Board = Board::default();
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = init_window();
    let mut rng = rand::thread_rng();

    let top_center = Block{ x: BOARD_WIDTH / 2, y: 0};

    board.set_block(&Block{ x: 3, y: BOARD_HEIGHT-1}, rng.gen());
    board.set_block(&Block{ x: 5, y: BOARD_HEIGHT-2}, rng.gen());
    board.set_block(&Block{ x: 5, y: BOARD_HEIGHT-5}, rng.gen());

    let mut running = true;

    let mut curr_shape: Shape = Shape::random(&top_center, &mut rng);

    let mut tick_start = SystemTime::now();
    while running {
        render_board(&board, &mut buffer, &curr_shape);
        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
        
        window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key|
            match key {
                Key::Escape => {
                    running = false
                },
                Key::Right => { 
                    curr_shape.move_right(&board);
                },
                Key::Left => {
                    curr_shape.move_left(&board);
                },
                Key::Space => {
                    curr_shape.drop(&mut board);
                    curr_shape = Shape::random(&top_center, &mut rng);
                },
                Key::Down => {
                    if !curr_shape.move_down(&mut board) {
                        curr_shape = Shape::random(&top_center, &mut rng);
                    }
                },
                _ => {},
            }
        );
        
        match tick_start.elapsed()  {
            Ok(n) => {
                if n > TICK_DURATION {
                    if !curr_shape.move_down(&mut board) {
                        curr_shape = Shape::random(&top_center, &mut rng);
                    }
                    tick_start = SystemTime::now();
                }
            },
            Err(_) => {},
        }

        sleep(SLEEP_DURATION);
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

fn render_board(board: &Board, buffer: &mut Vec<u32>, curr_shape: &Shape) {
    let mut draw_block = |x: usize, y: usize, color: Color| {
        for ix in 0..Block::SIZE {
            for iy in 0..Block::SIZE {
                buffer[(y*Block::SIZE)*BOARD_WIDTH*Block::SIZE + iy*BOARD_WIDTH*Block::SIZE + x*Block::SIZE + ix] = color;
            }
        }
    };

    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let color = board.color_at(x, y);
            draw_block(x, y, color);
        }
    }

    for block in &curr_shape.blocks {
        draw_block(block.x, block.y, curr_shape.color);
    }
}
