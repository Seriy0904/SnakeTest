use std::vec;

use rand::seq::IteratorRandom;

use crate::{TOURNAMENT_SIZE, game::world::World, perceptron::network::Network};

pub fn tournament_selection(networks: &Vec<World>) -> (&World, &World) {
    let mut rng = rand::rng();
    let sample = networks.iter().choose_multiple(&mut rng, TOURNAMENT_SIZE);
    let parent1 = *(sample.iter().max_by_key(|i| i.fitness_score).unwrap());

    let mut sample = networks.iter().choose_multiple(&mut rng, TOURNAMENT_SIZE);
    let mut parent2 = *(sample.iter().max_by_key(|i| i.fitness_score).unwrap());
    while parent2 == parent1 {
        sample = networks.iter().choose_multiple(&mut rng, TOURNAMENT_SIZE);
        parent2 = *(sample.iter().max_by_key(|i| i.fitness_score).unwrap());
    }
    return (parent1, parent2);
}
pub fn get_top_n(mut worlds: Vec<World>, n: usize) -> Vec<Network> {
    worlds.sort_by_key(|world| world.fitness_score as i32);
    let mut elitists = vec![];
    for world_id in 0..n {
        let top_n_world = worlds.pop().unwrap();
        // println!("Top: {}", top_n_world.fitness_score);
        elitists.push(top_n_world.network);
    }
    // println!();
    return elitists;
}
