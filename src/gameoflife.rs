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
    fn test_neighbor_count_full_field() {
        let game_of_life = GameOfLife::new(10, 10, 100);
        let corners_pos = [
            (0, 0),
            (0, 9),
            (9, 0),
            (9, 9)
        ];
        
        for &pos in &corners_pos {
            assert_eq!(game_of_life.get_alive_neighbor_count(pos), 3);
        }
        assert_eq!(game_of_life.get_alive_neighbor_count((5, 5)), 8);
    }
}