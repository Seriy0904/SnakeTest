use std::f32::consts::SQRT_2;

use rand::Rng;

use crate::{
    game::{Direction, Point, apple::Apple, snake::Snake},
    perceptron::network::Network,
};
#[derive(PartialEq, Clone)]
pub struct World {
    pub alive: bool,
    pub last_apple_time: usize,
    pub life_time: usize,
    pub score: usize,
    pub snake: Snake,
    pub apple: Apple,
    pub width: i32,
    pub height: i32,
    pub network: Network,
    pub fitness_score: usize,
}

impl World {
    pub fn new(width: i32, height: i32, network: Network) -> Self {
        let apple = Apple {
            position: Point { x: 3, y: 3 },
        };
        let snake = Snake::new(
            vec![Point {
                x: width / 2,
                y: height / 2,
            }],
            Point { x: 0, y: 0 },
        );
        let mut world = Self {
            alive: true,
            last_apple_time: 0,
            life_time: 0,
            score: 0,
            snake,
            apple,
            width,
            height,
            network,
            fitness_score: 0,
        };
        world.apple_random_pos();
        return world;
    }
    pub async fn tick(mut self) -> Self {
        if self.alive {
            self.life_time += 1;
            self.last_apple_time += 1;
            let network_answer = self.network.propagate(self.get_inputs());
            let new_dir = Direction::from_network(network_answer);

            let ghost_tail = self.snake.step(new_dir);
            if self.snake.points[0] == self.apple.position {
                self.snake.points.push(ghost_tail);
                self.score += 1;
                self.last_apple_time = 0;
                self.apple_random_pos();
            }
            if self.check_snake_collision() {
                self.alive = false;
            }
        }
        return self;
    }
    fn check_snake_collision(&self) -> bool {
        let mut collision = false;
        let head = self.snake.points[0];
        // Check snake head collision with its body
        for i in 1..self.snake.points.len() {
            if head == self.snake.points[i] {
                collision = true;
            }
        }
        // Check snake head collision with walls
        if head.x == 0 || head.y == 0 || head.x == self.width - 1 || head.y == self.height - 1 {
            collision = true;
        }
        return collision;
    }
    /*
    * Inputs are 29 neurons, each of which is responsible for:
    * 1,2,3,4 are for wall distance: right, up, left, down respectively
    * 5,6,7,8 are for diagonal wall distance: right-up, left-up, left-down, right-down respectively

    * 9,10,11,12 are for closest body part distance: right, up, left, down respectively
    * 13,14,15,16 are for diagonal closest body part distance: right-up, left-up, left-down, right-down respectively

    * 17,18,19,20 are for apple distance: right, up, left, down respectively
    * 21,22,23,24 are for diagonal apple distance: right-up, left-up, left-down, right-down respectively

    * 25,26,27,28 are the section in which apple is located: right, up, left, down
    * 29 is the sensor getting the distance to the apple
    * 30,31 are the dir of snake, x, y
    */
    pub fn get_inputs(&self) -> Vec<f32> {
        let mut inputs = vec![];

        let wall_dist = self.get_wall_dist();
        // println!("Wall distances: {:?}", wall_dist);
        inputs.extend(wall_dist);

        let body_dist = self.get_body_dist();
        // println!("Body distances: {:?}", body_dist);
        inputs.extend(body_dist);

        let apple_dist = self.get_apple_dist();
        // println!("Apple distances: {:?}", apple_dist);
        inputs.extend(apple_dist);

        inputs.extend([self.snake.dir.x as f32, self.snake.dir.y as f32]);
        return inputs;
    }
    fn get_wall_dist(&self) -> Vec<f32> {
        let mut distances = vec![];
        let right = (self.width - self.snake.points[0].x - 1) as f32;
        let up = (self.snake.points[0].y) as f32;
        let left = (self.snake.points[0].x) as f32;
        let down = (self.height - self.snake.points[0].y - 1) as f32;
        distances.push(1.0 / right);
        distances.push(1.0 / up);
        distances.push(1.0 / left);
        distances.push(1.0 / down);
        let right_up = right.min(up);
        let left_up = left.min(up);
        let left_down = left.min(down);
        let right_down = right.min(down);
        distances.push(1.0 / right_up);
        distances.push(1.0 / left_up);
        distances.push(1.0 / left_down);
        distances.push(1.0 / right_down);
        return distances;
    }
    fn get_body_dist(&self) -> Vec<f32> {
        let mut distances = vec![];
        let mut right: f32 = 0.0;
        let mut up: f32 = 0.0;
        let mut left: f32 = 0.0;
        let mut down: f32 = 0.0;

        let mut right_up: f32 = 0.0;
        let mut left_up: f32 = 0.0;
        let mut left_down: f32 = 0.0;
        let mut right_down: f32 = 0.0;

        let snake_points = &self.snake.points;
        for snake_point in 1..snake_points.len() {
            if snake_points[snake_point].y == snake_points[0].y {
                if snake_points[snake_point].x > snake_points[0].x {
                    right = right.min((snake_points[snake_point].x - snake_points[0].x) as f32);
                    if right == 0.0 {
                        right = (snake_points[snake_point].x - snake_points[0].x) as f32
                    }
                } else {
                    left = left.min((snake_points[0].x - snake_points[snake_point].x) as f32);
                    if left == 0.0 {
                        left = (snake_points[0].x - snake_points[snake_point].x) as f32;
                    }
                }
            }
            if snake_points[snake_point].x == snake_points[0].x {
                if snake_points[snake_point].y > snake_points[0].y {
                    down = down.min((snake_points[snake_point].y - snake_points[0].y) as f32);
                    if down == 0.0 {
                        down = (snake_points[snake_point].y - snake_points[0].y) as f32;
                    }
                } else {
                    up = up.min((snake_points[0].y - snake_points[snake_point].y) as f32);
                    if up == 0.0 {
                        up = (snake_points[0].y - snake_points[snake_point].y) as f32;
                    }
                }
            }
            let body_vec = snake_points[0] - snake_points[snake_point];
            if body_vec.x.abs() == body_vec.y.abs() {
                if body_vec.x > 0 && body_vec.y < 0 {
                    right_up = right_up.min((body_vec.x) as f32);
                    if right_up == 0.0 {
                        right_up = body_vec.x as f32;
                    }
                }
                if body_vec.x < 0 && body_vec.y < 0 {
                    left_up = left_up.min(-(body_vec.x) as f32);
                    if right_up == 0.0 {
                        right_up = (-body_vec.x) as f32;
                    }
                }
                if body_vec.x < 0 && body_vec.y > 0 {
                    left_down = left_down.min((body_vec.y) as f32);
                    if right_up == 0.0 {
                        right_up = body_vec.y as f32;
                    }
                }
                if body_vec.x > 0 && body_vec.y > 0 {
                    right_down = right_down.min((body_vec.y) as f32);
                    if right_up == 0.0 {
                        right_up = body_vec.y as f32;
                    }
                }
            }
        }
        distances.push(1.0 / right);
        distances.push(1.0 / up);
        distances.push(1.0 / left);
        distances.push(1.0 / down);
        distances.push(1.0 / right_up);
        distances.push(1.0 / left_up);
        distances.push(1.0 / left_down);
        distances.push(1.0 / right_down);
        return distances;
    }
    fn get_apple_dist(&self) -> Vec<f32> {
        let mut distances = vec![];
        let head = self.snake.points[0];
        let mut right: f32 = 0.0;
        let mut up: f32 = 0.0;
        let mut left: f32 = 0.0;
        let mut down: f32 = 0.0;

        if self.apple.position.y == head.y {
            if self.apple.position.x > head.x {
                right = (self.apple.position.x - head.x) as f32;
            }
            if self.apple.position.x < head.x {
                left = (head.x - self.apple.position.x) as f32;
            }
        }
        if self.apple.position.x == head.x {
            if self.apple.position.y > head.y {
                down = (self.apple.position.y - head.y) as f32;
            }
            if self.apple.position.y < head.y {
                up = (head.y - self.apple.position.y) as f32;
            }
        }

        let apple_vec = head - self.apple.position;
        distances.push(1.0 / right);
        distances.push(1.0 / up);
        distances.push(1.0 / left);
        distances.push(1.0 / down);

        distances.push(if apple_vec.x > 0 { 1.0 } else { 0.0 });
        distances.push(if apple_vec.y < 0 { 1.0 } else { 0.0 });
        distances.push(if apple_vec.x < 0 { 1.0 } else { 0.0 });
        distances.push(if apple_vec.y > 0 { 1.0 } else { 0.0 });
        let apple_dist = (apple_vec.x + apple_vec.y) as f32;
        distances.push(apple_dist);

        return distances;
    }
    fn apple_random_pos(&mut self) {
        let mut rng = rand::rng();
        let new_x = rng.random_range(1..(self.width - 1));
        let new_y = rng.random_range(1..(self.height - 1));
        self.apple.position = Point { x: new_x, y: new_y };
    }
    pub fn calculate_fitness_score(&mut self) {
        let mut fitness_score = 0;
        fitness_score += self.score * 100;
        fitness_score += self.life_time / 4;
        self.fitness_score = fitness_score;
    }
}
