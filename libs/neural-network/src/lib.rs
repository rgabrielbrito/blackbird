pub use self::layer_topology::*;

use self::layer::*;
use rand::Rng;
use std::iter::once;

mod layer;
mod layer_topology;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
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

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

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
