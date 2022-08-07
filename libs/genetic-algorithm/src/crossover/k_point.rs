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

    // For this function, you need to keep in mind that that k_points is not an exhaustive range
    // So, not every element is intended to be swapped. For the ones not included in the range,
    // you have to transfer the values as is. This is why the output of the test does not include
    // every value that appears in the chromosome.
    // Moreover, you should fix K, rather than using the random number generator. This will ensure
    // that each subset of parent_a and parent_b will be the same length and, thus, swappable
    fn construct_chromosome_subsets<'a>(
        &'a self,
        rng: &mut dyn RngCore,
        chromosome: &'a Chromosome,
    ) -> Vec<&[f32]> {
        let len_of_chromosome = chromosome.len();
        let k = rng.gen_range(0..=len_of_chromosome);
        let k_points = self.generate_k_points(rng, k, len_of_chromosome);
        let mut subsets: Vec<&[f32]> = vec![];

        for (prev, next) in k_points.iter().tuple_windows() {
            subsets.push(&chromosome[*prev..*next]);
        }

        subsets
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

    mod subsets {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn create_chromosome_subsets() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let chromosome = Chromosome::new(vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ]);
            let crossover = KPointCrossover::default();
            let subsets = crossover.construct_chromosome_subsets(&mut rng, &chromosome);
            assert_eq!(subsets.as_slice(), [[]].as_ref());
        }
    }
}
