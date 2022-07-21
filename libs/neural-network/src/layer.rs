use crate::*;

#[derive(Clone, Debug)]
pub struct Layer {
    pub(crate) biases: na::DMatrix<f32>,
    pub(crate) weights: na::DMatrix<f32>,
}

impl Layer {
    pub fn random(
        rng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize,
    ) -> Self {
        let mut gen_rand_vec = |amount| (0..amount).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        let biases = gen_rand_vec(output_neurons);
        let weights = gen_rand_vec(input_neurons * output_neurons);

        let biases = na::DMatrix::from_vec(1, output_neurons, biases);
        let weights = na::DMatrix::from_vec(input_neurons, output_neurons, weights);

        Self { biases, weights }
    }

    pub fn propagate(&self, inputs: na::DMatrix<f32>) -> na::DMatrix<f32> {
        assert_eq!(inputs.len(), self.weights.len());

        let mut output = inputs.transpose() * &self.weights + &self.biases;

        output.iter_mut().for_each(|cell| {
            *cell = cell.max(0.0);
        });

        output
    }

    pub fn from_weights(
        _input_size: usize,
        _output_size: usize,
        _weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        todo!()
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
        fn layer_random() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 2, 1);

            let biases = vec![-0.6255188];
            let weights = vec![0.67383957, 0.8181262];

            approx::assert_relative_eq!(layer.biases.as_slice(), biases.as_ref());
            approx::assert_relative_eq!(layer.weights.as_slice(), weights.as_ref());
        }
    }

    mod propagate {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn layer_propagate() {
            let input = vec![0.5, 1.0];
            let input = na::DMatrix::from_vec(2, 1, input);

            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 2, 1);

            let output = layer.propagate(input);

            approx::assert_relative_eq!(output.as_slice(), [0.5295272].as_ref());
        }
    }
}
