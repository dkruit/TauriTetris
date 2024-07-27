use std::sync::{Arc, Mutex, atomic};
use std::thread;
use std::time::Duration;

use crate::emitter::Emitter;
use crate::tetromino::{Tetromino, TetrominoShapeGenerator, SHAPE_SIZE};

pub const BOARD_ROWS: usize = 21;
pub const BOARD_COLS: usize = 10;

const TETROMINO_INITIAL_POS: (i32, i32) = (0, (BOARD_COLS-SHAPE_SIZE) as i32 / 2);

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
    next_tetromino: Tetromino,
    tetromino_shape_generator: TetrominoShapeGenerator,

    level: i32,
    total_lines_cleared: i32,
    score: i32,
    wait_ticks_remaining: i32,
    game_over: bool,

    emitter: Emitter
}

impl Game {
    pub fn new(emitter: Emitter) -> Self {
        let board = [['_'; BOARD_COLS]; BOARD_ROWS];
        let mut tetromino_shape_generator = TetrominoShapeGenerator::new();
        let mut game =  Game {
            board,
            current_tetromino: Tetromino::new(
                TETROMINO_INITIAL_POS,
                tetromino_shape_generator.make_random()
            ),
            next_tetromino: Tetromino::new(
                (0, 0),
                tetromino_shape_generator.make_random()
            ),
            tetromino_shape_generator,
            level: 0,
            total_lines_cleared: 0,
            score: 0,
            wait_ticks_remaining: 0, // Dummy value
            game_over: false,
            emitter,
        };
        game.reset_wait_ticks();
        game
    }

    pub fn get_game_over(&self) -> bool {
        self.game_over
    }

    fn reset_wait_ticks(&mut self) {
        self.wait_ticks_remaining = match self.level {
            0..=8 => 48 - 5*self.level,
            9 => 6,
            10..=12 => 5,
            13..=15 => 4,
            16..=18 => 3,
            19..=28 => 2,
            _ => 1
        }
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
                self.emitter.emit_tetromino("current_tetromino", &self.current_tetromino);
                true
            },
            Err(_) => {
                // If arrow down failed, the tetromino can not move futher down, and the next
                // tetromino has to be set
                if key == "ArrowDown" {
                    self.add_current_tetromino_to_board();
                    let _ = self.set_new_tetromino();
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
        let _ = self.set_new_tetromino();
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
                self.emitter.emit_tetromino("current_tetromino", &self.current_tetromino);
                success = true;
            }

            Err(MoveNotAllowedError::TooFarLeft(x)) | Err(MoveNotAllowedError::TooFarRight(x)) => {
                let step = (0, -x);

                match self.check_move(&tetromino, &step) {
                    Ok(()) => {
                        tetromino.move_pos(step);
                        self.current_tetromino = tetromino;
                        self.emitter.emit_tetromino("current_tetromino", &self.current_tetromino);
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
        for occupied_pos in tetromino.get_occupied_positions() {
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
        for occupied_pos in tetromino.get_occupied_positions() {
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
        let shape_name = self.current_tetromino.get_shape_name();
        for occupied_pos in self.current_tetromino.get_occupied_positions() {
            let i = occupied_pos.0 as usize;
            let j = occupied_pos.1 as usize;
            self.board[i][j] = shape_name;
        }
        self.clear_full_rows();
        self.emitter.emit_board("board", &self.board);
    }

    /// Update the score based on the number of cleared rows.
    fn update_score(&mut self, n_lines_cleared: i32) {
        let base_points = match n_lines_cleared {
            0 => 0,
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0 // This should not occur
        };
        let points = base_points * (self.level+1);
        self.score += points;
        println!("Updated score to {}", self.score);
        self.emitter.emit_number("score", self.score);
        self.emitter.emit_number("score_increase", points);
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
        }
    }
    /// Forwards the game a single tick. Returns true if the tick succeeded. Returns false if
    /// the tick fails because the player is game-over.
    pub fn tick(&mut self) -> bool {
        if self.game_over {
            // Do not forward the game a tick if the game is over
            return false;
        }

        if self.wait_ticks_remaining > 0 {
            // Only decrease counter if this tick does not forward the game
            self.wait_ticks_remaining -= 1;
            return true;
        }

        self.reset_wait_ticks();

        let step = (1, 0);
        let result = self.check_move(&self.current_tetromino, &step);
        match result {
            Ok(_) => {
                self.current_tetromino.move_pos(step);
                self.emitter.emit_tetromino("current_tetromino", &self.current_tetromino);
                true
            },
            Err(err) => {
                println!("Can not move tetromino down: {:?}", err);
                self.add_current_tetromino_to_board();
                match self.set_new_tetromino() {
                    Ok(_) => { true }
                    Err(_) => { false }
                }
            }
        }
    }

    fn set_new_tetromino(&mut self) -> Result<(), ()>{
        self.current_tetromino = self.next_tetromino.clone();
        self.current_tetromino.move_pos(TETROMINO_INITIAL_POS);

        // Game over if newly placed block overlaps with board
        match self.check_move(&self.current_tetromino, &(0, 0)) {
            Ok(_) => {
                self.next_tetromino = Tetromino::new(
                    (0, 0),
                    self.tetromino_shape_generator.make_random()
                );

                self.emitter.emit_tetromino("current_tetromino", &self.current_tetromino);
                self.emitter.emit_tetromino("next_tetromino", &self.next_tetromino);

                Ok(())
            },
            Err(MoveNotAllowedError::OverlapsWithOccupied) => {
                self.game_over = true;
                self.emitter.emit_string("game_over", "GAME OVER".to_string());
                Err(())
            }
            // Other err is not expected to occur.
            Err(_) => panic!("Unexpected error encountered when creating next tetromino.")
        }
    }

    pub fn reset(&mut self) {
        self.board = [['_'; BOARD_COLS]; BOARD_ROWS];
        self.current_tetromino = Tetromino::new(
            TETROMINO_INITIAL_POS, self.tetromino_shape_generator.make_random()
        );
        self.next_tetromino = Tetromino::new(
            (0, 0), self.tetromino_shape_generator.make_random()
        );
        self.level = 0;
        self.score = 0;
        self.total_lines_cleared = 0;
        self.game_over = false;
        self.reset_wait_ticks();
        self.emit_all();
    }

    pub fn emit_all(&mut self) {
        self.emitter.emit_tetromino("current_tetromino", &self.current_tetromino);
        self.emitter.emit_tetromino("next_tetromino", &self.next_tetromino);
        self.emitter.emit_number("score", self.score);
        self.emitter.emit_number("level", self.level);
        self.emitter.emit_board("board", &self.board);
    }
}

// Declare a shared game struct to use the state of the game
// Arc Mutex makes it usable in different threads
#[derive(Clone)]
pub struct GameRunner {
    pub game: Arc<Mutex<Game>>,
    running: Arc<atomic::AtomicBool>,
    tick_rate: f64,
}

impl GameRunner {
    pub fn new(game: Game) -> Self {
        GameRunner{
            game: Arc::new(Mutex::new(game)),
            running: Arc::new(atomic::AtomicBool::new(false)),
            tick_rate: 60.0,
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

        // Early return if the game-state is game-over. Game should be reset before running
        // Otherwise emit the current game state to sync the interface
        {
            let mut game = self.game.lock().unwrap();
            match game.game_over {
                true => { return; }
                false => { game.emit_all(); }
            }
        }

        // Set running flag to true
        self.running.store(true, atomic::Ordering::SeqCst);

        // Clone self to move it to the background thread
        let self_clone = self.clone();

        // Spawn a thread to increment the game at set intervals
        thread::spawn(move || {
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
                }
                thread::sleep(Duration::from_secs_f64(1. / self_clone.tick_rate));
            }
        });
    }

    pub fn reset(&self) {
        self.running.store(false, atomic::Ordering::SeqCst);
        let mut game = self.game.lock().unwrap();
        game.reset();
    }
}
