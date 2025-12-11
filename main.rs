mod game;
mod genetic;
mod perceptron;

use std::thread::{self};

use futures::future::join_all;
use macroquad::Window;
use macroquad::window::{Conf, clear_background, next_frame};
use macroquad::{
    color::*,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{
    game::world::World,
    genetic::{crossover::recombine_worlds, selection::get_top_n},
    perceptron::network::Network,
};

pub const POPULATION_SIZE: usize = 1000;
pub const ELITISTS_NUM: usize = 40;
pub const MUTATION_RATE: f32 = 0.02;
pub const TOURNAMENT_SIZE: usize = 100;

const SCREEN_SIZE: f32 = 800.0;
const CELL_SIZE: f32 = 20.0;
const FIELD_SIZE: i32 = (SCREEN_SIZE / CELL_SIZE) as i32;

#[tokio::main]
async fn main() {
    let mut worlds = vec![];
    (0..POPULATION_SIZE).for_each(|_| {
        worlds.push(World::new(
            FIELD_SIZE,
            FIELD_SIZE,
            Network::create_random_network(vec![31, 24, 12, 4]),
        ))
    });
    let mut gen_count = 1;
    let mut top_apples = 0;
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    open_window(rx);
    loop {
        if true {
            let mut all_dead = true;
            // all_dead = false
            let mut futures = vec![];
            for w in worlds {
                futures.push(tokio::task::spawn(w.tick()));
            }
            let results = join_all(futures).await;
            worlds = vec![];
            for result in results {
                worlds.push(result.unwrap());
            }

            // join_all(futures).await;
            for world in &mut worlds {
                // world.tick();
                if world.last_apple_time >= 2 * (world.width + world.height) as usize {
                    world.alive = false;
                }
                if world.alive {
                    all_dead = false;
                } else if world.fitness_score == 0 {
                    world.calculate_fitness_score();
                }
            }
            let _ = tx.send(GameState {
                world: worlds[worlds.len() - 1].clone(),
                top_apples: top_apples,
                generation: gen_count,
            });
            if all_dead {
                top_apples = worlds.iter().max_by_key(|world| world.score).unwrap().score;
                let mut networks = recombine_worlds(&worlds);
                networks.extend(get_top_n(worlds, ELITISTS_NUM));
                let mut recombinations: Vec<World> = vec![];
                for network in networks {
                    recombinations.push(World::new(FIELD_SIZE, FIELD_SIZE, network));
                }
                worlds = recombinations;
                gen_count += 1;
            }
        }
    }
}
pub fn open_window(rx: UnboundedReceiver<GameState>) {
    thread::spawn(|| {
        Window::from_config(window_conf(), draw(rx));
    });
}

fn window_conf() -> Conf {
    Conf {
        window_width: SCREEN_SIZE as i32,
        window_height: SCREEN_SIZE as i32,
        window_resizable: false,
        window_title: String::from("Python perceptron"),
        ..Default::default()
    }
}
pub async fn draw(mut rx: UnboundedReceiver<GameState>) {
    let mut gamestate = rx.blocking_recv().unwrap();
    loop {
        clear_background(GRAY);
        // for world in &worlds {
        draw_world(&gamestate.world);
        // }
        draw_text(
            &format!("Generation {}", gamestate.generation),
            0.0,
            20.0,
            40.0,
            BLUE,
        );
        draw_text(
            &format!("Best apples: {}", gamestate.top_apples),
            0.0,
            50.0,
            40.0,
            BLUE,
        );
        if let Ok(new_gamestate) = rx.try_recv() {
            gamestate = new_gamestate
        };
        next_frame().await
    }
}
fn draw_world(world: &World) {
    draw_rectangle_lines(0.0, 0.0, SCREEN_SIZE, SCREEN_SIZE, CELL_SIZE * 2.0, RED);
    for snake_point in &world.snake.points {
        draw_rectangle(
            (snake_point.x as f32) * CELL_SIZE,
            (snake_point.y as f32) * CELL_SIZE,
            CELL_SIZE - 1.0,
            CELL_SIZE - 1.0,
            YELLOW,
        );
    }
    draw_rectangle(
        (world.apple.position.x as f32) * CELL_SIZE,
        (world.apple.position.y as f32) * CELL_SIZE,
        CELL_SIZE - 1.0,
        CELL_SIZE - 1.0,
        GREEN,
    );
}
pub struct GameState {
    world: World,
    top_apples: usize,
    generation: usize,
}
