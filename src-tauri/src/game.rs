use std::sync::{Arc, Mutex, atomic};
use std::thread;
use std::time::Duration;

use crate::emitter::Emitter;

const SHAPE_SIZE: usize = 4; // A tetromino fills a SHAPE_SIZE by SHAPE_SIZE grid
const BOARD_ROWS: usize = 7;
const BOARD_COLS: usize = 7;

const TETROMINO_INITIAL_POS: (i32, i32) = (-1, (BOARD_COLS-SHAPE_SIZE) as i32 / 2);

#[derive(Clone)]
struct TetrominoShape {
    name: char,
    orientation: i32, // Rotation in degrees (0, 90, 180 or 270)
    shape: [[i32; SHAPE_SIZE]; SHAPE_SIZE],
}

impl TetrominoShape {
    fn rotate_clockwise(&mut self) {
        let mut rotated_shape: [[i32; SHAPE_SIZE]; SHAPE_SIZE] = [[0; SHAPE_SIZE]; SHAPE_SIZE];

        for i in 0..SHAPE_SIZE {
            for j in 0..SHAPE_SIZE {
                rotated_shape[j][SHAPE_SIZE-i] = self.shape[i][j];
            }
        }
        self.shape = rotated_shape;
    }

    fn rotate_anti_clockwise(&mut self) {
        let mut rotated_shape: [[i32; SHAPE_SIZE]; SHAPE_SIZE] =  [[0; SHAPE_SIZE]; SHAPE_SIZE];

        for i in 0..SHAPE_SIZE {
            for j in 0..SHAPE_SIZE {
                rotated_shape[SHAPE_SIZE-j][i] = self.shape[i][j];
            }
        }
        self.shape = rotated_shape;
    }
}

const SHAPES: [TetrominoShape; 7] = [
    TetrominoShape {
        name:'I',
        orientation: 0,
        shape: [
            [0,0,0,0],
            [0,0,0,0],
            [1,1,1,1],
            [0,0,0,0]] },
    TetrominoShape {
        name:'J',
        orientation: 0,
        shape: [
            [0,0,0,0],
            [0,1,1,1],
            [0,0,0,1],
            [0,0,0,0]] },
    TetrominoShape {
        name:'L',
        orientation: 0,
        shape: [
            [0,0,0,0],
            [0,1,1,1],
            [0,1,0,0],
            [0,0,0,0]] },
    TetrominoShape {
        name:'O',
        orientation: 0,
        shape: [
            [0,0,0,0],
            [0,1,1,0],
            [0,1,1,0],
            [0,0,0,0]] },
    TetrominoShape {
        name:'S',
        orientation: 0,
        shape: [
            [0,0,0,0],
            [0,0,1,1],
            [0,1,1,0],
            [0,0,0,0]] },
    TetrominoShape {
        name:'T',
        orientation: 0,
        shape: [
            [0,0,0,0],
            [0,1,1,1],
            [0,0,1,0],
            [0,0,0,0]] },
    TetrominoShape {
        name:'Z',
        orientation: 0,
        shape: [
            [0,0,0,0],
            [0,1,1,0],
            [0,0,1,1],
            [0,0,0,0]] },
];

pub struct Tetromino{
    pos: (i32, i32), // x and y coordinate of top left corner
    shape: TetrominoShape,
    occupied_positions: Vec<(i32, i32)>
}

impl Tetromino {
    pub fn new(shape_name: char) -> Self {
        let shape = SHAPES
            .iter()
            .find(|shape| shape.name == shape_name)
            .unwrap()
            .clone();

        let mut tetromino = Tetromino {
            pos: (TETROMINO_INITIAL_POS),
            shape,
            occupied_positions: Vec::new()
        };
        tetromino.set_occupied_positions();
        return tetromino;
    }

    pub fn get_shape_name(&self) -> char {
        return self.shape.name;
    }

    pub fn get_occupied_positions(&self) -> &Vec<(i32, i32)> {
        return &self.occupied_positions;
    }

    pub fn move_pos(&mut self, step: (i32, i32)) {
        self.pos = (self.pos.0 + step.0, self.pos.1 + step.1);
        self.set_occupied_positions();
    }

    fn set_occupied_positions(&mut self) {
        // Calculate the positions of the board which are occupied by the Tetromino, based on its
        // position and shape

        let mut occupied_postions: Vec<(i32, i32)> = Vec::new();

        for i in 0..SHAPE_SIZE {
            for j in 0..SHAPE_SIZE {
                if self.shape.shape[i][j] == 1 {
                    let occupied_pos: (i32, i32) = (
                        self.pos.0 + (i as i32),
                        self.pos.1 + (j as i32)
                    );
                    occupied_postions.push(occupied_pos)
                }
            }
        }
        self.occupied_positions = occupied_postions;
    }
}

pub struct Game {
    board: [[char; BOARD_COLS]; BOARD_ROWS],
    current_tetromino: Tetromino,

    level: i32,
    tick_rate: f64, // tick_rate in ticks per second

    emitter: Emitter
}

impl Game {
    pub fn new(emitter: Emitter) -> Self {
        let board = [['_'; BOARD_COLS]; BOARD_ROWS];
        let mut game =  Game {
            board,
            current_tetromino: Tetromino::new('T'),
            level: 1,
            tick_rate: 1., // Dummy value
            emitter,
        };
        game.set_tick_rate(); // Initialize tick rate
        return game;
    }

    fn set_tick_rate(&self) -> f64 {
        return 1.5 * (self. level as f64);
    }

    pub fn tick(&mut self) {
        self.current_tetromino.move_pos((1, 0));
        let result = format!("{:?} {}",
                             self.current_tetromino.occupied_positions,
                             self.current_tetromino.shape.name);
        self.emitter.emit_tetromino("tick", &self.current_tetromino);
    }

    pub fn reset(&mut self) {
        self.board = [['_'; BOARD_COLS]; BOARD_ROWS];
        self.current_tetromino = Tetromino::new('T');
        self.level = 1;
        self.set_tick_rate();
        self.emitter.emit_tetromino("tick", &self.current_tetromino);
    }

    pub fn get_tick_rate(&self) -> f64 {
        return self.tick_rate;
    }
}

// Declare a shared game struct to use the state of the game
// Arc Mutex makes it usable in different threads
#[derive(Clone)]
pub struct GameRunner {
    pub game: Arc<Mutex<Game>>,
    running: Arc<atomic::AtomicBool>
}

impl GameRunner {
    pub fn new(game: Game) -> Self {
        return GameRunner{
            game: Arc::new(Mutex::new(game)),
            running: Arc::new(atomic::AtomicBool::new(false))
        };
    }

    pub fn run(&self) {
        // Early return if running is true: the game is already started
        if self.running.load(atomic::Ordering::SeqCst) {
            println!("Game is already running!");
            return;
        }

        // Set running flag to true
        self.running.store(true, atomic::Ordering::SeqCst);

        // Clone self to move it to the background thread
        let self_clone = self.clone();

        // Spawn a thread to increment the game at set intervals
        thread::spawn(move || {
            let mut sleep_time;

            // Continue as long as running is true
            while self_clone.running.load(atomic::Ordering::SeqCst) {
                {
                    let mut game = self_clone.game.lock().unwrap();
                    game.tick();
                    sleep_time = 1. / game.get_tick_rate();
                }
                thread::sleep(Duration::from_secs_f64(sleep_time));
            }
        });
    }

    pub fn pause(&self) {
        self.running.store(false, atomic::Ordering::SeqCst);
    }

    pub fn reset(&self) {
        self.running.store(false, atomic::Ordering::SeqCst);
        let mut game = self.game.lock().unwrap();
        game.reset();
    }
}
