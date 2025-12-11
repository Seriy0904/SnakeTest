use std::f32::consts::E;

#[derive(Clone, PartialEq)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
}
impl Neuron {
    pub fn propagate(&self, input: &Vec<f32>) -> f32 {
        let mut output = 0.0;
        for i in 0..input.len() {
            output += input[i] * self.weights[i]
        }
        output = Self::activasion(output + self.bias);
        return output;
    }
    fn activasion(value: f32) -> f32 {
        // return value.tanh();
        // return 1.0 / (1.0 + E.powf(-value));
        return value.max(0.0);
    }
}
