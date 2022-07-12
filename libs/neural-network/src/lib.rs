use rand::Rng;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    pub fn random(rng: &mut dyn rand::RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        use std::iter::once;

        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .cloned()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("Too many weights");
        }

        Self { layers }
    }
}

impl Layer {
    pub fn random(
        rng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize,
    ) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng, input_neurons))
            .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}

impl Neuron {
    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    pub fn from_weights(output_neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("Not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("Not enough weights"))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn neuron_random() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            approx::assert_relative_eq!(neuron.bias, -0.6255188);

            approx::assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383957, 0.8181262, 0.26284897, 0.5238807,].as_ref(),
            );
        }

        #[test]
        fn layer_random() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 2, 1);

            assert_eq!(layer.neurons.len(), 1);

            let neuron = &layer.neurons[0];

            approx::assert_relative_eq!(neuron.bias, -0.6255188);
            approx::assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383957, 0.8181262].as_ref(),
            );
        }

        #[test]
        fn network_random() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let topology_one = LayerTopology { neurons: 2 };
            let topology_two = LayerTopology { neurons: 1 };

            let mut layers = Vec::with_capacity(2);

            layers.push(topology_one);
            layers.push(topology_two);

            let network = Network::random(&mut rng, &layers);

            assert_eq!(network.layers.len(), 1);
            assert_eq!(network.layers[0].neurons.len(), 1);

            let neuron = &network.layers[0].neurons[0];

            approx::assert_relative_eq!(neuron.bias, -0.6255188);
            approx::assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383957, 0.8181262].as_ref(),
            );
        }
    }

    mod propagate {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn neuron_propagate() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            approx::assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );
        }

        #[test]
        fn layer_propagate() {
            let input = vec![0.5, 1.0];
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let layer = Layer::random(&mut rng, 2, 1);
            let output = layer.propagate(input);

            approx::assert_relative_eq!(output.as_slice(), [0.5295272].as_ref());
        }

        #[test]
        fn network_propagate() {
            let input = vec![0.5, 1.0];
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let top_one = LayerTopology { neurons: 2 };
            let top_two = LayerTopology { neurons: 1 };

            let mut layers = Vec::with_capacity(2);

            layers.push(top_one);
            layers.push(top_two);

            let network = Network::random(&mut rng, &layers);
            let output = network.propagate(input);

            approx::assert_relative_eq!(output.as_slice(), [0.5295272].as_ref());
        }
    }
}
