use macroquad::math::Vec2;

use crate::game::{Direction, Point};

#[derive(PartialEq, Clone)]
pub struct Snake {
    pub points: Vec<Point>,
    pub dir: Point,
}
impl Snake {
    pub fn new(points: Vec<Point>, dir: Point) -> Self {
        return Self { points, dir };
    }
    pub fn step(&mut self, new_dir: Direction) -> Point {
        if new_dir.opposite().to_point() != self.dir {
            self.dir = new_dir.to_point();
        }
        let new_head_point = self.points[0] + self.dir;
        let prev_tail_point = self.points[self.points.len() - 1];
        for i in (1..(self.points.len())).rev() {
            self.points[i] = self.points[i - 1];
        }
        self.points[0] = new_head_point;
        return prev_tail_point;
    }
}
