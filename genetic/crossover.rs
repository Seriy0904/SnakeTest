use std::vec;

use rand::Rng;

use crate::{
    ELITISTS_NUM, POPULATION_SIZE,
    game::world::World,
    genetic::{mutation::mutate, selection::tournament_selection},
    perceptron::{layer::Layer, network::Network, neuron::Neuron},
};
pub fn recombine_worlds(worlds: &Vec<World>) -> Vec<Network> {
    let mut recombinations = vec![];
    for _recombination_id in 0..((POPULATION_SIZE - ELITISTS_NUM) / 2) {
        let (parent1, parent2) = tournament_selection(worlds);
        let (child1, child2) = crossover_networks(&parent1.network, &parent2.network);
        recombinations.push(child1);
        recombinations.push(child2);
    }
    return recombinations;
}
pub fn crossover_networks(network1: &Network, network2: &Network) -> (Network, Network) {
    assert_eq!(
        network1.layers.len(),
        network2.layers.len(),
        "You have given networks with different layers count"
    );
    let mut new_network1 = Network::new_empty();
    let mut new_network2 = Network::new_empty();
    for layer_id in 0..network1.layers.len() {
        let new_layer1 = crossover_layers(&network1.layers[layer_id], &network1.layers[layer_id]);
        let new_layer2 = crossover_layers(&network1.layers[layer_id], &network1.layers[layer_id]);
        new_network1.layers.push(new_layer1);
        new_network2.layers.push(new_layer2);
    }
    return (new_network1, new_network2);
}
fn crossover_layers(layer1: &Layer, layer2: &Layer) -> Layer {
    assert_eq!(
        layer1.neurons.len(),
        layer2.neurons.len(),
        "You have given layers with different neurons count"
    );
    let mut new_layer = Layer::new_empty();
    let mut rng = rand::rng();
    let chance = rng.random_range(0..layer1.neurons.len());
    for neuron_id in 0..layer1.neurons.len() {
        // let new_neuron = if neuron_id <= chance {
        //     layer1.neurons[neuron_id].clone()
        // } else {
        //     layer2.neurons[neuron_id].clone()
        // };
        let new_neuron = crossover_neurons(&layer1.neurons[neuron_id], &layer2.neurons[neuron_id]);
        new_layer.neurons.push(new_neuron);
    }
    return new_layer;
}
fn crossover_neurons(neuron1: &Neuron, neuron2: &Neuron) -> Neuron {
    let mut rng = rand::rng();
    let chance = rng.random::<f32>();
    let mut new_neuron;
    if chance <= 0.5 {
        new_neuron = (*neuron1).clone();
    } else {
        new_neuron = (*neuron2).clone();
    }
    new_neuron = mutate(new_neuron);
    return new_neuron;
}
