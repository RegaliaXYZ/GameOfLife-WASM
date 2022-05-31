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
                        if rng.gen_range(0..=100) <= percentage {
                            alive_fields.insert((x, y));
                        }
                    }
                }
                alive_fields
            }
        }
    }

    pub fn get_neighbor_iterator(&self, (x, y): Position) -> impl Iterator<Item =Position> {
        let width: usize = self.width;
        let height: usize = self.height;
        (x.max(1) - 1..=(x + 1).min(width - 1)).flat_map(move |i| {
            (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j))
        }).filter(move |&pos| pos != (x, y))
    }

    pub fn get_alive_neighbor_count(&self, pos: Position) -> u8 {
        self
            .get_neighbor_iterator(pos)
            .filter(|pos| self.alive_fields.contains(pos))
            .count() as u8
    }

    pub fn tick(&mut self) {
        let mut new_alive_fields: HashSet<Position> = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let position = (x, y);
                let alive_neighbor_count = self.get_alive_neighbor_count(position);
                // RULES OF GAME OF LIFE 
                // 1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
                // 2. Any live cell with two or three live neighbours lives on to the next generation.
                // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
                // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
 
                if self.alive_fields.contains(&position) {
                    if alive_neighbor_count == 2 || alive_neighbor_count == 3 {
                        new_alive_fields.insert(position);
                    }
                } else {
                    if alive_neighbor_count == 3 {
                        new_alive_fields.insert(position);
                    }
                }
            }
        }
        self.alive_fields = new_alive_fields;
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
    
    #[test]
    fn debug() {
        let game_of_life = GameOfLife::new(10, 10, 100);
        println!("{:?}", game_of_life.alive_fields.len());
    }
    #[test]
    fn test_neighbor_count_full_field() {
        let game_of_life = GameOfLife::new(10, 10, 100);
        let corners_pos = [
            (0, 0),
            (0, 9),
            (9, 0),
            (9, 9)
        ];

        for y in 0..9 {
            for x in 0..9 {
                let pos = (x, y);
                if corners_pos.contains(&pos) {
                    assert_eq!(game_of_life.get_alive_neighbor_count(pos), 3);
                } else {

                    if x == 0 || y == 0 {
                        assert_eq!(game_of_life.get_alive_neighbor_count(pos), 5);
                    } else {
                        assert_eq!(game_of_life.get_alive_neighbor_count(pos), 8);
                    }
                }
            }
        }
    }
}