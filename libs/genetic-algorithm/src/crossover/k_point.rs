use crate::*;

#[derive(Clone, Debug, Default)]
pub struct KPointCrossover;

impl KPointCrossover {
    fn generate_k_points(
        &self,
        rng: &mut dyn RngCore,
        k: usize,
        chromosome_len: usize,
    ) -> Vec<usize> {
        Itertools::sorted((0..k).map(|_| rng.gen_range(0..=chromosome_len)))
            .unique()
            .collect()
    }
}

impl CrossoverMethod for KPointCrossover {
    fn crossover(
        &self,
        _rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod k_points {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn gen_k_points() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let crossover = KPointCrossover::default();

            let k_points = crossover.generate_k_points(&mut rng, 5, 10);
            assert_eq!(k_points.as_slice(), [0, 1, 2, 9].as_ref());
        }
    }
}
