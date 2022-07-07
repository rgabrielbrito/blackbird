pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    _neurons: Vec<Neuron>,
}

struct Neuron {
    _bias: f32,
    _weights: Vec<f32>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    fn propagate(&self, _inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}
