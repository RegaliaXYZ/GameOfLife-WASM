use std::{collections::HashSet};
use rand::Rng;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    pub alive_fields: HashSet<Position>
}

impl GameOfLife {
    pub fn new(width: usize, height: usize, percentage: usize) -> GameOfLife {
        GameOfLife {
            width: width,
            height: height,
            alive_fields: {
                let mut alive_fields: HashSet<Position> = HashSet::new();
                let mut rng = rand::thread_rng();
                for y in 0..height {
                    for x in 0..width {
                        if rng.gen_range(0..=100) < percentage {
                            alive_fields.insert((x, y));
                        }
                    }
                }
                alive_fields
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameOfLife;

    #[test]
    fn test_display() {
        let game_of_life = GameOfLife::new(10, 10, 10);
        println!("{:?}", game_of_life);
    }
}