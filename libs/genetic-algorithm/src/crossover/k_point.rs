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
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let k = parent_a.len() / 2;
        let k_points = self.generate_k_points(rng, k, parent_a.len());

        let mut parent_a: Vec<f32> = parent_a.clone().into_iter().collect();
        let mut parent_b: Vec<f32> = parent_b.clone().into_iter().collect();

        for (prev, next) in k_points.iter().tuple_windows() {
            for curr in *prev..*next {
                mem::swap(&mut parent_a[curr], &mut parent_b[curr]);
            }
        }

        Chromosome::new(parent_a)
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
