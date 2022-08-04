use crate::*;

#[derive(Clone, Debug, Default)]
pub struct KPointCrossover;

impl CrossoverMethod for KPointCrossover {
    fn crossover(
        &self,
        _rng: &mut dyn RngCore,
        _parent_a: &Chromosome,
        _parent_b: &Chromosome,
    ) -> Chromosome {
        todo!()
    }
}
