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

    fn construct_chromosome_subsets(
        &self,
        rng: &mut dyn RngCore,
        chromosome: &Chromosome,
    ) -> Vec<Vec<f32>> {
        let len_of_chromosome = chromosome.len();
        let k = rng.gen_range(0..=len_of_chromosome);
        let k_points = self.generate_k_points(rng, k, len_of_chromosome);
        todo!()
    }
}

// Rather than iterating through every point and choosing
// either parent_a's value or parent_b's value with equal probability,
// K point crossover picks K crossover points at random and swaps them.
// For the purposes of this application, the K will most likely be fixed
// rather than passing it as parameter to maintain the original api
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
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn gen_k_points() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let crossover = KPointCrossover::default();

        let k_points = crossover.generate_k_points(&mut rng, 5, 10);
        assert_eq!(k_points.as_slice(), [1, 2, 8, 9].as_ref());
    }
}
