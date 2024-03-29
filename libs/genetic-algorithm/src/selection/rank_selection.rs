use crate::*;

#[derive(Clone, Debug, Default)]
pub struct RankSelection;

impl SelectionMethod for RankSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        let mut population: Vec<&I> = vec![population].into_iter().flatten().collect();
        population.sort_by(|i, j| i.fitness().partial_cmp(&j.fitness()).unwrap());

        let ranks: Vec<usize> = (1..=population.len()).collect();
        let total_fitness: f32 = population
            .iter()
            .map(|individual| individual.fitness())
            .sum();

        loop {
            let individual_rank = ranks.choose(rng).expect("Population is empty");
            let individual_share = *individual_rank as f32 / total_fitness;

            if rng.gen_bool(individual_share as f64) {
                return population[*individual_rank - 1];
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[test]
    fn rank_selection() {
        let method = RankSelection::default();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(3.0),
            TestIndividual::new(4.0),
        ];

        let actual_histogram = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(BTreeMap::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as i32).or_default() += 1;
                histogram
            });

        let expected_histogram = maplit::btreemap! {
            1 => 110,
            2 => 206,
            3 => 287,
            4 => 397,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }
}
