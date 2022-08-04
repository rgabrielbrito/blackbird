use crate::*;

#[derive(Clone, Debug, Default)]
pub struct KPointCrossover;

// Rather than iterating through every point and choosing
// either parent_a's value or parent_b's value with equal probability,
// K point crossover picks K crossover points at random and swaps them.
// For the purposes of this application, the K will most likely be fixed
// rather than passing it as parameter to maintain the original api
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
