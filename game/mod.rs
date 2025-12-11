use std::panic;

use rand::Rng;

pub mod apple;
pub mod snake;
pub mod world;

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn abs(&self) -> Self {
        return Self {
            x: self.x.abs(),
            y: self.y.abs(),
        };
    }
}
impl std::ops::Add<Point> for Point {
    type Output = Self;
    fn add(self, dir: Self) -> Self::Output {
        return Self {
            x: self.x + dir.x,
            y: self.y + dir.y,
        };
    }
}
impl std::ops::Sub<Point> for Point {
    type Output = Self;

    fn sub(self, dir: Self) -> Self::Output {
        return Self {
            x: self.x - dir.x,
            y: self.y - dir.y,
        };
    }
}
#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn from_network(network_output: Vec<f32>) -> Direction {
        assert_eq!(
            network_output.len(),
            4,
            "Error, network output neurons count has to be 4"
        );

        let max_neuron = (0_usize..4_usize)
            .max_by(|i1, i2| network_output[*i1].total_cmp(&network_output[*i2]))
            .unwrap();
        return match max_neuron {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("The max neuron's index is somehow bigger than 4"),
        };
    }
    pub fn opposite(&self) -> Direction {
        return match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
    }
    pub fn to_point(&self) -> Point {
        return match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        };
    }
}
