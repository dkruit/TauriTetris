use rand::seq::SliceRandom;
use rand::thread_rng;

use std::sync::{Arc, Mutex, atomic};
use std::thread;
use std::time::Duration;

use crate::emitter::Emitter;

const SHAPE_SIZE: usize = 4; // A tetromino fills a SHAPE_SIZE by SHAPE_SIZE grid
pub const BOARD_ROWS: usize = 21;
pub const BOARD_COLS: usize = 10;

const TETROMINO_INITIAL_POS: (i32, i32) = (0, (BOARD_COLS-SHAPE_SIZE) as i32 / 2);

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
                rotated_shape[j][SHAPE_SIZE-1-i] = self.shape[i][j];
            }
        }
        self.shape = rotated_shape;
    }

    fn rotate_anti_clockwise(&mut self) {
        let mut rotated_shape: [[i32; SHAPE_SIZE]; SHAPE_SIZE] =  [[0; SHAPE_SIZE]; SHAPE_SIZE];

        for i in 0..SHAPE_SIZE {
            for j in 0..SHAPE_SIZE {
                rotated_shape[SHAPE_SIZE-1-j][i] = self.shape[i][j];
            }
        }
        self.shape = rotated_shape;
    }
}

const N_SHAPES: usize = 7;
const SHAPES: [TetrominoShape; N_SHAPES] = [
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

struct TetrominoShapeGenerator {
    shape_permutation: [TetrominoShape; N_SHAPES],
    current_index: usize,
}

impl TetrominoShapeGenerator {
    pub fn new() -> Self {
        let mut tetromino_shape_generator = TetrominoShapeGenerator {
            shape_permutation: SHAPES.clone(),
            current_index: 0,
        };
        tetromino_shape_generator.shuffle();
        tetromino_shape_generator
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.shape_permutation.shuffle(&mut rng);
        self.current_index = 0;
    }

    pub fn make(name: char) -> Result<TetrominoShape, ()> {
        match SHAPES.iter().find(|shape| shape.name == name){
            None => Err(()),
            Some(shape) => Ok(shape.clone())
        }
    }

    pub fn make_random(&mut self) -> TetrominoShape {
        if self.current_index >= N_SHAPES {
            self.shuffle();
        }
        let result = self.shape_permutation[self.current_index].clone();
        self.current_index += 1;
        result
    }
}

#[derive(Clone)]
pub struct Tetromino{
    pos: (i32, i32), // x and y coordinate of top left corner
    shape: TetrominoShape,
    occupied_positions: Vec<(i32, i32)>
}

impl Tetromino {
    pub fn new(shape: TetrominoShape) -> Self {
        let mut tetromino = Tetromino {
            pos: TETROMINO_INITIAL_POS,
            shape,
            occupied_positions: Vec::new()
        };
        tetromino.set_occupied_positions();
        tetromino
    }

    pub fn get_shape_name(&self) -> char {
        self.shape.name
    }

    pub fn get_occupied_positions(&self) -> &Vec<(i32, i32)> {
        &self.occupied_positions
    }

    pub fn rotate(&mut self, direction: &str) {
        match direction {
            "clockwise" => { self.shape.rotate_clockwise(); }
            "counter-clockwise" => { self.shape.rotate_anti_clockwise(); }
            _ => { println!("Invalid rotation direction: {}", direction); }
        }
        self.set_occupied_positions();
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

#[derive(Debug)]
enum MoveNotAllowedError {
    // Error for illegal moves. If the position is too far left or right, it contains a number
    // describing the number of blocks so that the position can be adjusted accordingly.
    TooFarLeft(i32),
    TooFarRight(i32),
    TooFarDown,
    OverlapsWithOccupied,
}

pub struct Game {
    board: [[char; BOARD_COLS]; BOARD_ROWS],
    current_tetromino: Tetromino,
    tetromino_shape_generator: TetrominoShapeGenerator,

    level: i32,
    total_lines_cleared: i32,
    tick_rate: f64, // tick_rate in ticks per second
    score: i32,

    emitter: Emitter
}

impl Game {
    pub fn new(emitter: Emitter) -> Self {
        let board = [['_'; BOARD_COLS]; BOARD_ROWS];
        let mut game =  Game {
            board,
            current_tetromino: Tetromino::new(TetrominoShapeGenerator::make('T').unwrap()),
            tetromino_shape_generator: TetrominoShapeGenerator::new(),
            level: 0,
            total_lines_cleared: 0,
            tick_rate: 1., // Dummy value
            score: 0,
            emitter,
        };
        game.set_tick_rate(); // Initialize tick rate
        game
    }

    fn set_tick_rate(&mut self) {
        self.tick_rate = 1.25 * (1. + self.level as f64).sqrt();
    }

    pub fn proces_arrow_key(&mut self, key: &str) -> bool {
        let step = match key {
            "ArrowDown" => (1, 0),
            "ArrowLeft" => (0, -1),
            "ArrowRight" => (0, 1),
            _ => {
                println!("Invalid key: {}", key);
                (0, 0)
            }
        };

        match self.check_move(&self.current_tetromino, &step) {
            Ok(_) => {
                self.current_tetromino.move_pos(step);
                self.emitter.emit_tetromino("tick", &self.current_tetromino);
                true
            },
            Err(_) => {
                // If arrow down failed, the tetromino can not move futher down, and the next
                // tetromino has to be set
                if key == "ArrowDown" {
                    self.add_current_tetromino_to_board();
                    if let Ok(_) = self.set_new_tetromino() {
                        self.emitter.emit_tetromino("tick", &self.current_tetromino)
                    }
                }
                false
            }
        }
    }

    pub fn process_hard_drop(&mut self) {
        let step = (1, 0);
        let mut n = 0;
        while let Ok(_) = self.check_move(&self.current_tetromino, &step) {
            self.current_tetromino.move_pos(step);
            n += 1;
        }
        println!("HARD DROP: {} lines", n);
        self.add_current_tetromino_to_board();
        if let Ok(_) = self.set_new_tetromino() {
            self.emitter.emit_tetromino("tick", &self.current_tetromino);
        }
    }

    pub fn process_rotation(&mut self, direction: &str) -> bool {
        println!("Rotation {}", direction);

        let mut tetromino = self.current_tetromino.clone();
        tetromino.rotate(direction);

        // Check if position after rotation is valid
        let result = self.check_move(&tetromino, &(0, 0));

        let success;
        match result {
            Ok(_) => {
                self.current_tetromino = tetromino;
                self.emitter.emit_tetromino("tick", &self.current_tetromino);
                success = true;
            }

            Err(MoveNotAllowedError::TooFarLeft(x)) | Err(MoveNotAllowedError::TooFarRight(x)) => {
                let step = (0, -x);

                match self.check_move(&tetromino, &step) {
                    Ok(()) => {
                        tetromino.move_pos(step);
                        self.current_tetromino = tetromino;
                        self.emitter.emit_tetromino("tick", &self.current_tetromino);
                        success = true;
                    }
                    Err(_) => { success = false; }
                }
            }

            Err(_) => { success = false; }
        }
        success
    }

    fn check_row_full(&self, row_index: usize) -> bool {
        let mut row_full = true;
        for j in 0..BOARD_COLS {
            if self.board[row_index][j] == '_' {
                row_full = false;
                break;
            }
        }
        row_full
    }

    fn clear_full_rows(&mut self) {
        let mut n_cleared: i32 = 0;

        for i in (0..BOARD_ROWS).rev() {
            // While loop because if the row is full the other rows are moved down.
            // Another full row can take the place of the cleared row, which also has to be cleared.
            while self.check_row_full(i) {
                println!("Clearing row {}", i);
                // Move all the rows above i one position down
                for other_row_index in (0..i).rev() {
                    for j in 0..BOARD_COLS {
                        self.board[other_row_index+1][j] = self.board[other_row_index][j];
                    }
                }
                n_cleared += 1;
            }
        }

        // Make the top rows clear
        for i in 0..n_cleared as usize {
            for j in 0..BOARD_COLS {
                self.board[i][j] = '_';
            }
        }

        // Update board and score if rows have been cleared
        if n_cleared > 0 {
            self.emitter.emit_board("board", &self.board);
            println!("Cleared {} rows", n_cleared);
            self.update_score(n_cleared);
            self.update_level(n_cleared);
        }
    }

    fn check_move(
        &self,
        tetromino: &Tetromino,
        step: &(i32, i32)) -> Result<(), MoveNotAllowedError> {

        // Check if tetromino goes too far left or right.
        // This is checked first, so that the position can be adjusted
        let mut largest_dist = 0;
        let mut err: Option<MoveNotAllowedError> = None;
        for occupied_pos in &tetromino.occupied_positions {
            let pos_after_move = (occupied_pos.0 + step.0, occupied_pos.1 + step.1);
            if pos_after_move.1 < 0 {
                let dist = -pos_after_move.1;
                if dist > largest_dist {
                    largest_dist = dist;
                    err = Some(MoveNotAllowedError::TooFarLeft(-dist))
                }
            } else if pos_after_move.1 >= BOARD_COLS as i32 {
                let dist = pos_after_move.1 + 1 - BOARD_COLS as i32 ;
                if dist > largest_dist {
                    largest_dist = dist;
                    err = Some(MoveNotAllowedError::TooFarRight(dist))
                }
            }
        }
        if let Some(err_value) = err { return Err(err_value); }

        // Then check if it is too far down, or colliding with an already occupied spot.
        // This will not be adjusted and means the move is not valid.
        for occupied_pos in &tetromino.occupied_positions {
            let pos_after_move = (occupied_pos.0 + step.0, occupied_pos.1 + step.1);
            if pos_after_move.0 >= BOARD_ROWS as i32 {
                return Err(MoveNotAllowedError::TooFarDown)
            }
            else if self.board[pos_after_move.0 as usize][pos_after_move.1 as usize] != '_' {
                 // This has to be checked last. If a tetromino is (partly) out of the board and
                // overlaps with an occupied spot, it should first be addressed that the
                // tetromino is not completely on the board.
                return Err(MoveNotAllowedError::OverlapsWithOccupied);
            }
        }
        Ok(())
    }

    fn add_current_tetromino_to_board(&mut self) {
        // Makes the current tetromino part of the frozen blocks on the board
        // Clears full rows if the current block completes them
        for occupied_pos in &self.current_tetromino.occupied_positions {
            let i = occupied_pos.0 as usize;
            let j = occupied_pos.1 as usize;
            self.board[i][j] = self.current_tetromino.shape.name;
        }
        self.clear_full_rows();
        self.emitter.emit_board("board", &self.board);
    }

    /// Update the score based on the number of cleared rows.
    fn update_score(&mut self, n_lines_cleared: i32) {
        let base_score = match n_lines_cleared {
            0 => 0,
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0 // This should not occur
        };
        self.score += base_score * (self.level + 1);
        println!("Updated score to {}", self.score);
        self.emitter.emit_number("score", self.score);
    }

    fn lines_needed_for_level(&self, level: i32) -> i32 {
        if level < 1 {
            10
        }
        else {
            10 * level + self.lines_needed_for_level(level-1)
        }
    }

    fn update_level(&mut self, n_lines_cleared: i32) {
        self.total_lines_cleared += n_lines_cleared;
        let lines_needed = self.lines_needed_for_level(self.level);

        println!("{} / {} to next level", self.total_lines_cleared, lines_needed);

        if self.total_lines_cleared >= lines_needed{
            println!("Level up!");
            self.level += 1;
            self.emitter.emit_number("level", self.level);
            self.set_tick_rate();
        }
    }
    /// Forwards the game a single tick. Returns true if the tick succeeded. Returns false if
    /// the tick fails because the player is game-over.
    pub fn tick(&mut self) -> bool {
        let step = (1, 0);
        let result = self.check_move(&self.current_tetromino, &step);
        match result {
            Ok(_) => {
                self.current_tetromino.move_pos(step);
            },
            Err(err) => {
                println!("Can not move tetromino down: {:?}", err);
                self.add_current_tetromino_to_board();
                match self.set_new_tetromino() {
                    Ok(_) => {}
                    Err(_) => { return false; }
                }
            }
        }
        self.emitter.emit_tetromino("tick", &self.current_tetromino);
        true
    }

    fn set_new_tetromino(&mut self) -> Result<(), ()>{
        self.current_tetromino = Tetromino::new(self.tetromino_shape_generator.make_random());

        // Game over if newly placed block overlaps with board
        match self.check_move(&self.current_tetromino, &(0, 0)) {
            Ok(_) => { Ok(())},
            Err(MoveNotAllowedError::OverlapsWithOccupied) => {
                self.emitter.emit_string("game_over", "GAME OVER".to_string());
                Err(())
            }
            // Other err is not expected to occur.
            Err(_) => panic!("Unexpected error encountered when creating next tetromino.")
        }
    }

    pub fn reset(&mut self) {
        self.board = [['_'; BOARD_COLS]; BOARD_ROWS];
        self.current_tetromino = Tetromino::new(self.tetromino_shape_generator.make_random());
        self.level = 0;
        self.score = 0;
        self.set_tick_rate();
        self.emitter.emit_tetromino("tick", &self.current_tetromino);
        self.emitter.emit_number("score", self.score);
        self.emitter.emit_number("level", self.level);
        self.emitter.emit_board("board", &self.board);
    }

    pub fn get_tick_rate(&self) -> f64 {
        self.tick_rate
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
        GameRunner{
            game: Arc::new(Mutex::new(game)),
            running: Arc::new(atomic::AtomicBool::new(false))
        }
    }

    pub fn get_running(&self) -> bool {
        self.running.load(atomic::Ordering::SeqCst)
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
                    let success = game.tick();
                    if !success {
                        // Game over, stop running and end loop
                        self_clone.running.store(false, atomic::Ordering::SeqCst);
                        break
                    }
                    sleep_time = 1. / game.get_tick_rate();
                }
                thread::sleep(Duration::from_secs_f64(sleep_time));
            }
        });
    }

    pub fn reset(&self) {
        self.running.store(false, atomic::Ordering::SeqCst);
        let mut game = self.game.lock().unwrap();
        game.reset();
    }
}
