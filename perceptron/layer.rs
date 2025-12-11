use crate::perceptron::neuron::Neuron;

#[derive(PartialEq, Clone)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}
impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs: Vec<f32> = vec![];
        for neuron in &self.neurons {
            outputs.push(neuron.propagate(&inputs));
        }
        return outputs;
    }
    pub fn new_empty() -> Self {
        Self { neurons: vec![] }
    }
}
