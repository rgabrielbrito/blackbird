pub struct GeneticAlgorithm;

pub trait Individual {
    fn fitness(&self) -> f32;
}

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len()).map(|_| todo!()).collect()
    }
}
