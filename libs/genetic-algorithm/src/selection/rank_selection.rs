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

        let mut ranks: Vec<usize> = Vec::with_capacity(population.len());

        for i in 0..population.len() {
            ranks.push(i);
        }

        let total_fitness: f32 = population
            .iter()
            .map(|individual| individual.fitness())
            .sum();

        loop {
            let individual_rank = ranks.choose(rng).expect("Population is empty");
            let individual_share = *individual_rank as f32 / total_fitness;

            if rng.gen_bool(individual_share as f64) {
                return population[*individual_rank];
            }
        }
    }
}
