use crate::*;

#[derive(Clone, Debug, Default)]
pub struct KPointCrossover;

impl KPointCrossover {
    fn generate_k_points(&self, rng: &mut dyn RngCore, k: u32, chromosome_len: u32) -> Vec<u32> {
        let mut k_points: Vec<u32> = (0..k).map(|_| rng.gen_range(0..=chromosome_len)).collect();
        k_points.sort();
        k_points
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
        assert_eq!(k_points.as_slice(), [1, 2, 2, 8, 9].as_ref());
    }
}
