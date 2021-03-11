use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug)]
enum Tetrimino {
    T, J, L, I, O, Z, S
}

pub enum RotationDirection{
    Clockwise, Counterclockwise
}

pub struct tetrimino{
    pub shape: [[u8;4];4],
}

impl tetrimino {
    pub fn rotate(&mut self, dir: RotationDirection){
        match dir {
            Clockwise => {
                let old_shape = self.shape;
                for i in 0..4 {
                    self.shape[i] = [old_shape[0][i], old_shape[1][i], old_shape[2][i], old_shape[3][i] ];
                }
            }
            Counterclockwise  => {
                let old_shape = self.shape;
                for i in 0..4 {
                    self.shape[i] = [old_shape[0][3-i], old_shape[1][3-i], old_shape[2][3-i], old_shape[3][3-i] ];
                }
            }
        }   
    }
}
impl Distribution<Tetrimino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetrimino{
        match rng.gen_range(0..7) {
            0 => Tetrimino::T,
            1 => Tetrimino::J,
            2 => Tetrimino::L,
            3 => Tetrimino::I,
            4 => Tetrimino::O,
            5 => Tetrimino::Z,
            6 => Tetrimino::S,
            _ => Tetrimino::I
        }
    }
}

fn get_shape(kind: Tetrimino) -> tetrimino {
    match kind {
        T => tetrimino{shape: 
             [[0, 0, 0, 0],
              [0, 0, 0, 0],
              [0, 1, 0, 0],
              [1, 1, 1, 0]]},
        J => tetrimino{shape:
             [[0, 0, 0, 0],
              [0, 0, 0, 0],
              [1, 0, 0, 0],
              [1, 1, 1, 0]]},
        L => tetrimino{shape:[[0, 0, 0, 0],
               [0, 0, 0, 0],
               [0, 0, 1, 0],
               [1, 1, 1, 0]]},
        I => tetrimino{shape:
             [[1, 0, 0, 0],
              [1, 0, 0, 0],
              [1, 0, 0, 0],
              [1, 0, 0, 0]]}, 
        O => tetrimino{shape:
             [[0, 0, 0, 0],
              [0, 0, 0, 0],
              [1, 1, 0, 0],
              [1, 1, 0, 0]]}, 
        Z => tetrimino{shape:
             [[0, 0, 0, 0],
              [0, 0, 0, 0],
              [0, 1, 1, 0],
              [1, 1, 0, 0]]}, 
        S => tetrimino{shape:
             [[0, 0, 0, 0],
              [0, 0, 0, 0],
              [1, 1, 0, 0],
              [0, 1, 1, 0]]},       

    }
}

pub fn random_tetrimino() -> tetrimino {
    get_shape(rand::random())
}