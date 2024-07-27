use rand::seq::SliceRandom;
use rand::thread_rng;

pub const SHAPE_SIZE: usize = 4; // A tetromino fills a SHAPE_SIZE by SHAPE_SIZE grid

#[derive(Clone)]
pub struct TetrominoShape {
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

pub struct TetrominoShapeGenerator {
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
    pub fn new(pos: (i32, i32), shape: TetrominoShape) -> Self {
        let mut tetromino = Tetromino {
            pos,
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
