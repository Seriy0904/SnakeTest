use rand::Rng;

use crate::perceptron::{layer::Layer, neuron::Neuron};

#[derive(PartialEq, Clone)]
pub struct Network {
    pub layers: Vec<Layer>,
}
impl Network {
    // Giving the values of input neurons and getting out the output neurons values
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let output = self
            .layers
            .iter()
            .fold(inputs, |input, layer| layer.propagate(input));
        return output;
    }
    pub fn new_empty() -> Self {
        Self { layers: vec![] }
    }
    pub fn create_random_network(layer_sizes: Vec<usize>) -> Network {
        let mut network = Self::new_empty();
        let mut rng = rand::rng();
        // Starting from 1 because 0th layer is input layer
        for layer_id in 1..layer_sizes.len() {
            let mut layer = Layer::new_empty();
            // Creating neuron for each layer
            for _neuron_id in 0..layer_sizes[layer_id] {
                let mut weights = vec![];
                // Creating weights for every edge(number of neurons in previous layer) of neuron
                for _weight_id in 0..layer_sizes[layer_id - 1] {
                    let weight: f32 = 1.0 - (rng.random::<f32>() * 2.0);
                    weights.push(weight);
                }
                let bias: f32 = 1.0 - (rng.random::<f32>() * 2.0);
                let neuron = Neuron { weights, bias };
                // Adding randomly created neuron to the layer
                layer.neurons.push(neuron);
            }
            // Adding randomly created layer to the network
            network.layers.push(layer);
        }
        return network;
    }
}
