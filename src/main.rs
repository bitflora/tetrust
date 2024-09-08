use std::{thread::sleep, time::{Duration, SystemTime}};
use rand::{rngs::ThreadRng, Rng};
use minifb::{Key, KeyRepeat, Window, WindowOptions};

const TICK_DURATION: Duration = Duration::from_secs(1);
const SLEEP_DURATION: Duration = Duration::from_millis(10);

const BOARD_WIDTH: usize = 12;
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
        let mut ret = Board { data: vec![0; BOARD_WIDTH * BOARD_HEIGHT] };
        // A lot is easier if we have hard-coded borders that we never mess with.
        let white: Color = 0xffffff;
        for i in 0..BOARD_HEIGHT {
            ret.set_block(&Block{x:0, y: i}, white);
            ret.set_block(&Block{x:BOARD_WIDTH-1, y: i}, white);
        }
        for x in 0..BOARD_WIDTH {
            ret.set_block(&Block{x: x, y: BOARD_HEIGHT-1}, white);
        }
        ret
    }
}

impl Board {
    const BLANK: Color = 0x000000;

    const PLAYABLE_WIDTH : std::ops::Range<usize> = 1..BOARD_WIDTH-1;
    const PLAYABLE_HEIGHT : std::ops::Range<usize> = 1..BOARD_HEIGHT-1;
    const DOOM_ROW: usize = 1;

    fn check_rows(&mut self) -> u32 {
        let mut rows_removed: u32 = 0;
        let full_row = |board: &Board, row: usize| {
            for x in Board::PLAYABLE_WIDTH {
                if ! board.is_filled(x, row) {
                    return false;
                }
            }
            true
        };
        for row in Board::PLAYABLE_HEIGHT {
            if full_row( &self, row) {
                // TODO: scoring
                rows_removed += 1;
                for x in Board::PLAYABLE_WIDTH {
                    self.set_block(&Block{x: x, y: row}, Board::BLANK);
                }
                for prev_row in (1..=row).rev() {
                    for x in Board::PLAYABLE_WIDTH {
                        self.set_block(&Block{x: x, y: prev_row}, self.color_at(x, prev_row-1));
                    }
                }
            }
        }
        rows_removed
    }

    fn is_dead(&self) -> bool {
        for x in Board::PLAYABLE_WIDTH {
            if self.is_filled(x, Board::DOOM_ROW) {
                return true;
            }
        }
        false
    }

    fn emplace(&mut self, shape: &Shape) -> u32 {
        for b in &shape.blocks {
            self.set_block(&b, shape.color);
        }
        self.check_rows()
    }

    fn set_block(&mut self, b: &Block, val: Color) {
        self.data[b.y * BOARD_WIDTH + b.x] = val;
    }

    // TODO: maybe overload []
    fn color_at(&self, x: usize, y: usize) -> Color {
        self.data[y*BOARD_WIDTH + x]
    }

    fn is_filled(&self, x: usize, y: usize) -> bool {
        self.color_at(x, y) != Board::BLANK
    }

    fn valid_move(&self, block: &Block) -> bool {
        return !( !Board::PLAYABLE_WIDTH.contains(&block.x)
            || block.y >= BOARD_HEIGHT
            || self.is_filled(block.x, block.y))
    }
}


// TODO: learn more about cool ways to do enum stuff

#[derive(Clone)]
enum ShapeSpecies {
    Line,
    Square,
    LRight,
    LLeft,
    SquiggleRight,
    SquiggleLeft,
    Hat,
}

// TODO: Is there a nice way to abstract this out so it can't have invalid states
//  but easily allows me to increment and decrement?
type Rotation = u8;

struct Shape {
    species: ShapeSpecies,
    blocks: Vec<Block>,
    color: Color,
    rotation: Rotation, // Sure would be nice if I could default this
}


fn main() {
    let mut board: Board = Board::default();
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = init_window();
    let mut rng = rand::thread_rng();

    // I'd prefer to allow the shape to rotate into the negative space,
    // but then I'd need to change all my types and cast a lot
    let top_center = Block{ x: BOARD_WIDTH / 2, y: 1};

    // board.set_block(&Block{ x: 3, y: BOARD_HEIGHT-1}, rng.gen());
    // board.set_block(&Block{ x: 5, y: BOARD_HEIGHT-2}, rng.gen());
    // board.set_block(&Block{ x: 5, y: BOARD_HEIGHT-5}, rng.gen());

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
                Key::Up | Key::R => {
                    curr_shape.rotate_right(&board);
                },
                Key::Right | Key::D => { 
                    curr_shape.move_right(&board);
                },
                Key::Left | Key::A => {
                    curr_shape.move_left(&board);
                },
                Key::Space => {
                    curr_shape.drop(&mut board);
                    curr_shape = Shape::random(&top_center, &mut rng);
                },
                Key::Down | Key::S => {
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

        if board.is_dead() {
            // TODO: a more glorious death
            running = false;
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


impl Shape {
    fn random(start: &Block, rng: &mut ThreadRng) -> Shape {
        let c: Color = rng.gen();
        const SHAPE: [ShapeSpecies; 7] = [
            ShapeSpecies::Line,
            ShapeSpecies::Square,
            ShapeSpecies::LRight,
            ShapeSpecies::LLeft,
            ShapeSpecies::SquiggleRight,
            ShapeSpecies::SquiggleLeft,
            ShapeSpecies::Hat,
        ];
        let r = rng.gen_range(0..SHAPE.len());
        let chosen = SHAPE[r].clone();
        println!("shape: {r}");
        Shape {
            species: chosen.clone(),
            blocks: Shape::blocks_for(chosen, &start, 0),
            color: c,
            rotation: 0,
        }
    }

    fn blocks_for(species: ShapeSpecies, center: &Block, rotation: Rotation) -> Vec<Block> {
        match species {
            ShapeSpecies::Line => 
                if rotation % 2 == 0 {
                    vec![
                        center.clone(),
                        Block{ x: center.x + 1, y: center.y },
                        Block{ x: center.x + 2, y: center.y },
                        Block{ x: center.x + 3, y: center.y },
                    ]
                } else {
                    vec![
                        center.clone(),
                        Block{ x: center.x, y: center.y + 1 },
                        Block{ x: center.x, y: center.y + 2 },
                        Block{ x: center.x, y: center.y + 3 },
                    ]
                },
            ShapeSpecies::Square =>
                vec![
                    center.clone(),
                    Block{ x: center.x + 1, y: center.y },
                    Block{ x: center.x, y: center.y + 1 },
                    Block{ x: center.x + 1, y: center.y + 1 },
                ],
            ShapeSpecies::LRight => // Can currently climb back up
                if rotation == 0 {
                    vec![
                        Block{ x: center.x - 1, y: center.y + 1 },
                        Block{ x: center.x, y: center.y + 1 },
                        Block{ x: center.x + 1, y: center.y + 1 },
                        Block{ x: center.x + 1, y: center.y },
                    ]
                } else if rotation == 1 {
                    vec![
                        Block{ x: center.x - 1, y: center.y - 1 },
                        Block{ x: center.x - 1, y: center.y },
                        Block{ x: center.x - 1, y: center.y + 1 },
                        Block{ x: center.x, y: center.y + 1 },
                    ]
                } else if rotation == 2 {
                    vec![
                        Block{ x: center.x - 1, y: center.y - 1 },
                        Block{ x: center.x, y: center.y - 1 },
                        Block{ x: center.x + 1, y: center.y - 1 },
                        Block{ x: center.x - 1, y: center.y },
                    ]
                } else {
                    vec![
                        Block{ x: center.x + 1, y: center.y - 1 },
                        Block{ x: center.x + 1, y: center.y },
                        Block{ x: center.x + 1, y: center.y + 1 },
                        Block{ x: center.x, y: center.y - 1 },
                    ]
                },
            ShapeSpecies::LLeft => // These states are not in the right order
                if rotation == 0 {
                    vec![
                        Block{ x: center.x - 1, y: center.y + 1 },
                        Block{ x: center.x, y: center.y + 1 },
                        Block{ x: center.x + 1, y: center.y + 1 },
                        Block{ x: center.x - 1, y: center.y },
                    ]
                } else if rotation == 1 {
                    vec![
                        Block{ x: center.x + 1, y: center.y - 1 },
                        Block{ x: center.x + 1, y: center.y },
                        Block{ x: center.x + 1, y: center.y + 1 },
                        Block{ x: center.x, y: center.y + 1 },
                    ]
                } else if rotation == 2 {
                    vec![
                        Block{ x: center.x - 1, y: center.y - 1 },
                        Block{ x: center.x, y: center.y - 1 },
                        Block{ x: center.x + 1, y: center.y - 1 },
                        Block{ x: center.x + 1, y: center.y },
                    ]
                } else {
                    vec![
                        Block{ x: center.x + 1, y: center.y - 1 },
                        Block{ x: center.x + 1, y: center.y },
                        Block{ x: center.x + 1, y: center.y + 1 },
                        Block{ x: center.x, y: center.y - 1 },
                    ]
                },
            ShapeSpecies::SquiggleRight =>
                if rotation % 2 == 0 {
                    vec![
                        Block{ x: center.x - 1, y: center.y },
                        Block{ x: center.x - 1, y: center.y + 1 },
                        Block{ x: center.x, y: center.y + 1 },
                        Block{ x: center.x, y: center.y + 2 },
                    ]
                } else {
                    vec![
                        center.clone(),
                        Block{ x: center.x + 1, y: center.y },
                        Block{ x: center.x, y: center.y + 1 },
                        Block{ x: center.x - 1, y: center.y + 1 },
                    ]
                },
            ShapeSpecies::SquiggleLeft =>
                if rotation % 2 == 0 {
                    vec![
                        Block{ x: center.x + 1, y: center.y },
                        Block{ x: center.x + 1, y: center.y + 1 },
                        Block{ x: center.x, y: center.y + 1 },
                        Block{ x: center.x, y: center.y + 2 },
                    ]
                } else {
                    vec![
                        center.clone(),
                        Block{ x: center.x - 1, y: center.y },
                        Block{ x: center.x, y: center.y + 1 },
                        Block{ x: center.x + 1, y: center.y + 1 },
                    ]
                },
            ShapeSpecies::Hat => 
                if rotation == 0 {
                    vec![
                        center.clone(),
                        Block{ x: center.x - 1, y: center.y },
                        Block{ x: center.x, y: center.y - 1 },
                        Block{ x: center.x + 1, y: center.y },
                    ]
                } else if rotation == 1 {
                    vec![
                        center.clone(),
                        Block{ x: center.x, y: center.y - 1 },
                        Block{ x: center.x + 1, y: center.y },
                        Block{ x: center.x, y: center.y + 1 },
                    ]
                } else if rotation == 2 {
                    vec![
                        center.clone(),
                        Block{ x: center.x - 1, y: center.y },
                        Block{ x: center.x, y: center.y + 1 },
                        Block{ x: center.x + 1, y: center.y },
                    ]
                } else {
                    vec![
                        center.clone(),
                        Block{ x: center.x, y: center.y - 1 },
                        Block{ x: center.x - 1, y: center.y },
                        Block{ x: center.x, y: center.y + 1 },
                    ]
                },
        }
    }

    fn rotate_right(&mut self, board: &Board) -> bool {
        let new_rot = if self.rotation >= 3 { 0 } else { self.rotation + 1 };
        // TODO: track center separately (and indealy with a pointer)
        let new_blocks = Shape::blocks_for(self.species.clone(), &self.blocks[0], new_rot);
        for block in &new_blocks {
            if !board.valid_move(&block) {
                return false;
            }
        }
        self.rotation = new_rot;
        self.blocks = new_blocks;
        true
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
        while self.move_down(board) { }
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