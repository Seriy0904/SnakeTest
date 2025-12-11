use rand::Rng;

use crate::{MUTATION_RATE, perceptron::neuron::Neuron};

pub fn mutate(mut neuron: Neuron) -> Neuron {
    let mut rng = rand::rng();
    let chance: f32 = rng.random();
    if chance <= MUTATION_RATE {
        for weight_id in 0..neuron.weights.len() {
            neuron.weights[weight_id] = rng.random_range(-1.0..=1.0);
        }
    }
    if chance <= MUTATION_RATE {
        neuron.bias = rng.random_range(-1.0..=1.0);
    }
    return neuron;
}
