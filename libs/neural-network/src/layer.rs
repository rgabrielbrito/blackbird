use crate::*;

#[derive(Clone, Debug)]
pub struct Layer {
    pub(crate) biases: na::DVector<f32>,
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

        let biases = na::DVector::from_vec(biases);
        let weights = na::DMatrix::from_vec(input_neurons, output_neurons, weights);

        Self { biases, weights }
    }

    pub fn propagate(&self, inputs: na::DMatrix<f32>) -> na::DMatrix<f32> {
        todo!()
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
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
            todo!()
        }
    }
}
